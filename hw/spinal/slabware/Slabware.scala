package slabware

import spinal.core._
import spinal.lib._
import spinal.lib.bus.amba4.axi._
import spinal.lib.io._
import spinal.lib.blackbox.xilinx.s7.IOBUF
import spinal.lib.eda.xilinx.VivadoConstraintWriter
import spinal.lib.com.usb.phy.UsbDevicePhyNative

import slabware.hdmirx.{HdmiIo, DiffPair}

class Slabware(
    firmwareBinPath: String = null,
    numLcdDims: Int = 9,
    numSpiClusters: Int = 36
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
    val BTNROW = out Bits (8 bits)
    val BTNCOL = in Bits (18 bits)

    // TMDS181 I2c
    val HDMI_CTL_SDA = inout(Analog(Bool))
    val HDMI_CTL_SCL = inout(Analog(Bool))

    // HDMI DDC
    val HDMI_RX_SDA = inout(Analog(Bool))
    val HDMI_RX_SCL = inout(Analog(Bool))

    // HDMI input
    val hdmi = slave(HdmiIo())
    hdmi.clk.p.setName("HDMI_CLK_P")
    hdmi.clk.n.setName("HDMI_CLK_N")
    hdmi.channels(0).p.setName("HDMI_D0_P")
    hdmi.channels(0).n.setName("HDMI_D0_N")
    hdmi.channels(1).p.setName("HDMI_D1_P")
    hdmi.channels(1).n.setName("HDMI_D1_N")
    hdmi.channels(2).p.setName("HDMI_D2_P")
    hdmi.channels(2).n.setName("HDMI_D2_N")
    hdmi.hpd.setName("HDMI_RX_HPD")
    hdmi.cableDetect.setName("HDMI_RX_PWR_DET")

    // USB
    val usbDP = master(TriState(Bool())) setName ("USB_D_P")
    val usbDM = master(TriState(Bool())) setName ("USB_D_N")
    val usbPullUp = out Bool () setName ("USB_PULLUP")
    val usbPower = in Bool () setName ("USB_PWR_DET")

    // Debug
    val DBG_UART_TX = out Bool ()
    val DBG_UART_RX = out Bool ()
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
      multiplier = 12.0,
      divider = 1,
      clkOutDivider0 = 18,
      clkOutDivider1 = 25
    )
  }

  val spiCD = ClockDomain(
    clock = sysClockArea.clockGen.io.clkOut0,
    reset = sysClockArea.clockGen.io.locked,
    config = ClockDomainConfig(
      clockEdge = RISING,
      resetKind = ASYNC,
      resetActiveLevel = LOW
    ),
    frequency = sysClockArea.clockGen.clkOut0Freq
  )

  val usbPhyCD = ClockDomain(
    clock = sysClockArea.clockGen.io.clkOut1,
    reset = sysClockArea.clockGen.io.locked,
    config = ClockDomainConfig(
      clockEdge = RISING,
      resetKind = ASYNC,
      resetActiveLevel = LOW
    ),
    frequency = sysClockArea.clockGen.clkOut1Freq
  )

  val spiClockArea = new ClockingArea(spiCD) {
    val slabControl = new SlabControl(
      firmwareBinPath = firmwareBinPath
    )
    slabControl.io.hdmi <> io.hdmi
    slabControl.io.btnCol := io.BTNCOL
    io.BTNROW := slabControl.io.btnRow
    io.LED := slabControl.io.leds
    io.DIM.setAllTo(slabControl.io.lcdPwmOut)

    val gridResetArea = new ResetArea(!slabControl.io.gridEnable, true) {
      val grid = SlabGrid(
        videoClkDomain = slabControl.videoClkDomain,
        numSpiClusters = numSpiClusters,
        lcdReset = io.RESET,
        scl = io.SCL,
        sda = io.SDA,
        dc = io.DC,
        dsa = io.DSA,
        dsb = io.DSB
      )
    }
    slabControl.io.videoOut >> gridResetArea.grid.io.videoIn
  }

  val usbIo = new ClockingArea(usbPhyCD) {
    import spiClockArea.slabControl
    val usbPhy = UsbDevicePhyNative(sim = false)
    slabControl.io.usbPhy.cc(spiCD, usbPhyCD) <> usbPhy.io.ctrl

    val nativeIo = usbPhyCD on usbPhy.io.usb.toNativeIo()
    when(!usbPhy.io.pullup) {
      nativeIo.dp.writeEnable := True
      nativeIo.dm.writeEnable := True
      nativeIo.dp.write := False
      nativeIo.dm.write := False
    }
    val diff = usbPhyCD on nativeIo.bufferized()

    io.usbDP <> diff.dp
    io.usbDM <> diff.dm
    usbPhy.io.power := !io.usbPower
    io.usbPullUp := usbPhy.io.pullup

    io.DBG_UART_TX := usbPhy.io.usb.tx.data
    io.DBG_UART_RX := sysClockArea.clockGen.io.locked
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
    val spinalReport = SpinalConfig(
      inlineRom = true
    ).generateVerilog(
      InOutWrapper(
        new Slabware(
          // firmwareBinPath = "fw/slabware/target/slabware.bin"
        )
      )
    )

    VivadoConstraintWriter(spinalReport, "data/spinal.xdc")
  }
}
