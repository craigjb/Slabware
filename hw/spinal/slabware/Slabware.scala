package slabware

import spinal.core._
import spinal.lib._
import spinal.lib.bus.amba4.axi._

class Slabware(
    dimCounterWidth: BigInt,
    dimPeriodDefault: BigInt,
    dimDutyDefault: Double,
    broadcastFifoDepth: Int,
    numSpiClusters: Int = 36,
    numLcdDims: Int = 9
) extends Component {

  val io = new Bundle {
    // 100 MHz clock input
    val SYSCLK = in Bool ()

    // 8 Debug LEDs
    val LED = out Bits (8 bits)

    // LCD SPI interfaces
    val RESET = out Bool ()
    val SDA = out Bits (numSpiClusters bits)
    val SCL = out Bits (numSpiClusters bits)
    val DC = out Bits (numSpiClusters bits)
    val DSA = out Bits (numSpiClusters bits)
    val DSB = out Bits (numSpiClusters bits)

    // Backlight PWM lines
    val DIM = out Bits (numLcdDims bits)

    // Button matrix
    val BTNCOL = out Bits (18 bits)
    val BTNROW = in Bits (8 bits)
  }
  noIoPrefix()

  io.LED.setAll()
  io.RESET := False
  io.BTNCOL.setAll()

  val spiClockDomain = ClockDomain(
    clock = io.SYSCLK,
    config = ClockDomainConfig(
      clockEdge = RISING,
      resetKind = BOOT,
      resetActiveLevel = HIGH
    )
  )

  val spiClockArea = new ClockingArea(spiClockDomain) {
    val lcdBroadcastWord = Flow(Bits(9 bits))
    lcdBroadcastWord.setIdle()
    val lcdFrameEnable = False

    val lcdClusters = Range(0, numSpiClusters)
      .map(index => {
        val cluster = LcdCluster(
          broadcastFifoDepth,
          scl = io.SCL(index),
          sda = io.SDA(index),
          dc = io.DC(index),
          dsa = io.DSA(index),
          dsb = io.DSB(index),
          broadcastIn = lcdBroadcastWord,
          frameEnable = lcdFrameEnable
        )
        cluster.io.frameDataStream.setIdle()
        cluster
      })

    val backlightEnable = True
    val lcdDims = Range(0, numLcdDims)
      .map(index =>
        LcdDim(
          counterWidth = dimCounterWidth,
          defaultPeriod = dimPeriodDefault,
          defaultDuty = dimDutyDefault,
          enable = backlightEnable,
          pwmOut = io.DIM(index)
        )
      )

    val wordLoaded = RegInit(False)
    when(!wordLoaded) {
      lcdBroadcastWord.push(B"9'x1FF")
      wordLoaded := True
    }
  }

  // val axiClockArea = new ClockingArea(spiClockDomain) {
  //   val memByteCount = 128 * 128 * 2
  //   val memWordCount = memByteCount / 4
  //   val frameMem = Mem(
  //     Bits(32 bits),
  //     Utils.read32BitMemFromFile("benedict.bin", memWordCount)
  //   )

  //   val axiConfig = ZynqPs.mAxiGpConfig
  //   val frameMemAxi4ReadOnly =
  //     Axi4SharedOnChipRamPort(config = axiConfig, ram = frameMem).axi
  //   frameMemAxi4ReadOnly.writeData.setIdle()
  //   frameMemAxi4ReadOnly.writeRsp.setBlocked()
  //   frameMemAxi4ReadOnly.sharedCmd.write := False

  //   val enabled = BufferCC(spiClockArea.lcdFrameEnable)
  //   val currentCluster = RegInit(U(0, log2Up(lcdClusters.length) bits))

  //   for (clusterIndex <- 0 until lcdClusters.length) {
  //     val cluster = lcdClusters(clusterIndex)
  //     val clusterFifo = StreamFifoCC(
  //       dataType = cluster.io.frameDataStream.payloadType,
  //       depth = frameDataFifoDepth,
  //       pushClock = spiClockDomain,
  //       popClock = spiClockDomain
  //     )
  //     clusterFifo.io.pop >> cluster.io.frameDataStream

  //     val clusterNeedsData =
  //       clusterFifo.io.pushOccupancy <= (frameDataFifoDepth - 16)
  //     val clusterReadAddress = RegInit(U(0, 32 bits))

  //     when(currentCluster === clusterIndex) {
  //       frameMemAxi4ReadOnly.sharedCmd.addr := clusterReadAddress
  //       frameMemAxi4ReadOnly.sharedCmd.id := clusterIndex
  //     }
  //   }

  //   val memByteCount = 128 * 128 * 2
  //   val memWordCount = memByteCount / 4
  //   val frameMem = Mem(
  //     Bits(32 bits),
  //     Utils.read32BitMemFromFile("benedict.bin", memWordCount)
  //   )
  //   for (cluster <- lcdClusters) {
  //     val addr = RegInit(U(0, (log2Up(memWordCount - 1) bits)))
  //     val valid = RegNext(True).init(False)
  //     val data = RegInit(B(0, 32 bits))

  //     data := frameMem.readSync(addr.resized)

  //     cluster.io.frameDataStream.valid := valid
  //     cluster.io.frameDataStream.payload := data

  //     when(!spiClockArea.lcdFrameEnable) {
  //       valid := False
  //       addr := 0
  //     } otherwise {
  //       when(cluster.io.frameDataStream.fire) {
  //         addr := addr + 1
  //       }
  //     }
  //   }
  // }
}

object TopLevelVerilog {
  def main(args: Array[String]): Unit = {
    SpinalConfig(
      inlineRom = true
    ).generateVerilog(
      new Slabware(
        dimCounterWidth = 8,
        dimPeriodDefault = BigInt(2).pow(8) - 1,
        dimDutyDefault = 0.5,
        broadcastFifoDepth = 8
      )
    )
  }
}
