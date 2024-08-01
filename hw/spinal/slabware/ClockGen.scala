package slabware

import spinal.core._
import spinal.lib._

class ClockGen(
    multiplier: Double,
    divider: Int,
    clkOutDivider: Int
) extends Component {
  val io = new Bundle {
    val clkOut = out Bool ()
    val locked = out Bool ()
  }

  val clkInFreq = ClockDomain.current.frequency.getValue
  val clkInPeriodNs = clkInFreq.toTime.toBigDecimal / 1e-9

  val vcoFreq = clkInFreq * multiplier
  val clkOutFreq = FixedFrequency(vcoFreq / divider / clkOutDivider)

  val mmcm = new MmcmE2Base(
    clkInPeriodNs = clkInPeriodNs,
    clkFbOutMultiplier = multiplier,
    divClkDivider = divider,
    clkOut0Divide = clkOutDivider
  )

  mmcm.io.clkIn := ClockDomain.current.readClockWire
  mmcm.io.reset := ClockDomain.current.readResetWire
  mmcm.io.clkFbIn := mmcm.io.clkFbOut
  mmcm.io.powerDown := False

  io.clkOut := mmcm.io.clkOut0
  io.locked := mmcm.io.locked

  class MmcmE2Base(
      clkInPeriodNs: BigDecimal,
      clkFbOutMultiplier: BigDecimal = 5.0,
      divClkDivider: Int = 1,
      clkOut0Divide: BigDecimal = 1.0,
      clkOut1Divide: Int = 1,
      clkOut2Divide: Int = 1,
      clkOut3Divide: Int = 1,
      clkOut4Divide: Int = 1,
      clkOut5Divide: Int = 1
  ) extends BlackBox {

    addGeneric("CLKIN1_PERIOD", clkInPeriodNs.toDouble)
    addGeneric("CLKFBOUT_MULT_F", clkFbOutMultiplier.toDouble)
    addGeneric("DIVCLK_DIVIDE", divClkDivider)
    addGeneric("CLKOUT0_DIVIDE_F", clkOut0Divide.toDouble)
    addGeneric("CLKOUT1_DIVIDE", clkOut1Divide)
    addGeneric("CLKOUT2_DIVIDE", clkOut2Divide)
    addGeneric("CLKOUT3_DIVIDE", clkOut3Divide)
    addGeneric("CLKOUT4_DIVIDE", clkOut4Divide)
    addGeneric("CLKOUT5_DIVIDE", clkOut5Divide)

    val io = new Bundle {
      val clkIn = in Bool ()
      val clkFbIn = in Bool ()
      val reset = in Bool ()
      val powerDown = in Bool ()

      val clkOut0 = out Bool ()
      val clkOut1 = out Bool ()
      val clkOut2 = out Bool ()
      val clkOut3 = out Bool ()
      val clkOut4 = out Bool ()
      val clkOut5 = out Bool ()

      val clkFbOut = out Bool ()
      val locked = out Bool ()
    }

    setDefinitionName("MMCME2_BASE")

    io.clkIn.setName("CLKIN1")
    io.clkFbIn.setName("CLKFBIN")
    io.reset.setName("RST")
    io.powerDown.setName("PWRDWN")
    io.clkOut0.setName("CLKOUT0")
    io.clkOut1.setName("CLKOUT1")
    io.clkOut2.setName("CLKOUT2")
    io.clkOut3.setName("CLKOUT3")
    io.clkOut4.setName("CLKOUT4")
    io.clkOut5.setName("CLKOUT5")
    io.clkOut5.setName("CLKOUT5")
    io.clkFbOut.setName("CLKFBOUT")
    io.locked.setName("LOCKED")
  }
}
