package slabware.sim

import spinal.core._
import spinal.core.sim._

import slabware.LcdSpi

object LcdSpiSim extends App {
  SimConfig.withWave.compile(new LcdSpi).doSim { dut =>
    dut.clockDomain.forkStimulus(period = 10)

    dut.io.input.payload.isCmd #= true
    dut.io.input.payload.data #= 0xa9
    dut.io.input.valid #= true
    dut.clockDomain.waitSampling()
    dut.io.input.valid #= false

    dut.clockDomain.waitSampling(3)
    dut.io.input.payload.isCmd #= false
    dut.io.input.payload.data #= 0x55
    dut.io.input.valid #= true
    dut.clockDomain.waitRisingEdgeWhere(dut.io.input.ready.toBoolean)
    dut.io.input.valid #= false

    dut.clockDomain.waitRisingEdgeWhere(dut.io.input.ready.toBoolean)
    dut.clockDomain.waitSampling(3)
    dut.io.input.payload.isCmd #= false
    dut.io.input.payload.data #= 0x77
    dut.io.input.valid #= true
    dut.clockDomain.waitRisingEdgeWhere(dut.io.input.ready.toBoolean)
    dut.io.input.valid #= false

    dut.clockDomain.waitRisingEdgeWhere(dut.io.input.ready.toBoolean)
    dut.clockDomain.waitSampling()
  }
}
