package slabware.hdmirx

import spinal.core._
import spinal.lib._

import spinal.lib.blackbox.xilinx.s7.BUFG
import spinal.lib.fsm._

case class DiffPair() extends Bundle {
  val p = Bool()
  val n = Bool()
}

object IBufDsGte2 {
  def apply(clkIn: DiffPair, clkOut: Bool, enable: Bool) = {
    val buf = new IBufDsGte2()
    buf.io.I := clkIn.p
    buf.io.IB := clkIn.n
    buf.io.CEB := !enable
    clkOut := BUFG.on(buf.io.O)
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
    pll1Config: Gtpe2PllConfig,
    simResetSpeedup: Boolean = false
) extends BlackBox {
  val generic = new Generic {
    // Simulation
    val SIM_RESET_SPEEDUP = if (simResetSpeedup) "TRUE" else "FALSE"
    val SIM_PLL0REFCLK_SEL = pll0Config.simRefClkSelect
    val SIM_PLL1REFCLK_SEL = pll1Config.simRefClkSelect
    val SIM_VERSION = "2.0"

    // Common configs
    val BIAS_CFG = B"64'h0000000000050001"
    val COMMON_CFG = B"32'h00000000"

    // PLL0
    val PLL0_REFCLK_DIV = pll0Config.refClkDiv
    val PLL0_FBDIV = pll0Config.fbDiv
    val PLL0_FBDIV_45 = pll0Config.fbDiv45
    val PLL0_CFG = B"27'h01F03DC"
    val PLL0_LOCK_CFG = B"9'h1E8"
    val PLL0_INIT_CFG = B"24'h00001E"
    val PLL0_DMON_CFG = B"1'b0"

    // PLL1
    val PLL1_REFCLK_DIV = pll1Config.refClkDiv
    val PLL1_FBDIV = pll1Config.fbDiv
    val PLL1_FBDIV_45 = pll1Config.fbDiv45
    val PLL1_CFG = B"27'h01F03DC"
    val PLL1_LOCK_CFG = B"9'h1E8"
    val PLL1_INIT_CFG = B"24'h00001E"
    val PLL1_DMON_CFG = B"1'b0"

    // Reserved
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

case class Gtp2eChannelClocking() extends Bundle {
  val pll0Clk = Bool()
  val pll0RefClk = Bool()
  val pll1Clk = Bool()
  val pll1RefClk = Bool()

  def fromGtpe2Common(common: Gtpe2Common) = {
    pll0Clk := common.io.pll0.outClk
    pll0RefClk := common.io.pll0.outRefClk
    pll1Clk := common.io.pll1.outClk
    pll1RefClk := common.io.pll1.outRefClk
  }
}

case class Gtp2eChannelDrp() extends Bundle with IMasterSlave {
  val clk = in Bool () setName ("DRPCLK")
  val addr = in UInt (9 bits) setName ("DRPADDR")
  val dataIn = in Bits (16 bits) setName ("DRPDI")
  val dataOut = out Bits (16 bits) setName ("DRPDO")
  val enable = in Bool () setName ("DRPEN")
  val writeEnable = in Bool () setName ("DRPWE")
  val ready = out Bool () setName ("DRPRDY")

  override def asMaster() = {
    out(clk, addr, dataIn, enable, writeEnable)
    in(dataOut, ready)
  }

  def disable() = {
    clk.default(False)
    addr := 0
    dataIn := B"16'0"
    enable := False
    writeEnable := False
  }
}

class Gtpe2Channel(
    simResetSpeedup: Boolean = false,
    drpClkDomain: ClockDomain = null,
    rxUsrClkDomain: ClockDomain = null,
    rxUsrClk2Domain: ClockDomain = null,
    rxDataWidth: Int = 20,
    rxOutDivider: Int = 4,
    commaEnable: String = "10'b1111111111",
    mCommaValue: String = "10'b1010000011",
    pCommaValue: String = "10'b0101111100"
) extends BlackBox {
  val generic = new Generic {
    // Simulation
    val SIM_RECEIVER_DETECT_PASS = "TRUE"
    val SIM_TX_EIDLE_DRIVE_LEVEL = "X"
    val SIM_RESET_SPEEDUP = if (simResetSpeedup) "TRUE" else "FALSE"
    val SIM_VERSION = "2.0"

    // RX Byte and Word Alignment
    val ALIGN_COMMA_DOUBLE = "FALSE"
    val ALIGN_COMMA_ENABLE = B(commaEnable)
    val ALIGN_COMMA_WORD = 1
    val ALIGN_MCOMMA_DET = "TRUE"
    val ALIGN_MCOMMA_VALUE = B(mCommaValue)
    val ALIGN_PCOMMA_DET = "TRUE"
    val ALIGN_PCOMMA_VALUE = B(pCommaValue)
    val SHOW_REALIGN_COMMA = "TRUE"
    val RXSLIDE_AUTO_WAIT = 7
    val RXSLIDE_MODE = "OFF"
    val RX_SIG_VALID_DLY = 10

    // RX 8B/10B Decoder
    val RX_DISPERR_SEQ_MATCH = "FALSE"
    val DEC_MCOMMA_DETECT = "FALSE"
    val DEC_PCOMMA_DETECT = "FALSE"
    val DEC_VALID_COMMA_ONLY = "FALSE"

    // RX Clock Correction
    val CBCC_DATA_SOURCE_SEL = "ENCODED"
    val CLK_COR_SEQ_2_USE = "FALSE"
    val CLK_COR_KEEP_IDLE = "FALSE"
    val CLK_COR_MAX_LAT = 9
    val CLK_COR_MIN_LAT = 7
    val CLK_COR_PRECEDENCE = "TRUE"
    val CLK_COR_REPEAT_WAIT = 0
    val CLK_COR_SEQ_LEN = 1
    val CLK_COR_SEQ_1_ENABLE = B"4'b1111"
    val CLK_COR_SEQ_1_1 = B"10'b0100000000"
    val CLK_COR_SEQ_1_2 = B"10'b0000000000"
    val CLK_COR_SEQ_1_3 = B"10'b0000000000"
    val CLK_COR_SEQ_1_4 = B"10'b0000000000"
    val CLK_CORRECT_USE = "FALSE"
    val CLK_COR_SEQ_2_ENABLE = B"4'b1111"
    val CLK_COR_SEQ_2_1 = B"10'b0100000000"
    val CLK_COR_SEQ_2_2 = B"10'b0000000000"
    val CLK_COR_SEQ_2_3 = B"10'b0000000000"
    val CLK_COR_SEQ_2_4 = B"10'b0000000000"

    // RX Channel Bonding
    val CHAN_BOND_KEEP_ALIGN = "FALSE"
    val CHAN_BOND_MAX_SKEW = 1
    val CHAN_BOND_SEQ_LEN = 1
    val CHAN_BOND_SEQ_1_1 = B"10'b0000000000"
    val CHAN_BOND_SEQ_1_2 = B"10'b0000000000"
    val CHAN_BOND_SEQ_1_3 = B"10'b0000000000"
    val CHAN_BOND_SEQ_1_4 = B"10'b0000000000"
    val CHAN_BOND_SEQ_1_ENABLE = B"4'b1111"
    val CHAN_BOND_SEQ_2_1 = B"10'b0000000000"
    val CHAN_BOND_SEQ_2_2 = B"10'b0000000000"
    val CHAN_BOND_SEQ_2_3 = B"10'b0000000000"
    val CHAN_BOND_SEQ_2_4 = B"10'b0000000000"
    val CHAN_BOND_SEQ_2_ENABLE = B"4'b1111"
    val CHAN_BOND_SEQ_2_USE = "FALSE"
    val FTS_DESKEW_SEQ_ENABLE = B"4'b1111"
    val FTS_LANE_DESKEW_CFG = B"4'b1111"
    val FTS_LANE_DESKEW_EN = "FALSE"

    // RX Margin Analysis
    val ES_CONTROL = B"6'b000000"
    val ES_ERRDET_EN = "FALSE"
    val ES_EYE_SCAN_EN = "FALSE"
    val ES_HORZ_OFFSET = B"12'h010"
    val ES_PMA_CFG = B"10'b0000000000"
    val ES_PRESCALE = B"5'b00000"
    val ES_QUALIFIER = B"80'h00000000000000000000"
    val ES_QUAL_MASK = B"80'h00000000000000000000"
    val ES_SDATA_MASK = B"80'h00000000000000000000"
    val ES_VERT_OFFSET = B"9'b000000000"

    // FPGA RX Interface
    assert(
      Seq(16, 20, 32, 40).contains(rxDataWidth),
      "rxDataWidth must be 16, 20, 32, or 40"
    )
    val RX_DATA_WIDTH = rxDataWidth

    // PMA
    val OUTREFCLK_SEL_INV = B"2'b11"
    val PMA_RSV = B"32'h00000333"
    val PMA_RSV2 = B"32'h00002040"
    val PMA_RSV3 = B"2'b00"
    val PMA_RSV4 = B"4'b0000"
    val RX_BIAS_CFG = B"16'b0000111100110011"
    val DMONITOR_CFG = B"24'h000A00"
    val RX_CM_SEL = B"2'b11"
    val RX_CM_TRIM = B"4'b1010"
    val RX_DEBUG_CFG = B"14'b00000000000000"
    val RX_OS_CFG = B"13'b0000010000000"
    val TERM_RCAL_CFG = B"15'b100001000010000"
    val TERM_RCAL_OVRD = B"3'b000"
    val TST_RSV = B"32'h00000000"
    val RX_CLK25_DIV = 12
    val TX_CLK25_DIV = 12
    val UCODEER_CLR = B"1'b0"

    // PCI Express
    val PCS_PCIE_EN = "FALSE"

    // PCS
    val PCS_RSVD_ATTR = B"48'h000000000000"

    // RX Buffer
    val RXBUF_ADDR_MODE = "FAST"
    val RXBUF_EIDLE_HI_CNT = B"4'b1000"
    val RXBUF_EIDLE_LO_CNT = B"4'b0000"
    val RXBUF_EN = "TRUE"
    val RX_BUFFER_CFG = B"6'b000000"
    val RXBUF_RESET_ON_CB_CHANGE = "TRUE"
    val RXBUF_RESET_ON_COMMAALIGN = "FALSE"
    val RXBUF_RESET_ON_EIDLE = "FALSE"
    val RXBUF_RESET_ON_RATE_CHANGE = "TRUE"
    val RXBUFRESET_TIME = B"5'b00001"
    val RXBUF_THRESH_OVFLW = 61
    val RXBUF_THRESH_OVRD = "FALSE"
    val RXBUF_THRESH_UNDFLW = 4
    val RXDLY_CFG = B"16'h001F"
    val RXDLY_LCFG = B"9'h030"
    val RXDLY_TAP_CFG = B"16'h0000"
    val RXPH_CFG = B"24'hC00002"
    val RXPHDLY_CFG = B"24'h084020"
    val RXPH_MONITOR_SEL = B"5'b00000"
    val RX_XCLK_SEL = "RXREC"
    val RX_DDI_SEL = B"6'b000000"
    val RX_DEFER_RESET_BUF_EN = "TRUE"

    // CDR
    val RXCDR_CFG = rxOutDivider match {
      // Table 4-13 (UG482 v1.9)
      case 1     => B"83'h0_0011_07FE_2060_2104_1010"
      case 2     => B"83'h0_0011_07FE_2060_2108_1010"
      case 4 | 8 => B"83'h0_0011_07FE_0860_2110_1010"
    }
    val RXCDR_FR_RESET_ON_EIDLE = B"1'b0"
    val RXCDR_HOLD_DURING_EIDLE = B"1'b0"
    val RXCDR_PH_RESET_ON_EIDLE = B"1'b0"
    val RXCDR_LOCK_CFG = B"6'b001001"

    // RX Initialization and Reset
    val RXCDRFREQRESET_TIME = B"5'b00001"
    val RXCDRPHRESET_TIME = B"5'b00001"
    val RXISCANRESET_TIME = B"5'b00001"
    val RXPCSRESET_TIME = B"5'b00001"
    val RXPMARESET_TIME = B"5'b00011"

    // RX OOB Signaling
    val RXOOB_CFG = B"7'b0000110"

    // RX Gearbox
    val RXGEARBOX_EN = "FALSE"
    val GEARBOX_MODE = B"3'b000"

    // PRBS Detection
    val RXPRBS_ERR_LOOPBACK = B"1'b0"

    // Power-Down
    val PD_TRANS_TIME_FROM_P2 = B"12'h03c"
    val PD_TRANS_TIME_NONE_P2 = B"8'h3c"
    val PD_TRANS_TIME_TO_P2 = B"8'h64"

    // RX OOB Signaling
    val SAS_MAX_COM = 64
    val SAS_MIN_COM = 36
    val SATA_BURST_SEQ_LEN = B"4'b0101"
    val SATA_BURST_VAL = B"3'b100"
    val SATA_EIDLE_VAL = B"3'b100"
    val SATA_MAX_BURST = 8
    val SATA_MAX_INIT = 21
    val SATA_MAX_WAKE = 7
    val SATA_MIN_BURST = 4
    val SATA_MIN_INIT = 12
    val SATA_MIN_WAKE = 4

    // RX Fabric Clock Output Control
    val TRANS_TIME_RATE = B"8'h0E"

    // TX Buffer
    val TXBUF_EN = "FALSE"
    val TXBUF_RESET_ON_RATE_CHANGE = "TRUE"
    val TXDLY_CFG = B"16'h001F"
    val TXDLY_LCFG = B"9'h030"
    val TXDLY_TAP_CFG = B"16'h0000"
    val TXPH_CFG = B"16'h0780"
    val TXPHDLY_CFG = B"24'h084020"
    val TXPH_MONITOR_SEL = B"5'b00000"
    val TX_XCLK_SEL = "TXUSR"

    // FPGA TX Interface
    val TX_DATA_WIDTH = 40

    // TX Configurable Driver
    val TX_DEEMPH0 = B"6'b000000"
    val TX_DEEMPH1 = B"6'b000000"
    val TX_EIDLE_ASSERT_DELAY = B"3'b110"
    val TX_EIDLE_DEASSERT_DELAY = B"3'b100"
    val TX_LOOPBACK_DRIVE_HIZ = "FALSE"
    val TX_MAINCURSOR_SEL = B"1'b0"
    val TX_DRIVE_MODE = "DIRECT"
    val TX_MARGIN_FULL_0 = B"7'b1001110"
    val TX_MARGIN_FULL_1 = B"7'b1001001"
    val TX_MARGIN_FULL_2 = B"7'b1000101"
    val TX_MARGIN_FULL_3 = B"7'b1000010"
    val TX_MARGIN_FULL_4 = B"7'b1000000"
    val TX_MARGIN_LOW_0 = B"7'b1000110"
    val TX_MARGIN_LOW_1 = B"7'b1000100"
    val TX_MARGIN_LOW_2 = B"7'b1000010"
    val TX_MARGIN_LOW_3 = B"7'b1000000"
    val TX_MARGIN_LOW_4 = B"7'b1000000"

    // TX Gearbox
    val TXGEARBOX_EN = "FALSE"

    // TX Initialization and Reset
    val TXPCSRESET_TIME = B"5'b00001"
    val TXPMARESET_TIME = B"5'b00001"

    // TX Receiver Detection
    val TX_RXDETECT_CFG = B"14'h1832"
    val TX_RXDETECT_REF = B"3'b100"

    // JTAG
    val ACJTAG_DEBUG_MODE = B"1'b0"
    val ACJTAG_MODE = B"1'b0"
    val ACJTAG_RESET = B"1'b0"

    // CDR
    val CFOK_CFG = B"43'h49000040E80"
    val CFOK_CFG2 = B"7'b0100000"
    val CFOK_CFG3 = B"7'b0100000"
    val CFOK_CFG4 = B"1'b0"
    val CFOK_CFG5 = B"2'h0"
    val CFOK_CFG6 = B"4'b0000"
    val RXOSCALRESET_TIME = B"5'b00011"
    val RXOSCALRESET_TIMEOUT = B"5'b00000"

    // PMA
    val CLK_COMMON_SWING = B"1'b0"
    val RX_CLKMUX_EN = B"1'b1"
    val TX_CLKMUX_EN = B"1'b1"
    val ES_CLK_PHASE_SEL = B"1'b0"
    val USE_PCS_CLK_PHASE_SEL = B"1'b0"
    val PMA_RSV6 = B"1'b0"
    val PMA_RSV7 = B"1'b0"

    // TX Configuration Driver
    val TX_PREDRIVER_MODE = B"1'b0"
    val PMA_RSV5 = B"1'b0"
    val SATA_PLL_CFG = "VCO_3000MHZ"

    // RX Fabric Clock Output Control
    assert(
      Seq(1, 2, 4, 8).contains(rxOutDivider),
      "rxOutDivider must be 1, 2, 4, or 8"
    )
    val RXOUT_DIV = rxOutDivider

    // TX Fabric Clock Output Control
    val TXOUT_DIV = 1

    // RX Phase Interpolator
    val RXPI_CFG0 = B"3'b000"
    val RXPI_CFG1 = B"1'b1"
    val RXPI_CFG2 = B"1'b1"

    // RX Equalizer
    val ADAPT_CFG0 = B"20'h00000"
    val RXLPMRESET_TIME = B"7'b0001111"
    val RXLPM_BIAS_STARTUP_DISABLE = B"1'b0"
    val RXLPM_CFG = B"4'b0110"
    val RXLPM_CFG1 = B"1'b0"
    val RXLPM_CM_CFG = B"1'b0"
    val RXLPM_GC_CFG = B"9'b111100010"
    val RXLPM_GC_CFG2 = B"3'b001"
    val RXLPM_HF_CFG = B"14'b00001111110000"
    val RXLPM_HF_CFG2 = B"5'b01010"
    val RXLPM_HF_CFG3 = B"4'b0000"
    val RXLPM_HOLD_DURING_EIDLE = B"1'b0"
    val RXLPM_INCM_CFG = B"1'b1"
    val RXLPM_IPCM_CFG = B"1'b0"
    val RXLPM_LF_CFG = B"18'b000000001111110000"
    val RXLPM_LF_CFG2 = B"5'b01010"
    val RXLPM_OSINT_CFG = B"3'b100"

    // TX Phase Interpolator PPM Controller
    val TXPI_CFG0 = B"2'b00"
    val TXPI_CFG1 = B"2'b00"
    val TXPI_CFG2 = B"2'b00"
    val TXPI_CFG3 = B"1'b0"
    val TXPI_CFG4 = B"1'b0"
    val TXPI_CFG5 = B"3'b000"
    val TXPI_GREY_SEL = B"1'b0"
    val TXPI_INVSTROBE_SEL = B"1'b0"
    val TXPI_PPMCLK_SEL = "TXUSRCLK2"
    val TXPI_PPM_CFG = B"8'h00"
    val TXPI_SYNFREQ_PPM = B"3'b001"

    // Loopback
    val LOOPBACK_CFG = B"1'b0"
    val PMA_LOOPBACK_CFG = B"1'b0"

    // RX OOB Signalling
    val RXOOB_CLK_CFG = "PMA"

    // TX OOB Signalling
    val TXOOB_CFG = B"1'b0"

    // RX Buffer
    val RXSYNC_MULTILANE = B"1'b1"
    val RXSYNC_OVRD = B"1'b0"
    val RXSYNC_SKIP_DA = B"1'b0"

    // TX Buffer
    val TXSYNC_MULTILANE = B"1'b0"
    val TXSYNC_OVRD = B"1'b1"
    val TXSYNC_SKIP_DA = B"1'b0"
  }

  val io = new Bundle {
    val resetSelection = in Bool () setName ("GTRESETSEL")

    val drp = slave(Gtp2eChannelDrp())
    drp.clk.setName("DRPCLK")
    ClockDomainTag(drpClkDomain)(
      drp.addr.setName("DRPADDR"),
      drp.dataIn.setName("DRPDI"),
      drp.dataOut.setName("DRPDO"),
      drp.enable.setName("DRPEN"),
      drp.writeEnable.setName("DRPWE"),
      drp.ready.setName("DRPRDY")
    )
    if (drpClkDomain != null) {
      mapClockDomain(drpClkDomain, drp.clk)
    }

    val clocking = in(Gtp2eChannelClocking())
    clocking.pll0Clk.setName("PLL0CLK")
    clocking.pll0RefClk.setName("PLL0REFCLK")
    clocking.pll1Clk.setName("PLL1CLK")
    clocking.pll1RefClk.setName("PLL1REFCLK")

    val rx = new Bundle {
      val powerDown = in Bits (2 bits) setName ("RXPD")

      val reset = in Bool () setName ("GTRXRESET")
      val resetDone = out Bool () setName ("RXRESETDONE")
      ClockDomainTag(rxUsrClk2Domain)(resetDone)

      val pmaReset = in Bool () setName ("RXPMARESET")
      val pmaResetDone = out Bool () setName ("RXPMARESETDONE")

      val pcsReset = in Bool () setName ("RXPCSRESET")

      def disable() = {
        powerDown := B"2'11"
        reset := False
        pmaReset := False
        pcsReset := False

        clocking.disable()
        analogFrontEnd.disable()
        outOfBand.disable()
        equalizer.disable()
        clockDataRecovery.disable()
        fabricClockOutput.disable()
        marginAnalysis.disable()
        polarity.disable()
        patternChecker.disable()
        commaAlignment.disable()
        decoder8b10b.disable()
        bufferBypass.disable()
        elasticBuffer.disable()
        channelBonding.disable()
        gearbox.disable()
      }

      val clocking = new Bundle {
        val sysClkSelect = in Bits (2 bits) setName ("RXSYSCLKSEL")
        val usrClk = in Bool () setName ("RXUSRCLK")
        val usrClk2 = in Bool () setName ("RXUSRCLK2")
        val usrReady = in Bool () setName ("RXUSERRDY")

        if (rxUsrClkDomain != null) {
          mapClockDomain(rxUsrClkDomain, usrClk)
        }

        if (rxUsrClk2Domain != null) {
          mapClockDomain(rxUsrClk2Domain, usrClk2)
        }

        def staticSysClk(pmaClkPll: Int, rxOutClkPll: Int) = {
          assert(
            (0 to 1).contains(pmaClkPll),
            "sysClkSelect must be PLL0 or PLL1"
          )
          assert(
            (0 to 1).contains(rxOutClkPll),
            "sysClkSelect must be PLL0 or PLL1"
          )
          sysClkSelect(0) := Bool(pmaClkPll == 1)
          sysClkSelect(1) := Bool(rxOutClkPll == 1)
        }

        def disable() = {
          sysClkSelect := B"2'0"
          usrClk := False
          usrClk2 := False
          usrReady := False
        }
      }

      val analogFrontEnd = new Bundle {
        val input = in(DiffPair())
        input.p.setName("GTPRXP")
        input.n.setName("GTPRXN")

        def disable() = {
          input.p := False
          input.n := False
        }
      }

      val outOfBand = new Bundle {
        val reset = in Bool () setName ("RXOOBRESET")
        val comInitDetect = out Bool () setName ("RXCOMINITDET")
        val comSasDetect = out Bool () setName ("RXCOMSASDET")
        val comWakeDetect = out Bool () setName ("RXCOMWAKEDET")
        val electricalIdle = out Bool () setName ("RXELECIDLE")
        val electricalIdleMode = in Bits (2 bits) setName ("RXELECIDLEMODE")
        val sigValidClk = in Bool () setName ("SIGVALIDCLK")

        ClockDomainTag(rxUsrClk2Domain)(
          comInitDetect,
          comSasDetect,
          comWakeDetect,
          electricalIdle
        )

        def disable() = {
          reset := False
          electricalIdleMode := B"2'11"
          sigValidClk := False
        }
      }

      val equalizer = new Bundle {
        val lpmReset = in Bool () setName ("RXLPMRESET")
        val lpmHighFreqOverrideEn = in Bool () setName ("RXLPMHFOVRDEN")
        val lpmHighFreqHold = in Bool () setName ("RXLPMHFHOLD")
        val lpmLowFreqOverrideEn = in Bool () setName ("RXLPMLFOVRDEN")
        val lpmLowFreqHold = in Bool () setName ("RXLPMLFHOLD")

        def disable() = {
          lpmReset := False
          lpmHighFreqOverrideEn := False
          lpmHighFreqHold := False
          lpmLowFreqOverrideEn := False
          lpmLowFreqHold := False
        }
      }

      val clockDataRecovery = new Bundle {
        val hold = in Bool () setName ("RXCDRHOLD")

        def disable() = {
          hold := False

          offset.disable()
        }

        val offset = new Bundle {
          val hold = in Bool () setName ("RXOSHOLD")
          val overrideEn = in Bool () setName ("RXOSOVRDEN")

          def disable() = {
            hold := False
            overrideEn := False
          }
        }
      }

      val fabricClockOutput = new Bundle {
        val outClkSelect = in Bits (3 bits) setName ("RXOUTCLKSEL")
        val outClk = out Bool () setName ("RXOUTCLK")

        def rxOutClkPma(): Bool = {
          outClkSelect := B"3'010"
          outClk
        }

        def disable() = {
          outClkSelect := B"3'011"
          rate.disable()
        }

        val rate = new Bundle {
          val mode = in Bool () setName ("RXRATEMODE")
          val divider = in Bits (3 bits) setName ("RXRATE")
          val done = out Bool () setName ("RXRATEDONE")

          ClockDomainTag(rxUsrClk2Domain)(done)

          def syncMode() = {
            mode := False
            ClockDomainTag(rxUsrClk2Domain)(divider)
          }

          def disable() = {
            mode := False
            divider := B"3'0"
          }
        }
      }

      val marginAnalysis = new Bundle {
        val reset = in Bool () setName ("EYESCANRESET")
        val mode = in Bool () setName ("EYESCANMODE")
        val trigger = in Bool () setName ("EYESCANTRIGGER")
        val dataErr = out Bool () setName ("EYESCANDATAERROR")

        ClockDomainTag(rxUsrClk2Domain)(trigger)

        def disable() = {
          reset := False
          mode := False
          trigger := False
        }
      }

      val polarity = new Bundle {
        val invert = in Bool () setName ("RXPOLARITY")

        def disable() = {
          invert := False
        }
      }

      val patternChecker = new Bundle {
        val prbsErrCounterReset = in Bool () setName ("RXPRBSCNTRESET")
        val prbsPatternSelect = in Bits (3 bits) setName ("RXPRBSSEL")
        val prbsErr = out Bool () setName ("RXPRBSERR")

        ClockDomainTag(rxUsrClk2Domain)(
          prbsErrCounterReset,
          prbsPatternSelect,
          prbsErr
        )

        def disable() = {
          prbsErrCounterReset := False
          prbsPatternSelect := B"3'0"
        }
      }

      // Byte alignment
      val byteAlignment = new Bundle {
        val isAligned = out Bool () setName ("RXBYTEISALIGNED")
        val realign = out Bool () setName ("RXBYTEREALIGN")

        ClockDomainTag(rxUsrClk2Domain)(
          isAligned,
          realign
        )
      }

      // Comma alignment
      val commaAlignment = new Bundle {
        val detectEnable = in Bool () setName ("RXCOMMADETEN")
        val detect = out Bool () setName ("RXCOMMADET")
        val mCommaEnable = in Bool () setName ("RXMCOMMAALIGNEN")
        val pCommaEnable = in Bool () setName ("RXPCOMMAALIGNEN")
        val slide = in Bool () setName ("RXSLIDE")

        ClockDomainTag(rxUsrClk2Domain)(
          detectEnable,
          detect,
          mCommaEnable,
          pCommaEnable,
          slide
        )

        def disable() = {
          detectEnable := False
          mCommaEnable := False
          pCommaEnable := False
          slide := False
        }
      }

      // 8b/10b decoder (not TMDS compatible)
      val decoder8b10b = new Bundle {
        val enable = in Bool () setName ("RX8B10BEN")
        val charIsComma = out Bits (4 bits) setName ("RXCHARISCOMMA")
        val charIsK = out Bits (4 bits) setName ("RXCHARISK")
        val disparityErr = out Bits (4 bits) setName ("RXDISPERR")
        val notInTable = out Bits (4 bits) setName ("RXNOTINTABLE")

        ClockDomainTag(rxUsrClk2Domain)(
          enable,
          charIsComma,
          charIsK,
          disparityErr,
          notInTable
        )

        def disable() = {
          enable := False
        }
      }

      val bufferBypass = new Bundle {
        val powerDown = in Bool () setName ("RXPHDLYPD")
        val reset = in Bool () setName ("RXPHDLYRESET")

        def disable() = {
          powerDown := False
          reset := False

          phaseAlignment.disable()
          delayAlignment.disable()
          sync.disable()
        }

        val phaseAlignment = new Bundle {
          val enable = in Bool () setName ("RXPHALIGNEN")
          val set = in Bool () setName ("RXPHALIGN")
          val done = out Bool () setName ("RXPHALIGNDONE")
          val counterOverrideEn = in Bool () setName ("RXPHOVRDEN")
          val monitor = out Bits (5 bits) setName ("RXPHMONITOR")
          val slipMonitor = out Bits (5 bits) setName ("RXPHSLIPMONITOR")

          def disable() = {
            enable := False
            set := False
            counterOverrideEn := False
          }
        }

        val delayAlignment = new Bundle {
          val bypass = in Bool () setName ("RXDLYBYPASS")
          val softReset = in Bool () setName ("RXDLYSRESET")
          val softResetDone = out Bool () setName ("RXDLYSRESETDONE")
          val enable = in Bool () setName ("RXDLYEN")
          val counterOverrideEn = in Bool () setName ("RXDLYOVRDEN")
          val insertionEnable = in Bool () setName ("RXDDIEN")

          def disable() = {
            bypass := True
            softReset := False
            enable := False
            counterOverrideEn := False
            insertionEnable := False
          }
        }

        val sync = new Bundle {
          val mode = in Bool () setName ("RXSYNCMODE")
          val input = in Bool () setName ("RXSYNCIN")
          val allPhaseAlignDone = in Bool () setName ("RXSYNCALLIN")
          val output = out Bool () setName ("RXSYNCOUT")
          val done = out Bool () setName ("RXSYNCDONE")

          def disable() = {
            mode := False
            input := False
            allPhaseAlignDone := False
          }
        }
      }

      val elasticBuffer = new Bundle {
        val reset = in Bool () setName ("RXBUFRESET")
        val status = out Bits (3 bits) setName ("RXBUFSTATUS")

        ClockDomainTag(rxUsrClk2Domain)(status)

        def disable() = {
          reset := False
        }
      }

      val clockCorrection = new Bundle {
        val status = out Bits (2 bits) setName ("RXCLKCORCNT")
        ClockDomainTag(rxUsrClk2Domain)(status)
      }

      val channelBonding = new Bundle {
        val enable = in Bool () setName ("RXCHBONDEN")
        val master = in Bool () setName ("RXCHBONDMASTER")
        val slave = in Bool () setName ("RXCHBONDSLAVE")
        val seqDetected = out Bool () setName ("RXCHANBONDSEQ")
        val isAligned = out Bool () setName ("RXCHANISALIGNED")
        val realign = out Bool () setName ("RXCHANREALIGN")
        val level = in Bits (3 bits) setName ("RXCHBONDLEVEL")
        val output = out Bits (4 bits) setName ("RXCHBONDO")
        val input = in Bits (4 bits) setName ("RXCHBONDI")

        ClockDomainTag(rxUsrClkDomain)(
          output,
          input
        )

        ClockDomainTag(rxUsrClk2Domain)(
          enable,
          master,
          slave,
          seqDetected,
          isAligned,
          realign,
          level
        )

        def disable() = {
          enable := False
          master := False
          slave := False
          level := B"3'0"
          input := B"4'0"
        }
      }

      val gearbox = new Bundle {
        val slip = in Bool () setName ("RXGEARBOXSLIP")
        if (generic.RXGEARBOX_EN == "FALSE") {
          slip.default(False)
        }
        val dataValid = out Bits (2 bits) setName ("RXDATAVALID")
        val headerValid = out Bool () setName ("RXHEADERVALID")
        val header = out Bits (3 bits) setName ("RXHEADER")
        val startOfSeq = out Bits (2 bits) setName ("RXSTARTOFSEQ")

        ClockDomainTag(rxUsrClk2Domain)(
          slip,
          dataValid,
          headerValid,
          header,
          startOfSeq
        )

        def disable() = {
          slip := False
        }
      }

      val pcie = new Bundle {
        val valid = out Bool () setName ("RXVALID")
        val status = out Bits (3 bits) setName ("RXSTATUS")
        val phyStatus = out Bool () setName ("PHYSTATUS")

        ClockDomainTag(rxUsrClk2Domain)(
          valid,
          phyStatus,
          status
        )
      }

      val rawData = out Bits (32 bits) setName ("RXDATA")
      ClockDomainTag(rxUsrClk2Domain)(rawData)

      def data(decoder8b10bBypass: Boolean): Bits = {
        if (decoder8b10bBypass) {
          Cat(
            decoder8b10b.disparityErr(3),
            decoder8b10b.charIsK(3),
            rawData(31 downto 24),
            decoder8b10b.disparityErr(2),
            decoder8b10b.charIsK(2),
            rawData(23 downto 16),
            decoder8b10b.disparityErr(1),
            decoder8b10b.charIsK(1),
            rawData(15 downto 8),
            decoder8b10b.disparityErr(0),
            decoder8b10b.charIsK(0),
            rawData(7 downto 0)
          )
        } else {
          rawData
        }
      }
    }

    val tx = new Bundle {
      val powerDown = in Bits (2 bits) setName ("TXPD")

      val reset = in Bool () setName ("GTTXRESET")
      val resetDone = out Bool () setName ("TXRESETDONE")

      val pmaReset = in Bool () setName ("TXPMARESET")
      val pmaResetDone = out Bool () setName ("TXPMARESETDONE")

      val pcsReset = in Bool () setName ("TXPCSRESET")

      val data = in Bits (32 bits) setName ("TXDATA")

      def disable() = {
        powerDown := B"2'11"
        reset := False
        pmaReset := False
        pcsReset := False
        data := B"32'0"

        clocking.disable()
        encoder8b10b.disable()
        gearbox.disable()
        bufferBypass.disable()
        patternGenerator.disable()
        polarity.disable()
        fabricClockOutput.disable()
        phaseInterpolator.disable()
        driver.disable()
        pcie.disable()
        outOfBand.disable()
      }

      val clocking = new Bundle {
        val sysClkSelect = in Bits (2 bits) setName ("TXSYSCLKSEL")
        val usrClk = in Bool () setName ("TXUSRCLK")
        val usrClk2 = in Bool () setName ("TXUSRCLK2")
        val usrReady = in Bool () setName ("TXUSERRDY")

        def disable() = {
          sysClkSelect := B"2'0"
          usrClk := False
          usrClk2 := False
          usrReady := False
        }
      }

      val encoder8b10b = new Bundle {
        val enable = in Bool () setName ("TX8B10BEN")
        val bypass = in Bits (4 bits) setName ("TX8B10BBYPASS")
        val charDisparityMode = in Bits (4 bits) setName ("TXCHARDISPMODE")
        val charDisparityValue = in Bits (4 bits) setName ("TXCHARDISPVAL")
        var charIsK = in Bits (4 bits) setName ("TXCHARISK")

        def disable() = {
          enable := False
          bypass := B"4'0"
          charDisparityMode := B"4'0"
          charDisparityValue := B"4'0"
          charIsK := B"4'0"
        }
      }

      val gearbox = new Bundle {
        val ready = out Bool () setName ("TXGEARBOXREADY")
        val header = in Bits (3 bits) setName ("TXHEADER")
        val sequence = in Bits (7 bits) setName ("TXSEQUENCE")
        val startSeq = in Bool () setName ("TXSTARTSEQ")

        def disable() = {
          header := B"3'0"
          sequence := B"7'0"
          startSeq := False
        }
      }

      val buffer = new Bundle {
        val status = out Bits (2 bits) setName ("TXBUFSTATUS")
      }

      val bufferBypass = new Bundle {
        val powerDown = in Bool () setName ("TXPHDLYPD")
        val reset = in Bool () setName ("TXPHDLYRESET")

        def disable() = {
          powerDown := True
          reset := False

          phaseAlignment.disable()
          delayAlignment.disable()
        }

        val phaseAlignment = new Bundle {
          val enable = in Bool () setName ("TXPHALIGNEN")
          val set = in Bool () setName ("TXPHALIGN")
          val done = out Bool () setName ("TXPHALIGNDONE")
          val init = in Bool () setName ("TXPHINIT")
          val initDone = out Bool () setName ("TXPHINITDONE")
          val counterOverrideEn = in Bool () setName ("TXPHOVRDEN")

          def disable() = {
            enable := False
            set := False
            init := False
            counterOverrideEn := False
          }
        }

        val delayAlignment = new Bundle {
          val bypass = in Bool () setName ("TXDLYBYPASS")
          val softReset = in Bool () setName ("TXDLYSRESET")
          val softResetDone = out Bool () setName ("TXDLYSRESETDONE")
          val enable = in Bool () setName ("TXDLYEN")
          val counterOverrideEn = in Bool () setName ("TXDLYOVRDEN")

          val clk = in Bool () setName ("TXPHDLYTSTCLK")
          val hold = in Bool () setName ("TXDLYHOLD")
          val upOrDown = in Bool () setName ("TXDLYUPDOWN")

          def disable() = {
            bypass := False
            softReset := False
            enable := False
            counterOverrideEn := False
            clk := False
            hold := False
            upOrDown := False
          }
        }
      }

      val patternGenerator = new Bundle {
        val prbsPatternSelect = in Bits (3 bits) setName ("TXPRBSSEL")
        val prbsForceErr = in Bool () setName ("TXPRBSFORCEERR")

        def disable() = {
          prbsPatternSelect := B"3'0"
          prbsForceErr := False
        }
      }

      val polarity = new Bundle {
        val invert = in Bool () setName ("TXPOLARITY")

        def disable() = {
          invert := False
        }
      }

      val fabricClockOutput = new Bundle {
        val outClkSelect = in Bits (3 bits) setName ("TXOUTCLKSEL")
        val outClk = out Bool () setName ("TXOUTCLK")

        def disable() = {
          outClkSelect := B"3'011"
          rate.disable()
        }

        val rate = new Bundle {
          val mode = in Bool () setName ("TXRATEMODE")
          val divider = in Bits (3 bits) setName ("TXRATE")
          val done = out Bool () setName ("TXRATEDONE")

          def disable() = {
            mode := False
            divider := B"3'0"
          }
        }
      }

      val phaseInterpolator = new Bundle {
        val powerDown = in Bool () setName ("TXPIPPMPD")
        val enable = in Bool () setName ("TXPIPPMEN")
        val overrideEn = in Bool () setName ("TXPIPPMOVRDEN")
        val stepSize = in Bits (5 bits) setName ("TXPIPPMSTEPSIZE")

        def disable() = {
          powerDown := True
          enable := False
          overrideEn := False
          stepSize := B"5'0"
        }
      }

      val driver = new Bundle {
        val inhibit = in Bool () setName ("TXINHIBIT")
        val electricalIdle = in Bool () setName ("TXELECIDLE")
        val preDriverSwing = in Bits (3 bits) setName ("TXBUFDIFFCTRL")
        val driverSwing = in Bits (4 bits) setName ("TXDIFFCTRL")
        val deEmphasis = in Bool () setName ("TXDEEMPH")

        val mainCursor = in Bits (7 bits) setName ("TXMAINCURSOR")
        val margin = in Bits (3 bits) setName ("TXMARGIN")

        val preCursor = in Bits (5 bits) setName ("TXPRECURSOR")
        val preCursorInvert = in Bool () setName ("TXPRECURSORINV")

        val postCursor = in Bits (5 bits) setName ("TXPOSTCURSOR")
        val postCursorInvert = in Bool () setName ("TXPOSTCURSORINV")

        val output = out(new DiffPair())
        output.p.setName("GTPTXP")
        output.n.setName("GTPTXN")

        def disable() = {
          inhibit := False
          electricalIdle := True
          preDriverSwing := B"3'0"
          driverSwing := B"4'0"
          deEmphasis := False
          mainCursor := B"7'0"
          margin := B"3'0"
          preCursor := B"5'0"
          preCursorInvert := False
          postCursor := B"5'0"
          postCursorInvert := False
        }
      }

      val pcie = new Bundle {
        val swing = in Bool () setName ("TXSWING")
        val detectReceiver = in Bool () setName ("TXDETECTRX")

        def disable() = {
          swing := False
          detectReceiver := False
        }
      }

      val outOfBand = new Bundle {
        val comInit = in Bool () setName ("TXCOMINIT")
        val comSas = in Bool () setName ("TXCOMSAS")
        val comWake = in Bool () setName ("TXCOMWAKE")
        val comFinish = out Bool () setName ("TXCOMFINISH")
        val electricalIdleMode = in Bool () setName ("TXPDELECIDLEMODE")

        def disable() = {
          comInit := False
          comSas := False
          comWake := False
          electricalIdleMode := False
        }
      }
    }

    val loopback = new Bundle {
      val mode = in Bits (3 bits) setName ("LOOPBACK")

      def disable() = {
        mode := B"3'0"
      }
    }

    val digitalMonitor = new Bundle {
      val clk = in Bool () setName ("DMONITORCLK")
      val output = out Bits (15 bits) setName ("DMONITOROUT")

      def disable() = {
        clk := False
      }
    }

    val reserved = new Bundle {
      val gtRsvd = in Bits (16 bits) setName ("GTRSVD") default (B"16'0")
      val pcsRsvdIn = in Bits (16 bits) setName ("PCSRSVDIN") default (B"16'0")
      val tstIn = in Bits (20 bits) setName ("TSTIN") default (B"20'hFFFFF")
      val pmaRsvdOut0 = out Bool () setName ("PMARSVDOUT0")
      val pmaRsvdOut1 = out Bool () setName ("PMARSVDOUT1")
      val pmaRsvdIn0 = in Bool () setName ("PMARSVDIN0") default (False)
      val pmaRsvdIn1 = in Bool () setName ("PMARSVDIN1") default (False)
      val pmaRsvdIn2 = in Bool () setName ("PMARSVDIN2") default (False)
      val pmaRsvdIn3 = in Bool () setName ("PMARSVDIN3") default (False)
      val pmaRsvdIn4 = in Bool () setName ("PMARSVDIN4") default (False)
      val rxCdrReset = in Bool () setName ("RXCDRRESET") default (False)
      val rxCdrFreqReset = in Bool () setName ("RXCDRFREQRESET") default (False)
      val rxCdrOvrdEn = in Bool () setName ("RXCDROVRDEN") default (False)
      val rxCdrResetRsv = in Bool () setName ("RXCDRRESETRSV") default (False)
      val rxCdrLock = out Bool () setName ("RXCDRLOCK")
      val rxOsIntDone = out Bool () setName ("RXOSINTDONE")
      val rxOsIntStarted = out Bool () setName ("RXOSINTSTARTED")
      val rxOsIntStrobeDone = out Bool () setName ("RXOSINTSTROBEDONE")
      val rxOsIntStrobeStarted = out Bool () setName ("RXOSINTSTROBESTARTED")
      val rxOsCalReset = in Bool () setName ("RXOSCALRESET") default (False)
      val rxOsIntEn = in Bool () setName ("RXOSINTEN") default (True)
      val rxOsIntHold = in Bool () setName ("RXOSINTHOLD") default (False)
      val rxOsIntNtrLen = in Bool () setName ("RXOSINTNTRLEN") default (False)
      val rxOsIntOvrdEn = in Bool () setName ("RXOSINTOVRDEN") default (False)
      val rxOsIntPd = in Bool () setName ("RXOSINTPD") default (False)
      val rxOsIntStrobe = in Bool () setName ("RXOSINTSTROBE") default (False)
      val rxOsIntTestOvrdEn =
        in Bool () setName ("RXOSINTTESTOVRDEN") default (False)
      val rxOsIntCfg =
        in Bits (4 bits) setName ("RXOSINTCFG") default (B"4'b0010")
      val rxOsIntID0 = in Bits (4 bits) setName ("RXOSINTID0") default (B"4'0")
      val rxLpmOsIntNtrLen =
        in Bool () setName ("RXLPMOSINTNTRLEN") default (False)
      val rxOutClkFabric = out Bool () setName ("RXOUTCLKFABRIC")
      val rxOutClkPcs = out Bool () setName ("RXOUTCLKPCS")
      val dMonFifoReset = in Bool () setName ("DMONFIFORESET") default (False)
      val pcsRsvdOut = out Bits (16 bits) setName ("PCSRSVDOUT")
      val clkRsvd0 = in Bool () setName ("CLKRSVD0") default (False)
      val clkRsvd1 = in Bool () setName ("CLKRSVD1") default (False)
      val resetOvrd = in Bool () setName ("RESETOVRD") default (False)
      val rxDfeXYDEn = in Bool () setName ("RXDFEXYDEN") default (False)
      val rxAdaptSelTest =
        in Bits (14 bits) setName ("RXADAPTSELTEST") default (B"14'0")
      val setErrStatus = in Bool () setName ("SETERRSTATUS") default (False)
      val txSyncMode = in Bool () setName ("TXSYNCMODE") default (False)
      val txSyncIn = in Bool () setName ("TXSYNCIN") default (False)
      val txSyncOut = out Bool () setName ("TXSYNCOUT")
      val txSyncAllIn = in Bool () setName ("TXSYNCALLIN") default (False)
      val txSyncDone = out Bool () setName ("TXSYNCDONE")
      val txOutClkFabric = out Bool () setName ("TXOUTCLKFABRIC")
      val txOutClkPcs = out Bool () setName ("TXOUTCLKPCS")
      val txPiPpmSel = in Bool () setName ("TXPIPPMSEL") default (True)
      val txPiSoPd = in Bool () setName ("TXPISOPD") default (False)
      val txDiffPd = in Bool () setName ("TXDIFFPD") default (False)
      val cfgReset = in Bool () setName ("CFGRESET") default (False)
    }
  }

  noIoPrefix()
  setBlackBoxName("GTPE2_CHANNEL")
}

// Performs workaround for reset per:
// 53561 - Design Advisory for Artix-7 FPGA GTP Transceivers:
// RX Reset Sequence Requirement for Production Silicon
class Gtpe2ChannelRxReset() extends Component {
  val io = new Bundle {
    val resetIn = in Bool ()
    val drp = master(Gtp2eChannelDrp())
    val rxReset = out Bool ()
    val rxPmaResetDone = in Bool ()
  }

  def driveChannel(channel: Gtpe2Channel, resetIn: Bool) = {
    io.resetIn := resetIn
    channel.io.drp.addr := io.drp.addr
    channel.io.drp.dataIn := io.drp.dataIn
    io.drp.dataOut := channel.io.drp.dataOut
    channel.io.drp.enable := io.drp.enable
    channel.io.drp.writeEnable := io.drp.writeEnable
    io.drp.ready := channel.io.drp.ready
    channel.io.rx.reset := io.rxReset
    io.rxPmaResetDone := channel.io.rx.pmaResetDone
  }

  io.drp.clk := ClockDomain.current.readClockWire
  io.drp.addr := 0x11
  io.drp.enable.setAsReg() init (False)
  io.drp.writeEnable.setAsReg() init (False)
  io.drp.dataIn.setAsReg().randBoot()
  io.rxReset.setAsReg() init (False)

  val backupBit = Reg(Bool) randBoot ()
  val pmaResetDoneFall = io.rxPmaResetDone.fall(initAt = False)

  val fsm = new StateMachine {
    io.drp.enable := False
    io.drp.writeEnable := False
    io.rxReset := False

    val idle: State = new State with EntryPoint {
      whenIsActive {
        when(io.resetIn) {
          io.rxReset := True

          // issue DRP read
          io.drp.enable := True
          io.drp.writeEnable := False
          goto(waitForRead)
        }
      }
    }

    val waitForRead: State = new State {
      whenIsActive {
        io.rxReset := True

        when(io.drp.ready) {
          backupBit := io.drp.dataOut(11)
          // clear bit 11
          io.drp.dataIn(15 downto 12) := io.drp.dataOut(15 downto 12)
          io.drp.dataIn(11) := False
          io.drp.dataIn(10 downto 0) := io.drp.dataOut(10 downto 0)

          // issue DRP write
          io.drp.enable := True
          io.drp.writeEnable := True

          goto(waitForWrite)
        }
      }
    }

    val waitForWrite: State = new State {
      whenIsActive {
        io.rxReset := True

        when(io.drp.ready) {
          when(io.resetIn) {
            goto(waitForResetInFall)
          } otherwise {
            goto(waitForPmaResetDone)
          }
        }
      }
    }

    val waitForResetInFall: State = new State {
      whenIsActive {
        io.rxReset := True

        when(!io.resetIn) {
          goto(waitForPmaResetDone)
        }
      }
    }

    val waitForPmaResetDone: State = new State {
      whenIsActive {
        when(pmaResetDoneFall) {
          goto(issueRestore)
        }
      }
    }

    val issueRestore: State = new State {
      whenIsActive {
        io.drp.dataIn(11) := backupBit

        // issue DRP write
        io.drp.enable := True
        io.drp.writeEnable := True
        goto(waitForRestore)
      }
    }

    val waitForRestore: State = new State {
      whenIsActive {
        when(io.drp.ready) {
          goto(idle)
        }
      }
    }
  }
}
