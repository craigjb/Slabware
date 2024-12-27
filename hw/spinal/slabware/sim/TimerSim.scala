package slabware.sim

import spinal.core._
import spinal.core.sim._
import spinal.lib.bus.amba3.apb.sim._

import slabware.{TimerCtrl, Apb3Bus}

object TestTimer extends App {
  SimConfig.withWave
    .compile(new TimerCtrl(Apb3Bus, prescalerResetValue = 4))
    .doSim { dut =>
      val driver = new Apb3Driver(dut.io.bus, dut.clockDomain)
      dut.clockDomain.forkStimulus(period = 10)
      dut.clockDomain.waitSampling(10)

      // compare 0
      driver.write(0x18, 100)
      // mask compare 1
      driver.write(0x14, 0x4)
      // enable with interrupts
      driver.write(0x8, 0x5)

      dut.clockDomain.waitSampling(1000)
    }
}
