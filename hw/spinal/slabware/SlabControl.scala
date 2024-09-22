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

import vexriscv._
import vexriscv.plugin._

class SlabControl extends Component {
  val io = new Bundle {
    val leds = out(Bits(8 bits))
    val i2c = master(I2c())
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
      cmdForkPersistence = true
    ),
    new DBusSimplePlugin(
      catchAddressMisaligned = false,
      catchAccessFault = false
    ),
    new DecoderSimplePlugin(
      catchIllegalInstruction = true
    ),
    new RegFilePlugin(
      regFileReadyKind = plugin.SYNC,
      zeroBoot = false
    ),
    new IntAluPlugin,
    new SrcPlugin(
      separatedAddSub = false,
      executeInsertion = true
    ),
    new LightShifterPlugin,
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
      catchAddressMisaligned = false
    ),
    new CsrPlugin(
      config = CsrPluginConfig.smallest.copy(
        mtvecInit = 0x0,
        ebreakGen = true,
        withPrivilegedDebug = true
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
      withTap = false
    )
    // new DebugPlugin(
    //   debugClockDomain,
    //   hardwareBreakpointCount = 3,
    //   BreakpointReadback = true
    // ),
    // new YamlPlugin("cpu.yaml")
  )

  val cpuConfig = VexRiscvConfig(
    plugins = cpuPlugins
  )

  new ClockingArea(sysClkDomain) {
    val cpu = new VexRiscv(cpuConfig)
    var iBus: Axi4ReadOnly = null
    var dBus: Axi4Shared = null
    for (plugin <- cpuConfig.plugins) plugin match {
      case plugin: IBusSimplePlugin => iBus = plugin.iBus.toAxi4ReadOnly()
      case plugin: DBusSimplePlugin => dBus = plugin.dBus.toAxi4Shared()
      case plugin: CsrPlugin => {
        plugin.externalInterrupt := False
        plugin.timerInterrupt := False
      }
      case plugin: DebugPlugin =>
        plugin.io.bus.fromBscane2(
          usedId = 2,
          jtagHeaderIgnoreWidth = 0
        )
      case plugin: EmbeddedRiscvJtag => {
        plugin.jtagInstruction <> jtagTap.toJtagTapInstructionCtrl()
        sysReset := plugin.ndmreset
      }
      case _ =>
    }

    val ram = Axi4SharedOnChipRam(
      dataWidth = 32,
      byteCount = 16 kB,
      idWidth = 4
    )

    val apbBridge = Axi4SharedToApb3Bridge(
      addressWidth = 16,
      dataWidth = 32,
      idWidth = 0
    )

    val axiCrossbar = Axi4CrossbarFactory()
    val apbBase = 0x10000000L
    axiCrossbar.addSlaves(
      ram.io.axi -> (0x00000000L, ram.byteCount),
      apbBridge.io.axi -> (apbBase, 8 kB)
    )
    axiCrossbar.addConnections(
      iBus -> List(ram.io.axi),
      dBus -> List(ram.io.axi, apbBridge.io.axi)
    )
    axiCrossbar.build()

    val ledCtrl = new LedCtrl(Apb3Bus, numLeds = 8)
    io.leds := ledCtrl.io.leds

    val i2cCtrl = new I2cCtrl(
      Apb3Bus,
      I2cSlaveMemoryMappedGenerics(
        ctrlGenerics = I2cSlaveGenerics(
          samplingWindowSize = 3,
          samplingClockDividerWidth = 12 bits,
          timeoutWidth = 20 bits
        ),
        addressFilterCount = 0,
        masterGenerics = I2cMasterMemoryMappedGenerics(timerWidth = 16)
      )
    )
    io.i2c <> i2cCtrl.io.i2c

    val ledCtrlOffset = 0x0

    val i2cCtrlOffset = 0x1000
    val apbDecoder = Apb3Decoder(
      master = apbBridge.io.apb,
      slaves = Seq(
        (ledCtrl.io.bus -> (ledCtrlOffset, 4 kB)),
        (i2cCtrl.io.bus -> (i2cCtrlOffset, 4 kB))
      )
    )

    val ledCtrlBase = apbBase + ledCtrlOffset
    val i2cCtrlBase = apbBase + i2cCtrlOffset

    val svd = SvdGenerator(
      "slabware",
      peripherals = Seq(
        ledCtrl.svd("LEDs", baseAddress = ledCtrlBase),
        i2cCtrl.svd("I2C0", baseAddress = i2cCtrlBase)
      ),
      description = "Slabware control system"
    )
    svd.dump("fw/slab-pac/slabware.svd")
  }
}
