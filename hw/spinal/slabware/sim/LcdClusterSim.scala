package slabware.sim

import spinal.core._
import spinal.core.sim._

import slabware.LcdCluster

object LcdClusterSim extends App {
  val BroadcastSequence = Seq(
    0x13a, 0x05, 0x136, 0xd0, 0x121, 0x111, 0x129
  )

  SimConfig.withWave
    .compile(
      new LcdCluster()
    )
    .doSim { dut =>
      dut.io.broadcastIn.valid #= false
      dut.io.broadcastIn.payload #= 0
      dut.io.frameEnable #= false
      dut.io.frameDataStream.valid #= false

      dut.clockDomain.forkStimulus(period = 10)
      dut.clockDomain.waitSampling(1)

      for (data <- BroadcastSequence) {
        dut.io.broadcastIn.payload #= data
        dut.io.broadcastIn.valid #= true
        dut.clockDomain.waitSamplingWhere(dut.io.broadcastIn.ready.toBoolean)
      }
      dut.io.broadcastIn.valid #= false
      dut.clockDomain.waitSampling((10 * 4 * BroadcastSequence.length) + 2)
      dut.clockDomain.waitSampling(50)

      dut.io.frameEnable #= true
      dut.clockDomain.waitSampling(
        (10 * LcdCluster.FrameStartupSequence.length) + 2
      )

      dut.io.frameDataStream.payload(0).valid #= false
      dut.io.frameDataStream.payload(0).data #= 0x0000
      dut.io.frameDataStream.payload(1).valid #= true
      dut.io.frameDataStream.payload(1).data #= 0x0000
      dut.io.frameDataStream.valid #= true
      dut.clockDomain.waitSamplingWhere(dut.io.frameDataStream.ready.toBoolean)

      dut.io.frameDataStream.payload(0).valid #= true
      dut.io.frameDataStream.payload(0).data #= 0x0001
      dut.io.frameDataStream.payload(1).valid #= true
      dut.io.frameDataStream.payload(1).data #= 0x0002
      dut.io.frameDataStream.valid #= true
      dut.clockDomain.waitSamplingWhere(dut.io.frameDataStream.ready.toBoolean)

      dut.io.frameDataStream.valid #= false
      dut.clockDomain.waitSampling(10 * 4)

      for (i <- 3 until ((128 * 128) / 2 + 10)) {
        dut.io.frameDataStream.valid #= true
        dut.io.frameDataStream.payload(0).valid #= true
        dut.io.frameDataStream.payload(0).data #= i
        dut.io.frameDataStream.payload(1).valid #= true
        dut.io.frameDataStream.payload(1).data #= i + 1
        dut.clockDomain.waitSamplingWhere(
          dut.io.frameDataStream.ready.toBoolean
        )
      }
      dut.io.frameDataStream.valid #= false
      dut.clockDomain.waitSampling(10 * 30)
    }
}
