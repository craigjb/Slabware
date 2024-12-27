package slabware.hdmirx.sim

import scala.util.Random

import spinal.core._
import spinal.core.sim._
import spinal.lib.bus.amba3.apb.sim._

import slabware.Apb3Bus
import slabware.hdmirx._

object HdmiRxSim extends App {
  def SimSpeedup = true
  def SkipClockDetect = true
  def RandomPixelData = false

  Random.setSeed(1337)

  SimConfig
    .withConfig(
      SpinalConfig(
        defaultClockDomainFrequency = FixedFrequency(50 MHz),
        dumpWave = DumpWaveConfig(vcdPath = "simWorkspace/HdmiRx/wave.vcd")
      )
    )
    .withXilinxDevice("xc7a200t-fbg676-2")
    .withXSim
    .compile(
      new HdmiRx(
        Apb3Bus,
        config = HdmiRxConfig(
          simSpeedup = SimSpeedup,
          edidBinPath = "SoundSlab.edid"
        )
      )
    )
    .doSim { dut =>
      dut.clockDomain.forkStimulus(period = (50 MHz).toTime)
      val HdmiTmdsClkFreq = 90.29 MHz
      val tmdsClkPeriod = HdmiTmdsClkFreq.toTime
      val bitClkPeriod = tmdsClkPeriod / 10.0

      val hdmiClk = ClockDomain(dut.io.hdmi.clk.p)
      val hdmiClkSource = fork {
        dut.io.hdmi.clk.p #= false
        dut.io.hdmi.clk.n #= true

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

      val hdmiChannelSources = (0 to 2).map(channel =>
        fork {
          dut.io.hdmi.channels(channel).p #= false
          dut.io.hdmi.channels(channel).n #= true

          val encoder = new HdmiEncoder(
            dut.io.hdmi.channels(channel),
            bitClkPeriod,
            hdmiClk
          )

          while (true) {
            for (y <- 0 until 1045) {
              channel match {
                case 0 => {
                  // front porch (64)
                  encoder.sendControl(true, y < 21, count = 12)
                  for (_ <- 0 until 52) {
                    encoder.sendData(Random.nextInt().abs % 16)
                  }

                  // H sync (-) (225)
                  encoder.sendControl(false, y < 21, count = 12)
                  for (_ <- 0 until 213) {
                    encoder.sendData(Random.nextInt().abs % 16)
                  }

                  // back porch (288)
                  encoder.sendControl(true, y < 21, count = 12)
                  for (_ <- 0 until 264) {
                    encoder.sendData(Random.nextInt().abs % 16)
                  }
                  encoder.sendControl(true, y < 21, count = 12)

                }
                case 1 => {
                  // front porch (64)
                  encoder.sendControl(true, false, count = 12)
                  for (_ <- 0 until 52) {
                    encoder.sendData(Random.nextInt().abs % 16)
                  }

                  // H sync (-) (225)
                  encoder.sendControl(true, false, count = 12)
                  for (_ <- 0 until 213) {
                    encoder.sendData(Random.nextInt().abs % 16)
                  }

                  // back porch (288)
                  encoder.sendControl(true, false, count = 12)
                  for (_ <- 0 until 264) {
                    encoder.sendData(Random.nextInt().abs % 16)
                  }
                  encoder.sendControl(true, false, count = 12)
                }
                case 2 => {
                  // front porch (64)
                  encoder.sendControl(true, false, count = 12)
                  for (_ <- 0 until 52) {
                    encoder.sendData(Random.nextInt().abs % 16)
                  }

                  // H sync (-) (225)
                  encoder.sendControl(true, false, count = 12)
                  for (_ <- 0 until 213) {
                    encoder.sendData(Random.nextInt().abs % 16)
                  }

                  // back porch (288)
                  encoder.sendControl(true, false, count = 12)
                  for (_ <- 0 until 264) {
                    encoder.sendData(Random.nextInt().abs % 16)
                  }
                  encoder.sendControl(false, false, count = 12)
                }
              }

              // video data
              for (x <- 0 until 2304) {
                if (RandomPixelData) {
                  encoder.sendPixel(Random.nextInt().abs % 256)
                } else {
                  encoder.sendPixel(x % 256)
                }
              }
            }
          }
        }
      )

      dut.io.ddc.scl.read #= true
      dut.io.ddc.sda.read #= true
      dut.io.hdmi.cableDetect #= false

      val apbDriver = new Apb3Driver(dut.io.bus, dut.clockDomain)

      dut.clockDomain.waitSampling(10)
      assert(dut.io.hdmi.hpd.toBoolean == false)

      // turn on HPD
      apbDriver.write(dut.control.controlReg.getAddr(), 0x3)
      dut.clockDomain.waitSampling()
      assert(dut.io.hdmi.hpd.toBoolean == true)
      println("Asserted HPD")

      // wait for clock detection
      if (!SkipClockDetect) {
        println("Starting clock detection")
        var prevClkCounter: BigInt = 0
        var done = false
        var tries = 10
        while (!done && tries > 0) {
          sleep(dut.clockDetector.config.sampleRate.toTime)
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
      } else {
        println("Skipping clock detection")
      }

      // power up PLL and reset
      println("Powering up and resetting PLL")
      apbDriver.write(dut.control.controlReg.getAddr(), 0xd)
      apbDriver.write(dut.control.controlReg.getAddr(), 0x9)

      // wait for PLL lock
      println("Waiting for PLL lock")
      var pllLocked = false
      while (!pllLocked) {
        sleep(5 us)
        pllLocked =
          (apbDriver.read(dut.status.statusReg.getAddr()) & (0x2)) != 0
      }
      println("PLL locked")
      dut.clockDomain.waitSampling(1)

      // reset GTP
      println("Resetting GTP")
      apbDriver.write(dut.control.controlReg.getAddr(), 0x9)
      dut.clockDomain.waitSampling(20)
      apbDriver.write(dut.control.controlReg.getAddr(), 0x1)

      // wait for GTP reset
      println("Waiting for GTP reset")
      if (!SimSpeedup) {
        println("\tThis can take awhile...")
      }
      var gtpReset = false
      var waitedUs = 0
      while (!gtpReset) {
        sleep(10 us)
        waitedUs += 10
        gtpReset = (apbDriver.read(
          dut.channels(0).channelStatus.statusReg.getAddr()
        ) & (0x1)) != 0
        println(f"Waited: ${waitedUs} µs")
      }
      println("GTP reset done")

      println(f"Simulating some scanlines...")
      for (i <- 0 until 200) {
        sleep(1 us)
        println(f"Simulated: ${i} µs")
      }
    }
}
