package slabware

import spinal.core._
import spinal.lib._
import spinal.lib.bus.amba4.axi._
import spinal.lib.io._
import spinal.lib.blackbox.xilinx.s7.IOBUF

class Slabware(
    numLcdDims: Int = 9,
    numSpiClusters: Int = 18
) extends Component {

  val io = new Bundle {
    // 100 MHz clock input
    val SYSCLK = in Bool ()

    // 8 Debug LEDs
    val LED = out Bits (8 bits)

    // Backlight PWM lines
    val DIM = out Bits (numLcdDims bits)

    // LCD SPI interfaces
    val RESET = out Bool ()
    val SDA = out Bits (numSpiClusters bits)
    val SCL = out Bits (numSpiClusters bits)
    val DC = out Bits (numSpiClusters bits)
    val DSA = out Bits (numSpiClusters bits)
    val DSB = out Bits (numSpiClusters bits)

    // Button matrix
    // val BTNCOL = out Bits (18 bits)
    // val BTNROW = in Bits (8 bits)

    // TMDS181 I2c
    val HDMI_CTL_SDA = inout(Analog(Bool))
    val HDMI_CTL_SCL = inout(Analog(Bool))
  }
  noIoPrefix()

  val sysClockDomain = ClockDomain(
    clock = io.SYSCLK,
    config = ClockDomainConfig(
      clockEdge = RISING,
      resetKind = BOOT,
      resetActiveLevel = HIGH
    ),
    frequency = FixedFrequency(HertzNumber(100e6))
  )

  val sysClockArea = new ClockingArea(sysClockDomain) {
    val clockGen = new ClockGen(
      multiplier = 10.0,
      divider = 1,
      clkOutDivider = 20
    )
  }

  val spiClockDomain = ClockDomain(
    clock = sysClockArea.clockGen.io.clkOut,
    reset = sysClockArea.clockGen.io.locked,
    config = ClockDomainConfig(
      clockEdge = RISING,
      resetKind = ASYNC,
      resetActiveLevel = LOW
    ),
    frequency = sysClockArea.clockGen.clkOutFreq
  )

  val spiClockArea = new ClockingArea(spiClockDomain) {
    val leds = RegInit(B"8'x55")
    val counter = Counter(100000000)

    counter.increment()
    when(counter.willOverflow) {
      leds := ~leds
    }

    val backlightEnable = False
    // val backlightEnable = True
    val lcdDims = Range(0, numLcdDims)
      .map(index =>
        LcdDim(
          enable = backlightEnable,
          pwmOut = io.DIM(index)
        )
      )

    val grid = SlabGrid(
      lcdReset = io.RESET,
      scl = io.SCL,
      sda = io.SDA,
      dc = io.DC,
      dsa = io.DSA,
      dsb = io.DSB,
      numSpiClusters = numSpiClusters
    )

    val slabControl = new SlabControl()
  }

  io.LED := spiClockArea.slabControl.io.leds

  val i2cIo = new Area {
    io.HDMI_CTL_SCL := OpenDrainBuffer(spiClockArea.slabControl.io.i2c.scl)
    io.HDMI_CTL_SDA := OpenDrainBuffer(spiClockArea.slabControl.io.i2c.sda)
  }
}

object TopLevelVerilog {
  def main(args: Array[String]): Unit = {
    SpinalConfig(
      inlineRom = true
    ).generateVerilog(
      new Slabware(
        numLcdDims = 9,
        numSpiClusters = 36
      )
    )
  }
}
