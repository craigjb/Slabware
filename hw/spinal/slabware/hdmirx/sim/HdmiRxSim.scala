package slabware.hdmirx.sim

import spinal.core._
import spinal.core.sim._
import spinal.lib.bus.amba3.apb.sim._

import slabware.Apb3Bus
import slabware.hdmirx._

object HdmiRxSim extends App {
  SimConfig
    .withConfig(
      SpinalConfig(
        defaultClockDomainFrequency = FixedFrequency(50 MHz),
        dumpWave = DumpWaveConfig(vcdPath = "simWorkspace/HdmiRx/wave.vcd")
      )
    )
    .withXilinxDevice("xc7a200t-fbg676-2")
    .withXSim
    .compile(new HdmiRx(Apb3Bus, edidBinPath = "SoundSlab.edid"))
    .doSim { dut =>
      dut.clockDomain.forkStimulus(period = (50 MHz).toTime)
      val HdmiTmdsClkFreq = 155.75 MHz

      val hdmiSource = fork {
        dut.io.hdmi.clk.p #= false
        dut.io.hdmi.clk.n #= true

        val tmdsClkPeriod = HdmiTmdsClkFreq.toTime
        waitUntil(dut.io.hdmi.hpd.toBoolean == true)
        sleep(tmdsClkPeriod * 10)

        while (true) {
          dut.io.hdmi.clk.p #= true
          dut.io.hdmi.clk.n #= false
          sleep(tmdsClkPeriod / 2.0)
          dut.io.hdmi.clk.p #= false
          dut.io.hdmi.clk.n #= true
          sleep(tmdsClkPeriod / 2.0)
        }
      }

      dut.io.hdmi.ddc.scl.read #= true
      dut.io.hdmi.ddc.sda.read #= true
      dut.io.hdmi.cableDetect #= false

      val apbDriver = new Apb3Driver(dut.io.bus, dut.clockDomain)

      dut.clockDomain.waitSampling(10)
      assert(dut.io.hdmi.hpd.toBoolean == false)

      // turn on HPD
      apbDriver.write(dut.control.controlReg.getAddr(), 0x3)
      dut.clockDomain.waitSampling()
      assert(dut.io.hdmi.hpd.toBoolean == true)

      // wait for clock detection
      var prevClkCounter: BigInt = 0
      var done = false
      var tries = 10
      while (!done && tries > 0) {
        sleep(dut.clockDetector.sampleRate.toTime)
        val counterValue =
          apbDriver.read(dut.clockDetectorBusIf.countReg.getAddr())
        if (counterValue != 0) {
          if (
            prevClkCounter != 0 &&
            (counterValue - prevClkCounter).abs <= 1
          ) {
            done = true
          }
          prevClkCounter = counterValue
        }
        tries -= 1
      }
      assert(done, "Clock detector did not stabilize")
      println(f"Clock detector stable at: ${prevClkCounter}")
      dut.clockDomain.waitSampling()

      // power up PLL and reset
      apbDriver.write(dut.control.controlReg.getAddr(), 0x5)
      apbDriver.write(dut.control.controlReg.getAddr(), 0x1)

      // wait for PLL lock
      var pllLocked = false
      while (!pllLocked) {
        sleep(5 us)
        pllLocked =
          (apbDriver.read(dut.status.statusReg.getAddr()) & (0x2)) != 0
      }

      dut.clockDomain.waitSampling(10)
    }
}
