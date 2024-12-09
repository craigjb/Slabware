package slabware.hdmirx

import spinal.core._
import spinal.lib._

object IBufDsGte2 {
  def apply(clkIn: HdmiClk, clkOut: Bool, enable: Bool) = {
    val buf = new IBufDsGte2()
    buf.io.I := clkIn.p
    buf.io.IB := clkIn.n
    buf.io.CEB := !enable
    clkOut := buf.io.O
    buf
  }
}

class IBufDsGte2() extends BlackBox {
  val io = new Bundle {
    val I = in Bool ()
    val IB = in Bool ()
    val CEB = in Bool ()
    val O = out Bool ()
    val ODIV2 = out Bool ()
  }

  noIoPrefix()
  setBlackBoxName("IBUFDS_GTE2")
}

object Gtpe2PllRefClk {
  def GtRefClk0 = B"001"
  def GtRefClk1 = B"010"
  def GtEastRefClk0 = B"011"
  def GtEastRefClk1 = B"100"
  def GtWestRefClk0 = B"101"
  def GtWestRefClk1 = B"110"
}

case class Gtp2ePllIo() extends Bundle {
  val outClk = out Bool ()
  val outRefClk = out Bool ()

  val lock = out Bool ()
  val lockEnable = in Bool () default (True)
  val powerDown = in Bool ()
  val refClkSelect = in Bits (3 bits)
  val reset = in Bool ()

  val lockDetectClk = in Bool ()
  val refClkLost = out Bool ()
  val fbClkLost = out Bool ()

  def forPllIndex(i: Int) = {
    assert((0 to 1).contains(i), "Must be PLL0 or PLL1")
    outClk.setName(f"PLL${i}OUTCLK")
    outRefClk.setName(f"PLL${i}OUTREFCLK")
    lock.setName(f"PLL${i}LOCK")
    lockEnable.setName(f"PLL${i}LOCKEN")
    powerDown.setName(f"PLL${i}PD")
    refClkSelect.setName(f"PLL${i}REFCLKSEL")
    reset.setName(f"PLL${i}RESET")
    lockDetectClk.setName(f"PLL${i}LOCKDETCLK")
    refClkLost.setName(f"PLL${i}REFCLKLOST")
    fbClkLost.setName(f"PLL${i}FBCLKLOST")
    this
  }

  def disable() = {
    powerDown := True
    refClkSelect := Gtpe2PllRefClk.GtRefClk0
    reset := False
    lockDetectClk := False
  }
}

object Gtpe2PllConfig {
  def default() = Gtpe2PllConfig(
    refClkDiv = 1,
    fbDiv = 1,
    fbDiv45 = 4,
    simRefClkSelect = Gtpe2PllRefClk.GtRefClk0
  )
}

case class Gtpe2PllConfig(
    refClkDiv: Int,
    fbDiv: Int,
    fbDiv45: Int,
    simRefClkSelect: Bits
) {
  assert((1 to 2).contains(refClkDiv), "refClkDiv must be 1 or 2")
  assert((1 to 5).contains(fbDiv), "fbDiv must be within 1-5")
  assert((4 to 5).contains(fbDiv45), "fbDiv45 must be 4 or 5")
}

class Gtpe2Common(
    pll0Config: Gtpe2PllConfig,
    pll1Config: Gtpe2PllConfig
) extends BlackBox {
  val generic = new Generic {
    val SIM_RESET_SPEEDUP = "FALSE"
    val SIM_PLL0REFCLK_SEL = pll0Config.simRefClkSelect
    val SIM_PLL1REFCLK_SEL = pll1Config.simRefClkSelect
    val SIM_VERSION = "2.0"

    val BIAS_CFG = B"64'h0000000000050001"
    val COMMON_CFG = B"32'h00000000"

    val PLL0_REFCLK_DIV = pll0Config.refClkDiv
    val PLL0_FBDIV = pll0Config.fbDiv
    val PLL0_FBDIV_45 = pll0Config.fbDiv45
    val PLL0_CFG = B"27'h01F03DC"
    val PLL0_LOCK_CFG = B"9'h1E8"
    val PLL0_INIT_CFG = B"24'h00001E"
    val PLL0_DMON_CFG = B"1'b0"

    val PLL1_REFCLK_DIV = pll1Config.refClkDiv
    val PLL1_FBDIV = pll1Config.fbDiv
    val PLL1_FBDIV_45 = pll1Config.fbDiv45
    val PLL1_CFG = B"27'h01F03DC"
    val PLL1_LOCK_CFG = B"9'h1E8"
    val PLL1_INIT_CFG = B"24'h00001E"
    val PLL1_DMON_CFG = B"1'b0"

    val PLL_CLKOUT_CFG = B"8'0"
    val RSVD_ATTR0 = B"16'0"
    val RSVD_ATTR1 = B"16'0"
  }

  val io = new Bundle {
    val drp = new Bundle {
      val clk = in Bool () setName ("DRPCLK")
      val addr = in UInt (8 bits) setName ("DRPADDR")
      val dataIn = in Bits (16 bits) setName ("DRPDI")
      val dataOut = out Bits (16 bits) setName ("DRPDO")
      val enable = in Bool () setName ("DRPEN")
      val writeEnable = in Bool () setName ("DRPWE")
      val ready = out Bool () setName ("DRPRDY")

      def disable() = {
        clk := False
        addr := 0
        dataIn := B"16'0"
        enable := False
        writeEnable := False
      }
    }

    val clocking = new Bundle {
      val gtEastRefClk0 = in Bool () setName ("GTEASTREFCLK0")
      val gtEastRefClk1 = in Bool () setName ("GTEASTREFCLK1")
      val gtWestRefClk0 = in Bool () setName ("GTWESTREFCLK0")
      val gtWestRefClk1 = in Bool () setName ("GTWESTREFCLK1")
      val gtRefClk0 = in Bool () setName ("GTREFCLK0")
      val gtRefClk1 = in Bool () setName ("GTREFCLK1")

      val internalGtgRefClk0 = in Bool () setName ("GTGREFCLK0") default (False)
      val internalGtgRefClk1 = in Bool () setName ("GTGREFCLK1") default (False)

      def disableEastWest() = {
        gtEastRefClk0 := False
        gtEastRefClk1 := False
        gtWestRefClk0 := False
        gtWestRefClk1 := False
      }
    }

    val pll0 = Gtp2ePllIo().forPllIndex(0)
    val pll1 = Gtp2ePllIo().forPllIndex(1)

    val digitalMonitorOut = out Bits (8 bits) setName ("DMONITOROUT")
    val refClkOutMonitor0 = out Bool () setName ("REFCLKOUTMONITOR0")
    val refClkOutMonitor1 = out Bool () setName ("REFCLKOUTMONITOR1")

    val reserved = new Bundle {
      val bgBypassB = in Bool () setName ("BGBYPASSB") default (True)
      val bgMonitorEnB = in Bool () setName ("BGMONITORENB") default (True)
      val bgPDB = in Bool () setName ("BGPDB") default (True)
      val bgRCalOvrd =
        in Bits (5 bits) setName ("BGRCALOVRD") default (B"11111")
      val bgRCalOvrdEnB = in Bool () setName ("BGRCALOVRDENB") default (True)
      val rCalEnB = in Bool () setName ("RCALENB") default (True)
      val pllRsvd1 = in Bits (16 bits) setName ("PLLRSVD1") default (B"16'0")
      val pllRsvd2 = in Bits (5 bits) setName ("PLLRSVD2") default (B"5'0")
      val pmaRsvd = in Bits (8 bits) setName ("PMARSVD") default (B"8'0")
      val pmaRsvdOut = out Bits (16 bits) setName ("PMARSVDOUT")
    }
  }

  noIoPrefix()
  setBlackBoxName("GTPE2_COMMON")
}
