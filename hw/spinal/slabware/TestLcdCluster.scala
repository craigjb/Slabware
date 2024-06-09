package slabware

import spinal.core._
import spinal.core.sim._

object TestLcdCluster extends App {
  val BroadcastSequence = Seq(
    0x13a, 0x05, 0x136, 0xd0, 0x121, 0x111, 0x129
  )

  SimConfig.withWave
    .compile(
      new LcdCluster(
        broadcastFifoDepth = 8
      )
    )
    .doSim { dut =>
      dut.io.broadcastIn.valid #= false
      dut.io.broadcastIn.payload #= 0
      dut.io.frameEnable #= false
      dut.io.frameDataStream.valid #= false
      dut.io.frameDataStream.payload #= 0

      dut.clockDomain.forkStimulus(period = 10)
      dut.clockDomain.waitSampling(1)

      for (data <- BroadcastSequence) {
        dut.io.broadcastIn.payload #= data
        dut.io.broadcastIn.valid #= true
        dut.clockDomain.waitSampling(1)
      }
      dut.io.broadcastIn.valid #= false
      dut.clockDomain.waitSampling((10 * 4 * BroadcastSequence.length) + 2)
      dut.clockDomain.waitSampling(50)

      dut.io.frameEnable #= true
      dut.clockDomain.waitSampling(
        (10 * LcdCluster.FrameStartupSequence.length) + 2
      )

      dut.io.frameDataStream.valid #= true
      dut.io.frameDataStream.payload #= 0x01020304
      dut.clockDomain.waitSamplingWhere(dut.io.frameDataStream.ready.toBoolean)

      dut.io.frameDataStream.payload #= 0x05060708
      dut.clockDomain.waitSamplingWhere(dut.io.frameDataStream.ready.toBoolean)

      dut.io.frameDataStream.valid #= false
      dut.io.frameDataStream.payload #= 0
      dut.clockDomain.waitSamplingWhere(dut.io.frameDataStream.ready.toBoolean)
      dut.clockDomain.waitSampling(10)

      dut.io.frameDataStream.valid #= true
      dut.io.frameDataStream.payload #= 0x090a0b0c
      dut.clockDomain.waitSamplingWhere(dut.io.frameDataStream.ready.toBoolean)
      dut.io.frameDataStream.payload #= 0x0d0e0f10
      dut.clockDomain.waitSamplingWhere(dut.io.frameDataStream.ready.toBoolean)
      dut.io.frameDataStream.payload #= 0x11121314
      dut.clockDomain.waitSamplingWhere(dut.io.frameDataStream.ready.toBoolean)

      dut.io.frameDataStream.valid #= false
      dut.clockDomain.waitSampling(10 * 4)
      dut.clockDomain.waitSampling(10)

      dut.io.frameDataStream.valid #= true
      dut.io.frameDataStream.payload #= 0x090a0b0c
      for (i <- 0 until (128 * 128)) {
        dut.clockDomain.waitSamplingWhere(
          dut.io.frameDataStream.ready.toBoolean
        )
      }
      dut.clockDomain.waitSampling(10)
    }
}
