package slabware

import spinal.core._
import spinal.lib._
import spinal.lib.bus.amba4.axi._

import zynqps.ZynqPs

class Slabware(
    dimCounterWidth: BigInt,
    dimPeriodDefault: BigInt,
    dimDutyDefault: Double,
    broadcastFifoDepth: Int,
    frameDataFifoDepth: Int
) extends Component {

  val io = new Bundle {
    val leds = out Bits (4 bits)
    val lcd_reset = out Bool ()
    val lcd_dim = out Bool ()

    val lcdSpi0 = out(LcdCluster.SpiBus())
    val lcdSpi1 = out(LcdCluster.SpiBus())
    val lcdSpi2 = out(LcdCluster.SpiBus())
    val lcdSpi3 = out(LcdCluster.SpiBus())
  }

  val zynqPs = new ZynqPs()

  val spiClockDomain = ClockDomain(
    clock = zynqPs.io.fClk0,
    reset = zynqPs.io.fResetN0,
    config = ClockDomainConfig(
      clockEdge = RISING,
      resetKind = SYNC,
      resetActiveLevel = LOW
    )
  )
  zynqPs.io.mAxiGp0AClk := spiClockDomain.readClockWire

  val spiClockArea = new ClockingArea(spiClockDomain) {
    val baseAddress = 0x40000000
    val ledRegAddress = baseAddress
    val ctrlRegAddress = baseAddress + 4
    val broadcastBusyAddr = baseAddress + 8
    val broadcastWordAddr = baseAddress + 12
    val dimBaseAddress = baseAddress + 16

    val bus = Axi4SlaveFactory(zynqPs.io.mAxiGp0)

    val ledReg = RegInit(B"0000")
    io.leds := ledReg
    bus.readAndWrite(ledReg, ledRegAddress, 0)

    val ctrlReg = RegInit(B"000")
    bus.readAndWrite(ctrlReg, ctrlRegAddress, 0)

    io.lcd_reset := !ctrlReg(0)
    val backlightEnable = ctrlReg(1)
    val lcdFrameEnable = ctrlReg(2)

    val lcdBroadcastWord =
      bus.createAndDriveFlow(Bits(9 bits), broadcastWordAddr, 0)

    val lcdCluster0 =
      LcdCluster(
        broadcastFifoDepth,
        spiBus = io.lcdSpi0,
        broadcastIn = lcdBroadcastWord,
        frameEnable = lcdFrameEnable
      )
    val lcdCluster1 =
      LcdCluster(
        broadcastFifoDepth,
        spiBus = io.lcdSpi1,
        broadcastIn = lcdBroadcastWord,
        frameEnable = lcdFrameEnable
      )
    val lcdCluster2 =
      LcdCluster(
        broadcastFifoDepth,
        spiBus = io.lcdSpi2,
        broadcastIn = lcdBroadcastWord,
        frameEnable = lcdFrameEnable
      )
    val lcdCluster3 =
      LcdCluster(
        broadcastFifoDepth,
        spiBus = io.lcdSpi3,
        broadcastIn = lcdBroadcastWord,
        frameEnable = lcdFrameEnable
      )

    val lcdBroadcastBusy = lcdCluster0.io.broadcastBusy ||
      lcdCluster1.io.broadcastBusy ||
      lcdCluster2.io.broadcastBusy ||
      lcdCluster3.io.broadcastBusy
    bus.read(lcdBroadcastBusy, broadcastBusyAddr, 0)

    val dim = LcdDim(
      counterWidth = dimCounterWidth,
      defaultPeriod = dimPeriodDefault,
      defaultDuty = dimDutyDefault,
      enable = backlightEnable,
      pwmOut = io.lcd_dim,
      bus = bus,
      addrOffset = dimBaseAddress
    )
  }

  val lcdClusters = Seq(
    spiClockArea.lcdCluster0,
    spiClockArea.lcdCluster1,
    spiClockArea.lcdCluster2,
    spiClockArea.lcdCluster3
  )

  val axiClockArea = new ClockingArea(spiClockDomain) {
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

    val memByteCount = 128 * 128 * 2
    val memWordCount = memByteCount / 4
    val frameMem = Mem(
      Bits(32 bits),
      Utils.read32BitMemFromFile("benedict.bin", memWordCount)
    )
    for (cluster <- lcdClusters) {
      val addr = RegInit(U(0, (log2Up(memWordCount - 1) bits)))
      val valid = RegNext(True).init(False)
      val data = RegInit(B(0, 32 bits))

      data := frameMem.readSync(addr.resized)

      cluster.io.frameDataStream.valid := valid
      cluster.io.frameDataStream.payload := data

      when(!spiClockArea.lcdFrameEnable) {
        valid := False
        addr := 0
      } otherwise {
        when(cluster.io.frameDataStream.fire) {
          addr := addr + 1
        }
      }
    }
  }
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
        broadcastFifoDepth = 8,
        frameDataFifoDepth = 32
      )
    )
  }
}
