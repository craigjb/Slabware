package slabware.sim

import spinal.core._
import spinal.core.sim._
import spinal.lib.bus.amba3.apb.sim._

import slabware.{SlabGridCtrl, Apb3Bus}

object SlabGridCtrlSim extends App {
  SimConfig
    .withConfig(
      SpinalConfig(defaultClockDomainFrequency = FixedFrequency(50 MHz))
    )
    .withWave
    .compile(
      new SlabGridCtrl(Apb3Bus, debounceSampleRate = 25 MHz)
    )
    .doSim { dut =>
      dut.clockDomain.forkStimulus(period = 10)

      val driver = new Apb3Driver(dut.io.bus, dut.clockDomain)
      dut.io.btnCol #= 0x3ffff

      // row 0
      dut.clockDomain.waitSamplingWhere(dut.io.btnRow.toInt == 0xfe)
      dut.io.btnCol #= 0x15555
      // row 1
      dut.clockDomain.waitSamplingWhere(dut.io.btnRow.toInt == 0xfd)
      dut.io.btnCol #= 0x2aaaa
      // row 2
      dut.clockDomain.waitSamplingWhere(dut.io.btnRow.toInt == 0xfb)
      dut.io.btnCol #= 0x15555
      // row 3
      dut.clockDomain.waitSamplingWhere(dut.io.btnRow.toInt == 0xf7)
      dut.io.btnCol #= 0x2aaaa
      // row 4
      dut.clockDomain.waitSamplingWhere(dut.io.btnRow.toInt == 0xef)
      dut.io.btnCol #= 0x15555
      // row 5
      dut.clockDomain.waitSamplingWhere(dut.io.btnRow.toInt == 0xdf)
      dut.io.btnCol #= 0x2aaaa
      // row 6
      dut.clockDomain.waitSamplingWhere(dut.io.btnRow.toInt == 0xbf)
      dut.io.btnCol #= 0x15555
      // row 7
      dut.clockDomain.waitSamplingWhere(dut.io.btnRow.toInt == 0x7f)
      dut.io.btnCol #= 0x2aaaa

      dut.clockDomain.waitSamplingWhere(dut.io.btnRow.toInt == 0xfe)
      dut.io.btnCol #= 0x3ffff

      for (col <- (0 until 4)) {
        val keyState = driver.read(dut.keyStates(col).colReg.getAddr())
        if (col % 2 == 0) {
          assert(keyState == 0xaa)
        } else {
          assert(keyState == 0x55)
        }
      }

      dut.clockDomain.waitSamplingWhere(dut.io.btnRow.toInt == 0x7f)
      dut.clockDomain.waitSamplingWhere(dut.io.btnRow.toInt == 0xfe)
      dut.io.btnCol #= 0x00000

      for (col <- (0 until 4)) {
        val keyState = driver.read(dut.keyStates(col).colReg.getAddr())
        assert(keyState == 0x00)
      }

      dut.clockDomain.waitSamplingWhere(dut.io.btnRow.toInt == 0x7f)
      dut.clockDomain.waitSamplingWhere(dut.io.btnRow.toInt == 0xfe)

      for (col <- (0 until 4)) {
        val keyState = driver.read(dut.keyStates(col).colReg.getAddr())
        assert(keyState == 0xff)
      }
    }
}
