package slabware.hdmirx

import spinal.core._
import spinal.lib._
import spinal.lib.bus.regif.{AccessType, SymbolName}

import slabware.{BusDefinition, SvdPeripheral}

class HdmiRx[B <: BusDefinition.Bus](
    busDef: BusDefinition[B],
    edidBinPath: String
) extends Component {
  val AddressWidth = 10
  val DataWidth = 32

  val io = new Bundle {
    val bus = slave(busDef.createBus(AddressWidth, DataWidth))
    val hdmi = slave(new HdmiIo())
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
  }

  val status = new Area {
    val statusReg = busIf.newReg(doc = "Status").setName("status")

    val cableDetect =
      statusReg.field(Bool(), AccessType.RO, doc = "Cable detect")
    cableDetect := !io.hdmi.cableDetect

    val pllLock = statusReg.field(Bool(), AccessType.RO, doc = "PLL is locked")
  }

  val hdmiTmdsClk = Bool()
  val hdmiTmdsClkBuf =
    IBufDsGte2(clkIn = io.hdmi.clk, clkOut = hdmiTmdsClk, enable = True)

  val clockDetector = ClockDetector()
  clockDetector.io.hdmiTmdsClk := hdmiTmdsClk
  val clockDetectorBusIf = clockDetector.drive(busIf)

  val edid = new Edid(edidBinPath = edidBinPath)
  io.hdmi.ddc <> edid.io.ddc

  val gtpCommon = new Gtpe2Common(
    pll0Config = Gtpe2PllConfig(
      refClkDiv = 1,
      fbDiv = 4,
      fbDiv45 = 5,
      simRefClkSelect = Gtpe2PllRefClk.GtRefClk0
    ),
    pll1Config = Gtpe2PllConfig.default()
  )
  gtpCommon.io.drp.disable()

  gtpCommon.io.clocking.disableEastWest()
  gtpCommon.io.clocking.gtRefClk0 := hdmiTmdsClk
  gtpCommon.io.clocking.gtRefClk1 := False

  gtpCommon.io.pll0.refClkSelect := Gtpe2PllRefClk.GtRefClk0
  gtpCommon.io.pll0.lockDetectClk := False
  gtpCommon.io.pll0.powerDown := control.pllPowerDown
  gtpCommon.io.pll0.reset := control.pllReset
  status.pllLock := BufferCC(gtpCommon.io.pll0.lock, randBoot = true)

  gtpCommon.io.pll1.disable()

  def svd(name: String, baseAddress: BigInt) = {
    SvdPeripheral(
      busIf,
      name,
      baseAddress,
      description = "HDMI Receiver"
    )
  }
}
