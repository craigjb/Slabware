package slabware

import spinal.core._
import spinal.lib._
import spinal.lib.bus.amba4.axi._
import spinal.lib.io._
import spinal.lib.blackbox.xilinx.s7.IOBUF

import slabware.hdmirx.{HdmiIo, DiffPair}

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

    // HDMI DDC
    val HDMI_RX_SDA = inout(Analog(Bool))
    val HDMI_RX_SCL = inout(Analog(Bool))

    // HDMI input
    val hdmi = slave(HdmiIo(invertD0 = true))
    hdmi.clk.p.setName("HDMI_CLK_P")
    hdmi.clk.n.setName("HDMI_CLK_N")
    hdmi.channel0.setName("HDMI_D0_P")
    hdmi.channel0.setName("HDMI_D0_N")
    hdmi.hpd.setName("HDMI_RX_HPD")
    hdmi.cableDetect.setName("HDMI_RX_PWR_DET")
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
    slabControl.io.hdmi <> io.hdmi
    io.LED := slabControl.io.leds
  }

  val hdmiCtlI2cIo = new Area {
    io.HDMI_CTL_SCL := OpenDrainBuffer(
      spiClockArea.slabControl.io.hdmiCtrlI2c.scl
    )
    io.HDMI_CTL_SDA := OpenDrainBuffer(
      spiClockArea.slabControl.io.hdmiCtrlI2c.sda
    )
  }

  val ddcI2cIo = new Area {
    io.HDMI_RX_SCL := OpenDrainBuffer(
      spiClockArea.slabControl.io.ddc.scl
    )
    io.HDMI_RX_SDA := OpenDrainBuffer(
      spiClockArea.slabControl.io.ddc.sda
    )
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
