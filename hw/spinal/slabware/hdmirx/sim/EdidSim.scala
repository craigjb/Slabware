package slabware.hdmirx.sim

import spinal.core._
import spinal.core.sim._
import spinal.lib.com.i2c.sim._

import slabware.hdmirx._

object EdidSim extends App {

  class I2cSoftMaster(
      scl: OpenDrainSoftConnection,
      sda: OpenDrainSoftConnection,
      baudPeriod: TimeNumber
  ) {
    def sendStart() = {
      sda.write(false)
      sleep(baudPeriod)
      scl.write(false)
    }

    def sendStop() = {
      sleep(baudPeriod / 2.0)
      sda.write(false)
      sleep(baudPeriod / 2.0)
      scl.write(true)
      sleep(baudPeriod)
      sda.write(true)
      sleep(baudPeriod)
    }

    def sendBit(value: Boolean) = {
      sleep(baudPeriod / 2.0)
      sda.write(value)
      sleep(baudPeriod / 2.0)
      scl.write(true)
      sleep(baudPeriod)
      sda.write(true)
      scl.write(false)
    }

    def sendByte(value: Int) = {
      for (i <- (0 until 8).reverse) {
        val bit = (1 << i) & value
        sendBit(bit != 0)
      }
    }
  }

  SimConfig
    .withConfig(
      SpinalConfig(
        defaultClockDomainFrequency = FixedFrequency(50 MHz)
      )
    )
    .withWave
    .compile(new Edid(edidBinPath = "SoundSlab.edid"))
    .doSim { dut =>
      dut.clockDomain.forkStimulus(period = (50 MHz).toTime)

      val scl = new OpenDrainInterconnect()
      val sda = new OpenDrainInterconnect()

      scl.addHard(dut.io.ddc.scl)
      sda.addHard(dut.io.ddc.sda)

      val master = new I2cSoftMaster(
        scl.newSoftConnection(),
        sda.newSoftConnection(),
        (100 kHz).toTime
      )

      dut.clockDomain.waitSampling(500 * 2)

      // test a read
      master.sendStart()
      master.sendByte(0xa1) // address (read)
      master.sendBit(true) // release for ack
      master.sendByte(0xff) // release for read
      master.sendBit(false) // ack (continue read)
      master.sendByte(0xff) // release for read
      master.sendBit(false) // ack (continue read)
      master.sendByte(0xff) // release for read
      master.sendBit(false) // ack (continue read)
      master.sendByte(0xff) // release for read
      master.sendBit(true) // nack (end read)
      master.sendStop()

      dut.clockDomain.waitSampling(500 * 10)

      // test a write
      master.sendStart()
      master.sendByte(0xa0) // address (write)
      master.sendBit(true) // release for ack
      master.sendByte(0xde) // offset
      master.sendBit(true) // release for ack
      master.sendStop()

      dut.clockDomain.waitSampling(500 * 10)

      // test a write on invalid address
      master.sendStart()
      master.sendByte(0xbf) // address (read)
      master.sendBit(true) // release for ack
      master.sendByte(0xde) // offset
      master.sendBit(true) // release for ack
      master.sendStop()

      dut.clockDomain.waitSampling(500 * 10)

      // test a read on invalid address
      dut.clockDomain.waitSampling(500 * 10)
      master.sendStart()
      master.sendByte(0xbe) // address (read)
      master.sendBit(true) // release for ack
      master.sendByte(0xff) // release for read
      master.sendBit(false) // ack (continue read)
      master.sendByte(0xff) // release for read
      master.sendBit(true) // nack (end read)
      master.sendStop()

      dut.clockDomain.waitSampling(500 * 100)
    }
}
