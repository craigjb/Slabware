package slabware.hdmirx.sim

import spinal.core._
import spinal.core.sim._
import spinal.lib._
import spinal.lib.bus.amba3.apb.Apb3
import spinal.lib.bus.amba3.apb.sim._
import spinal.lib.bus.regif.Apb3BusInterface
import slabware.hdmirx._

object ClockDetectorSim extends App {

  class ClockDetectorDut() extends Component {
    val clockDetector = ClockDetector(sampleRate = 1000 Hz)

    val io = new Bundle {
      val bus = slave(Apb3(addressWidth = 8, dataWidth = 32))
      val hdmiTmdsClk = clockDetector.io.hdmiTmdsClk.toIo
      val divisor = clockDetector.io.divisor.toIo
      val sampleRate = clockDetector.io.sampleRate.toIo
      val count = clockDetector.io.count.toIo
      val counterUpdate = clockDetector.io.counterUpdate.toIo
      val freqChanged = clockDetector.io.freqChanged.toIo
    }

    val busIf = Apb3BusInterface(io.bus, (0, 0x100), "", false)
    clockDetector.drive(busIf)
  }

  SimConfig
    .withConfig(
      SpinalConfig(defaultClockDomainFrequency = FixedFrequency(50 MHz))
    )
    .withWave
    .compile(new ClockDetectorDut())
    .doSim { dut =>
      dut.clockDomain.forkStimulus(period = (50 MHz).toTime)

      val apbDriver = new Apb3Driver(dut.io.bus, dut.clockDomain)

      def hdmiClk(freq: HertzNumber) = {
        val period = freq.toTime
        while (true) {
          dut.io.hdmiTmdsClk #= false
          sleep(period / 2.0)
          dut.io.hdmiTmdsClk #= true
          sleep(period / 2.0)
        }
      }

      val hdmiClk155_75MHz = fork(hdmiClk(155.75 MHz))

      // get clock config via APB interface while waiting
      val divisor = apbDriver.read(0x0)
      assert(divisor == 256, "Incorrect divisor read on APB interface")
      val sampleRate = apbDriver.read(0x4)
      assert(sampleRate == 1000, "Incorrect sample rate read on APB interface")

      dut.clockDomain.waitSamplingWhere(dut.io.counterUpdate.toBoolean)
      hdmiClk155_75MHz.terminate()
      dut.clockDomain.waitSampling()
      assert((608 - dut.io.count.toInt).abs <= 1, "155.75 MHz count incorrect")
      assert(dut.io.freqChanged.toBoolean)

      val hdmiClk80Mhz = fork(hdmiClk(80 MHz))

      // check via the APB interface while waiting
      assert(
        (608 - apbDriver.read(0x8)).abs <= 1,
        "155.75 MHz count read from APB interface incorrect"
      )

      dut.clockDomain.waitSamplingWhere(dut.io.counterUpdate.toBoolean)
      dut.clockDomain.waitSampling()
      assert((312 - dut.io.count.toInt).abs <= 1, "80 MHz count incorrect")
      assert(dut.io.freqChanged.toBoolean)

      assert(
        (312 - apbDriver.read(0x8)).abs <= 1,
        "80 MHz count read from APB interface incorrect"
      )

      dut.clockDomain.waitSamplingWhere(dut.io.counterUpdate.toBoolean)
      dut.clockDomain.waitSampling()
      assert((312 - dut.io.count.toInt).abs <= 1, "80 MHz count incorrect")
      assert(!dut.io.freqChanged.toBoolean)

      dut.clockDomain.waitSampling(10)
    }
}
