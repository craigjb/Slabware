package slabware.hdmirx

import spinal.core._
import spinal.lib._

import spinal.lib.blackbox.xilinx.s7.BUFG
import spinal.lib.bus.regif.{AccessType, SymbolName}
import spinal.lib.com.i2c.I2c

import slabware.{BusDefinition, SvdPeripheral}

case class HdmiVideo(redBits: Int = 5, greenBits: Int = 6, blueBits: Int = 5)
    extends Bundle {
  val vSync = Bool()
  val hSync = Bool()
  val pixelsValid = Vec.fill(2)(Bool())
  val redPixels = Vec.fill(2)(Bits(redBits bits))
  val greenPixels = Vec.fill(2)(Bits(greenBits bits))
  val bluePixels = Vec.fill(2)(Bits(blueBits bits))
}

case class HdmiRxConfig(
    edidBinPath: String,
    simSpeedup: Boolean = false,
    clockDetectorConfig: ClockDetectorConfig = ClockDetectorConfig(),
    invertChannels: Seq[Boolean] = Seq.fill(3)(false),
    redBits: Int = 5,
    greenBits: Int = 6,
    blueBits: Int = 5
)

class HdmiRx[B <: BusDefinition.Bus](
    busDef: BusDefinition[B],
    config: HdmiRxConfig
) extends Component {
  import config._

  val AddressWidth = 10
  val DataWidth = 32

  val io = new Bundle {
    val bus = slave(busDef.createBus(AddressWidth, DataWidth))
    val interrupt = out Bool ()
    val hdmi = slave(HdmiIo())
    val ddc = master(I2c())
    val videoClk = out Bool ()
    val videoOut = master(Flow(HdmiVideo(redBits, greenBits, blueBits)))
  }

  val busIf = busDef.createBusInterface(io.bus, (0, 0x100))

  val control = new Area {
    val controlReg = busIf.newReg(doc = "Control").setName("control")
    val hpd =
      controlReg.field(
        Bool(),
        AccessType.RW,
        resetValue = 0,
        doc = "Hot plug detect enable"
      )(SymbolName("hpdEnable"))
    io.hdmi.hpd := hpd

    val pllPowerDown = controlReg.field(
      Bool(),
      AccessType.RW,
      resetValue = 1,
      doc = "PLL power down"
    )

    val pllReset = controlReg.field(
      Bool(),
      AccessType.RW,
      resetValue = 0,
      doc = "PLL reset"
    )

    val gtpReset = controlReg.field(
      Bool(),
      AccessType.RW,
      resetValue = 0,
      doc = "GTP transceiver reset"
    )
  }

  val status = new Area {
    val statusReg = busIf.newReg(doc = "Status").setName("status")

    val cableDetect =
      statusReg.field(Bool(), AccessType.RO, doc = "Cable detect")
    cableDetect := !io.hdmi.cableDetect

    val cableDetectChanged = statusReg.field(
      Bool(),
      AccessType.W1C,
      resetValue = 0,
      doc = "Cable detect status changed"
    )
    cableDetectChanged.setWhen(cableDetect.rise(initAt = False))
    cableDetectChanged.setWhen(cableDetect.fall(initAt = False))

    val pllLock = statusReg.field(Bool(), AccessType.RO, doc = "PLL is locked")

    val freqChanged = statusReg.field(
      Bool(),
      AccessType.W1C,
      resetValue = 0,
      doc = "Clock detector frequency changed"
    )

    val allGtpResetsDone =
      statusReg.field(
        Bool(),
        AccessType.RO,
        doc = "All channel GTP resets done"
      )

    val allHdmiDataValid =
      statusReg.field(Bool(), AccessType.RO, doc = "All channel data is valid")
  }

  val hdmiTmdsClk = Bool()
  val hdmiTmdsClkBuf =
    IBufDsGte2(clkIn = io.hdmi.clk, clkOut = hdmiTmdsClk, enable = True)

  val clockDetector = ClockDetector(clockDetectorConfig)
  clockDetector.io.hdmiTmdsClk := hdmiTmdsClk
  val clockDetectorBusIf = clockDetector.drive(busIf)
  status.freqChanged.setWhen(clockDetector.io.freqChanged.rise(initAt = False))

  val interruptCtrl = new Area {
    val interruptReg =
      busIf.newReg("Interrupt enables").setName("InterruptEnable")
    io.interrupt.clear()

    val cableDetectChangedEnable = interruptReg.field(
      Bool(),
      AccessType.RW,
      resetValue = 0,
      doc = "Enable cable detect changed interrupt"
    )
    io.interrupt.setWhen(
      cableDetectChangedEnable && status.cableDetectChanged
    )

    val pllLockEnable = interruptReg.field(
      Bool(),
      AccessType.RW,
      resetValue = 0,
      doc = "Enable PLL lock interrupt"
    )
    io.interrupt.setWhen(pllLockEnable && status.pllLock)

    val freqChangedEnable = interruptReg.field(
      Bool(),
      AccessType.RW,
      resetValue = 0,
      doc = "Enable frequency change interrupt"
    )
    io.interrupt.setWhen(freqChangedEnable && status.freqChanged)

    val allGtpResetsDoneEnable = interruptReg.field(
      Bool(),
      AccessType.RW,
      resetValue = 0,
      doc = "Enable all channel GTP resets done interrupt"
    )
    io.interrupt.setWhen(allGtpResetsDoneEnable && status.allGtpResetsDone)

    val allHdmiDataValidEnable = interruptReg.field(
      Bool(),
      AccessType.RW,
      resetValue = 0,
      doc = "Enable all channel data is valid interrupt"
    )
    io.interrupt.setWhen(allHdmiDataValidEnable && status.allHdmiDataValid)
  }

  val edid = new Edid(edidBinPath = edidBinPath)
  io.ddc <> edid.io.ddc

  val gtpCommon = new Gtpe2Common(
    pll0Config = Gtpe2PllConfig(
      refClkDiv = 1,
      fbDiv = 4,
      fbDiv45 = 5,
      simRefClkSelect = Gtpe2PllRefClk.GtRefClk0
    ),
    pll1Config = Gtpe2PllConfig.default(),
    simResetSpeedup = simSpeedup
  )
  gtpCommon.io.drp.disable()
  gtpCommon.io.pll1.disable()

  gtpCommon.io.clocking.disableEastWest()
  gtpCommon.io.clocking.gtRefClk0 := hdmiTmdsClkBuf.io.O
  gtpCommon.io.clocking.gtRefClk1 := False

  gtpCommon.io.pll0.refClkSelect := Gtpe2PllRefClk.GtRefClk0
  gtpCommon.io.pll0.lockDetectClk := False
  gtpCommon.io.pll0.powerDown := control.pllPowerDown
  gtpCommon.io.pll0.reset := control.pllReset
  status.pllLock := BufferCC(gtpCommon.io.pll0.lock, randBoot = true)

  val videoClkDomain = ClockDomain(
    clock = io.videoClk,
    reset = control.gtpReset
  )

  status.allGtpResetsDone.set()
  status.allHdmiDataValid.set()
  val channels = (0 to 2).map(i =>
    new Area {
      val channelStatus =
        new Area {
          val statusReg =
            busIf.newReg(doc = f"Channel $i status").setName(f"channel${i}")

          val gtpResetDone = statusReg.field(
            Bool(),
            AccessType.RO,
            doc = "GTP transceiver reset is done"
          )

          val hdmiDataOut0Valid = statusReg.field(
            Bool(),
            AccessType.RO,
            doc = "HDMI data out 0 is valid"
          )

          val hdmiDataOut1Valid = statusReg.field(
            Bool(),
            AccessType.RO,
            doc = "HDMI data out 1 is valid"
          )
        }

      val hdmiChannel = new HdmiChannel(
        invertPolarity = invertChannels(i),
        simResetSpeedup = simSpeedup,
        pll = 0,
        usrClkDomain = videoClkDomain
      )

      if (i == 0) {
        io.videoClk := BUFG.on(hdmiChannel.io.rxOutClk)
      }

      hdmiChannel.io.clocking.fromGtpe2Common(gtpCommon)
      hdmiChannel.io.pair <> io.hdmi.channels(i)
      hdmiChannel.io.reset := control.gtpReset
      channelStatus.gtpResetDone := hdmiChannel.io.resetDone
      status.allGtpResetsDone.clearWhen(!hdmiChannel.io.resetDone)
      channelStatus.hdmiDataOut0Valid := BufferCC(
        hdmiChannel.io.hdmiOut(0).valid,
        randBoot = true
      )
      channelStatus.hdmiDataOut1Valid := BufferCC(
        hdmiChannel.io.hdmiOut(1).valid,
        randBoot = true
      )
      status.allHdmiDataValid.clearWhen(
        !channelStatus.hdmiDataOut0Valid || !channelStatus.hdmiDataOut1Valid
      )
    }
  )

  val videoClkArea = new ClockingArea(videoClkDomain) {
    val videoOutFlow = Flow(HdmiVideo())
    val hSync = RegInit(False)
    videoOutFlow.hSync := hSync
    val vSync = RegInit(True)
    videoOutFlow.vSync := vSync

    val allValid = RegInit(False)
    when(
      channels.foldLeft(True)((acc, chan) => acc && chan.hdmiChannel.allValid)
    ) {
      allValid := True
    }
    videoOutFlow.valid := allValid

    val inDataIsland = RegInit(False)
    val chanOuts =
      channels.map(chan => chan.hdmiChannel.io.hdmiOut.map(co => co.payload))
    val syncOut = chanOuts(0)(1)
    val isPreamble = chanOuts(1)(1).isControl && chanOuts(2)(1).isControl
    val preamble = Cat(
      chanOuts(1)(1).c0,
      chanOuts(1)(1).c1,
      chanOuts(2)(1).c0,
      chanOuts(2)(1).c1
    )

    when(allValid) {
      when(syncOut.isControl) {
        hSync := syncOut.c0
        vSync := syncOut.c1
      }
      when(isPreamble && preamble === B"4'1010") {
        inDataIsland := True
      } elsewhen (isPreamble) {
        inDataIsland := False
      }
    }

    val pixelValidOut = videoOutFlow.payload.pixelsValid
    // use channel 0 as reference for valid pixel data
    pixelValidOut(0) := chanOuts(0)(0).isVideo && !inDataIsland
    pixelValidOut(1) := chanOuts(0)(1).isVideo && !inDataIsland

    // red is HDMI channel 2
    val redPixelsOut = videoOutFlow.payload.redPixels
    redPixelsOut(0) := chanOuts(2)(0).data(7 downto (8 - redBits))
    redPixelsOut(1) := chanOuts(2)(1).data(7 downto (8 - redBits))

    // green is HDMI channel 1
    val greenPixelsOut = videoOutFlow.payload.greenPixels
    greenPixelsOut(0) := chanOuts(1)(0).data(7 downto (8 - greenBits))
    greenPixelsOut(1) := chanOuts(1)(1).data(7 downto (8 - greenBits))

    // blue is HDMI channel 1
    val bluePixelsOut = videoOutFlow.payload.bluePixels
    bluePixelsOut(0) := chanOuts(0)(0).data(7 downto (8 - blueBits))
    bluePixelsOut(1) := chanOuts(0)(1).data(7 downto (8 - blueBits))

    io.videoOut := videoOutFlow.stage()
  }

  def svd(name: String, baseAddress: BigInt) = {
    SvdPeripheral(
      busIf,
      name,
      baseAddress,
      description = "HDMI Receiver"
    )
  }
}
