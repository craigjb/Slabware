package slabware

import spinal.core._
import spinal.core.sim._

object TestLcdDim extends App {
  SimConfig.withWave
    .compile(
      new LcdDim(
        counterWidth = 8
      )
    )
    .doSim { dut =>
      val dimPeriodDefault = BigInt(2).pow(8) - 1;
      val defaultDuty = 0.5;
      val duty = (dimPeriodDefault.toDouble * defaultDuty).toInt

      dut.io.period #= dimPeriodDefault
      dut.io.duty #= duty

      dut.clockDomain.forkStimulus(period = 10)
      dut.clockDomain.waitSampling(2)

      dut.io.enable #= true
      dut.clockDomain.waitSampling(256 * 10)

      dut.clockDomain.waitSampling(50)
      dut.io.duty #= 0x10

      dut.clockDomain.waitSampling(256 * 10)
    }
}
