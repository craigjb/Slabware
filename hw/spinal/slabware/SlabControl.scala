package slabware

import scala.collection.mutable.ArrayBuffer

import spinal.core._
import spinal.lib._
import spinal.lib.bus.amba3.apb._
import spinal.lib.bus.amba4.axi._
import spinal.lib.com.uart._

import vexriscv._
import vexriscv.plugin._

class SlabControl extends Component {
  val io = new Bundle {
    val leds = out(Bits(8 bits))
  }

  val debugClockDomain = ClockDomain(
    clock = ClockDomain.current.readClockWire,
    frequency = ClockDomain.current.frequency,
    config = ClockDomainConfig(
      resetKind = BOOT
    )
  )

  val cpuPlugins = ArrayBuffer(
    new IBusSimplePlugin(
      resetVector = 0x80000000L,
      cmdForkOnSecondStage = false,
      cmdForkPersistence = true,
      prediction = NONE,
      catchAccessFault = false,
      compressedGen = false
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
      config = CsrPluginConfig.smallest.copy(ebreakGen = true)
    ),
    new DebugPlugin(
      debugClockDomain,
      hardwareBreakpointCount = 3,
      BreakpointReadback = true
    ),
    new YamlPlugin("cpu.yaml")
  )

  val cpuConfig = VexRiscvConfig(
    plugins = cpuPlugins
  )

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
    case _ =>
  }

  val ram = Axi4SharedOnChipRam(
    dataWidth = 32,
    byteCount = 4 kB,
    idWidth = 4
  )

  val apbBridge = Axi4SharedToApb3Bridge(
    addressWidth = 32,
    dataWidth = 32,
    idWidth = 0
  )
  val slaveFactory = Apb3SlaveFactory(apbBridge.io.apb)
  val ledReg = slaveFactory.createReadWrite(Bits(8 bits), 0xf0000000L, 0)
  io.leds := ledReg

  // val uartCtrl = Apb3UartCtrl(
  //   UartCtrlMemoryMappedConfig(
  //     uartCtrlConfig = UartCtrlGenerics(),
  //     initConfig = UartCtrlInitConfig(
  //       baudrate = 921600,
  //       dataLength = 7, // 8 bits
  //       parity = UartParityType.NONE,
  //       stop = UartStopType.ONE
  //     ),
  //     busCanWriteClockDividerConfig = true,
  //     busCanWriteFrameConfig = true,
  //     txFifoDepth = 32,
  //     rxFifoDepth = 32
  //   )
  // )

  val axiCrossbar = Axi4CrossbarFactory()
  axiCrossbar.addSlaves(
    ram.io.axi -> (0x00000000L, ram.byteCount),
    apbBridge.io.axi -> (0xf0000000L, 4 kB)
  )
  axiCrossbar.addConnections(
    iBus -> List(ram.io.axi),
    dBus -> List(ram.io.axi, apbBridge.io.axi)
  )
  axiCrossbar.build()

}