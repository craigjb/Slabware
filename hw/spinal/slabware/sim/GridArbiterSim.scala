package slabware.sim

import scala.util.Random

import spinal.core._
import spinal.core.sim._

import slabware.GridArbiter

object GridArbiterSim extends App {
  Random.setSeed(0xdeadbeef)

  val BroadcastSequence = Seq(
    0x13a, 0x05, 0x136, 0xd0, 0x121, 0x111, 0x129
  )

  SimConfig
    .withConfig(
      SpinalConfig(defaultClockDomainFrequency = FixedFrequency(66.666 MHz))
    )
    .withWave
    .compile(
      new GridArbiter(videoClkDomain = ClockDomain.external("videoClk"))
    )
    .doSim { dut =>
      dut.clockDomain.forkStimulus()
      dut.videoClkDomain.forkStimulus(45 MHz)

      // HDMI video source
      fork {
        def setPixel(i: Int, r: Int, g: Int, b: Int, valid: Boolean = false) = {
          dut.io.videoIn.redPixels(i) #= r
          dut.io.videoIn.greenPixels(i) #= g
          dut.io.videoIn.bluePixels(i) #= b
          dut.io.videoIn.pixelsValid(i) #= valid
        }

        def setSync(vSync: Boolean, hSync: Boolean) = {
          dut.io.videoIn.vSync #= !vSync
          dut.io.videoIn.hSync #= hSync
        }

        def blanking(pixels: Int, hSync: Boolean, vSync: Boolean) {
          assert(pixels % 2 == 0)
          setSync(vSync, hSync)
          setPixel(0, r = 0xff, g = 0xff, b = 0xff)
          setPixel(1, r = 0xff, g = 0xff, b = 0xff)
          dut.videoClkDomain.waitSampling(pixels / 2)
        }

        def videoData(
            pixels: Int,
            vSync: Boolean,
            random: Boolean = false,
            offset: Boolean = false
        ) = {
          assert(pixels % 2 == 0)
          setSync(vSync, false)

          def videoPixel(i: Int, n: Int) = {
            val p = if (random) {
              Random.nextInt().abs
            } else {
              n
            }
            setPixel(i, r = p % 16, g = p % 32, b = p % 16, valid = true)
          }

          val pOffset = if (offset) { 1 }
          else { 0 }
          if (offset) {
            setPixel(0, r = 0xff, g = 0xff, b = 0xff)
            videoPixel(1, 0)
            dut.videoClkDomain.waitSampling()
          }

          for (i <- 0 until ((pixels / 2) - pOffset)) {
            videoPixel(0, (i * 2) + pOffset)
            videoPixel(1, (i * 2) + 1 + pOffset)
            dut.videoClkDomain.waitSampling()
          }

          if (offset) {
            videoPixel(0, pixels - 1)
            setPixel(1, r = 0xff, g = 0xff, b = 0xff)
            dut.videoClkDomain.waitSampling()
          }

        }

        dut.io.videoIn.valid #= false
        setSync(false, false)
        setPixel(0, r = 0, g = 0, b = 0)
        setPixel(1, r = 0, g = 0, b = 0)

        dut.videoClkDomain.waitSampling(100)
        dut.io.videoIn.valid #= true

        // vSync line
        blanking(64, hSync = false, vSync = true)
        blanking(224, hSync = true, vSync = true)
        blanking(288, hSync = false, vSync = true)
        videoData(2304, vSync = true, random = true)

        // data line (no offset)
        blanking(64, hSync = false, vSync = false)
        blanking(224, hSync = true, vSync = false)
        blanking(288, hSync = false, vSync = false)
        videoData(2304, vSync = false)

        // data line (with offset)
        blanking(64, hSync = false, vSync = false)
        blanking(224, hSync = true, vSync = false)
        blanking(288, hSync = false, vSync = false)
        videoData(2304, vSync = false, offset = true)

      }

      sleep(100 us)
    }
}
