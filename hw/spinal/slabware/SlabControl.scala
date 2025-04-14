package slabware

import scala.collection.mutable.ArrayBuffer

import spinal.core._
import spinal.lib._
import spinal.lib.bus.amba3.apb._
import spinal.lib.bus.amba4.axi._
import spinal.lib.com.uart._
import spinal.lib.com.i2c._
import spinal.lib.cpu.riscv.debug.DebugTransportModuleParameter
import spinal.lib.blackbox.xilinx.s7.BSCANE2
import spinal.lib.com.usb.udc.{
  UsbDeviceCtrlParameter,
  UsbDeviceCtrl => SpinalUsbCtrl
}

import vexriscv._
import vexriscv.plugin._

import slabware.hdmirx.{HdmiRx, HdmiRxConfig, HdmiIo, HdmiVideo}
import slabware.Utils._

class SlabControl(
    firmwareBinPath: String = null,
    numGridRows: Int = 8,
    numGridCols: Int = 18
) extends Component {
  val io = new Bundle {
    val leds = out(Bits(8 bits))
    val hdmiCtrlI2c = master(I2c())
    val hdmi = slave(HdmiIo())
    val ddc = master(I2c())
    val videoOut = master(Flow(HdmiVideo()))
    val lcdPwmOut = out Bool ()
    val gridEnable = out Bool ()
    val btnRow = out Bits (numGridRows bits)
    val btnCol = in Bits (numGridCols bits)
    val usbPhy = master(SpinalUsbCtrl.PhyIo())
  }

  val sysReset = Bool()
  val sysClkDomain = ClockDomain(
    clock = ClockDomain.current.readClockWire,
    reset = sysReset,
    frequency = ClockDomain.current.frequency
  )

  val debugReset = False
  val debugClockDomain = ClockDomain(
    clock = ClockDomain.current.readClockWire,
    reset = debugReset,
    frequency = ClockDomain.current.frequency
  )

  val jtagTap = BSCANE2(userId = 4)
  val jtagClockDomain = ClockDomain(
    clock = jtagTap.TCK
  )

  val cpuPlugins = ArrayBuffer(
    new IBusSimplePlugin(
      resetVector = 0x00000000L,
      cmdForkOnSecondStage = false,
      cmdForkPersistence = true,
      compressedGen = true
    ),
    new DBusSimplePlugin(
      catchAddressMisaligned = false,
      catchAccessFault = false,
      withLrSc = true
    ),
    new DecoderSimplePlugin(
      catchIllegalInstruction = true
    ),
    new RegFilePlugin(
      regFileReadyKind = plugin.SYNC,
      zeroBoot = false
    ),
    new IntAluPlugin,
    new MulPlugin,
    new DivPlugin,
    new SrcPlugin(
      separatedAddSub = false,
      executeInsertion = true
    ),
    new FullBarrelShifterPlugin,
    new HazardSimplePlugin(
      bypassExecute = true,
      bypassMemory = true,
      bypassWriteBack = true,
      bypassWriteBackBuffer = true,
      pessimisticUseSrc = false,
      pessimisticWriteRegFile = false,
      pessimisticAddressMatch = false
    ),
    new BranchPlugin(
      earlyBranch = false,
      catchAddressMisaligned = true
    ),
    new CsrPlugin(
      config = CsrPluginConfig.smallest.copy(
        misaExtensionsInit = 70,
        misaAccess = CsrAccess.READ_ONLY,
        mtvecInit = 0x0,
        mtvecAccess = CsrAccess.READ_WRITE,
        ebreakGen = true,
        withPrivilegedDebug = true,
        wfiGenAsWait = true
      )
    ),
    new EmbeddedRiscvJtag(
      p = DebugTransportModuleParameter(
        addressWidth = 7,
        version = 1,
        idle = 7
      ),
      debugCd = debugClockDomain,
      jtagCd = jtagClockDomain,
      withTunneling = true,
      withTap = false,
      withSysBus = true
    )
  )

  val mi2cInterruptPlugin = new UserInterruptPlugin(
    interruptName = "mi2cInt",
    code = 20
  )
  cpuPlugins += mi2cInterruptPlugin

  val hdmiRxInterruptPlugin = new UserInterruptPlugin(
    interruptName = "hdmiRxInt",
    code = 21
  )
  cpuPlugins += hdmiRxInterruptPlugin

  val usbInterruptPlugin = new UserInterruptPlugin(
    interruptName = "usbInt",
    code = 22
  )
  cpuPlugins += usbInterruptPlugin

  val cpuConfig = VexRiscvConfig(
    plugins = cpuPlugins
  )

  val sysClkArea = new ClockingArea(sysClkDomain) {
    val cpu = new VexRiscv(cpuConfig)

    var iBus: Axi4ReadOnly = null
    var dBus: Axi4Shared = null
    var debugSysBus: Axi4Shared = null
    val timerInterrupt = Bool()
    for (plugin <- cpuConfig.plugins) plugin match {
      case plugin: IBusSimplePlugin => iBus = plugin.iBus.toAxi4ReadOnly()
      case plugin: DBusSimplePlugin => dBus = plugin.dBus.toAxi4Shared()
      case plugin: CsrPlugin => {
        plugin.externalInterrupt := False
        plugin.timerInterrupt := timerInterrupt
      }
      case plugin: DebugPlugin =>
        plugin.io.bus.fromBscane2(
          usedId = 2,
          jtagHeaderIgnoreWidth = 0
        )
      case plugin: EmbeddedRiscvJtag => {
        plugin.jtagInstruction <> jtagTap.toJtagTapInstructionCtrl()
        sysReset := plugin.ndmreset
        debugSysBus = plugin.sysBus.toAxi4Shared()
      }
      case _ =>
    }

    val ram = Axi4SharedOnChipRam(
      dataWidth = 32,
      byteCount = 32 kB,
      idWidth = 4
    )

    if (firmwareBinPath != null) {
      ram.ram.init(read32BitMemFromFile(firmwareBinPath, ram.wordCount.toInt))
    }

    val apbBridge = Axi4SharedToApb3Bridge(
      addressWidth = 17,
      dataWidth = 32,
      idWidth = 4
    )

    val axiCrossbar = Axi4CrossbarFactory()
    val apbBase = 0x10000000L
    axiCrossbar.addSlaves(
      ram.io.axi -> (0x00000000L, ram.byteCount),
      apbBridge.io.axi -> (apbBase, 128 kB)
    )
    axiCrossbar.addConnections(
      iBus -> List(ram.io.axi),
      dBus -> List(ram.io.axi, apbBridge.io.axi),
      debugSysBus -> List(ram.io.axi, apbBridge.io.axi)
    )
    axiCrossbar.build()

    val ledCtrl = new LedCtrl(Apb3Bus, numLeds = 8)
    io.leds := ledCtrl.io.leds

    val mi2cCtrl = new I2cCtrl(
      Apb3Bus,
      I2cSlaveMemoryMappedGenerics(
        ctrlGenerics = I2cSlaveGenerics(),
        masterGenerics = I2cMasterMemoryMappedGenerics(timerWidth = 16)
      )
    )
    io.hdmiCtrlI2c <> mi2cCtrl.io.i2c
    mi2cInterruptPlugin.interrupt := mi2cCtrl.io.interrupt

    val timerCtrl = new TimerCtrl(Apb3Bus)
    timerInterrupt := timerCtrl.io.interrupt

    val hdmiRx =
      new HdmiRx(
        Apb3Bus,
        HdmiRxConfig(
          edidBinPath = "SoundSlab.edid",
          invertChannels = Seq(true, false, false)
        )
      )
    hdmiRx.io.hdmi <> io.hdmi
    hdmiRx.io.ddc <> io.ddc
    hdmiRx.io.videoOut >> io.videoOut
    hdmiRxInterruptPlugin.interrupt := hdmiRx.io.interrupt

    val lcdDim = new LcdDim(Apb3Bus)
    io.lcdPwmOut := lcdDim.io.pwmOut

    val gridCtrl = new SlabGridCtrl(Apb3Bus)
    io.gridEnable := gridCtrl.io.gridEnable
    io.btnRow := gridCtrl.io.btnRow
    gridCtrl.io.btnCol := io.btnCol

    val usbCtrl =
      new UsbDeviceCtrl(Apb3Bus, UsbDeviceCtrlParameter(addressWidth = 14))
    usbInterruptPlugin.interrupt := usbCtrl.io.interrupt
    io.usbPhy <> usbCtrl.io.phy

    val ledCtrlOffset = 0x0
    val timerCtrlOffset = 0x400
    val mi2cCtrlOffset = 0x800
    val hdmiRxOffset = 0xc00
    val lcdDimOffset = 0x1000
    val gridCtrlOffset = 0x1400
    val usbCtrlOffset = 0x10000

    val apbDecoder = Apb3Decoder(
      master = apbBridge.io.apb,
      slaves = Seq(
        (ledCtrl.io.bus -> (ledCtrlOffset, 1 kB)),
        (timerCtrl.io.bus -> (timerCtrlOffset, 1 kB)),
        (mi2cCtrl.io.bus -> (mi2cCtrlOffset, 1 kB)),
        (hdmiRx.io.bus -> (hdmiRxOffset, 1 kB)),
        (lcdDim.io.bus -> (lcdDimOffset, 1 kB)),
        (gridCtrl.io.bus -> (gridCtrlOffset, 1 kB)),
        (usbCtrl.io.bus -> (usbCtrlOffset, 64 kB))
      )
    )

    val ledCtrlBase = apbBase + ledCtrlOffset
    val timerCtrlBase = apbBase + timerCtrlOffset
    val mi2cCtrlBase = apbBase + mi2cCtrlOffset
    val hdmiRxBase = apbBase + hdmiRxOffset
    val lcdDimBase = apbBase + lcdDimOffset
    val gridCtrlBase = apbBase + gridCtrlOffset
    val usbCtrlBase = apbBase + usbCtrlOffset

    val svd = SvdGenerator(
      "slabware",
      peripherals = Seq(
        ledCtrl.svd("Leds", baseAddress = ledCtrlBase),
        timerCtrl.svd("Timer", baseAddress = timerCtrlBase),
        mi2cCtrl.svd("Mi2c", baseAddress = mi2cCtrlBase),
        hdmiRx.svd("HdmiRX", baseAddress = hdmiRxBase),
        lcdDim.svd("LcdDim", baseAddress = lcdDimBase),
        gridCtrl.svd("GridCtrl", baseAddress = gridCtrlBase),
        usbCtrl.svd("UsbCtrl", baseAddress = usbCtrlBase)
      ),
      description = "Slabware control system"
    )
    svd.dump("fw/slab-pac/slabware.svd")
  }

  def videoClkDomain: ClockDomain = {
    sysClkArea.hdmiRx.videoClkDomain
  }
}
