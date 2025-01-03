package slabware.sim

import spinal.core._
import spinal.core.sim._
import spinal.lib.bus.amba3.apb.sim._

import slabware.{UsbDeviceCtrl, Apb3Bus}
import spinal.lib.com.usb.udc.UsbDeviceCtrlParameter

object UsbCtrlSim extends App {
  SimConfig.withWave
    .compile(
      new UsbDeviceCtrl(Apb3Bus, UsbDeviceCtrlParameter(addressWidth = 14))
    )
    .doSim { dut =>
      dut.clockDomain.forkStimulus(period = 10)
      dut.clockDomain.waitSampling(10)

      val driver = new Apb3Driver(dut.io.bus, dut.clockDomain)
      driver.write(0x0, 0xdeadbeefL)
      assert(driver.read(0x0) == 0xdeadbeefL)
      assert(driver.read(0x0) == 0xdeadbeefL)

      driver.write(0x0, 0xbeefdeadL)
      assert(driver.read(0x0) == 0xbeefdeadL)
      assert(driver.read(0x0) == 0xbeefdeadL)

      driver.write(0x3fff, 0xdeadbeefL)
      assert(driver.read(0x3fff) == 0xdeadbeefL)
      assert(driver.read(0x3fff) == 0xdeadbeefL)

      driver.write(0x3fff, 0xbeefdeadL)
      assert(driver.read(0x3fff) == 0xbeefdeadL)
      assert(driver.read(0x3fff) == 0xbeefdeadL)
    }
}
