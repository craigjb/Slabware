package slabware.hdmirx

import scala.BigDecimal.RoundingMode

import spinal.core._
import spinal.lib._
import spinal.lib.bus.regif.{BusIf, AccessType, SymbolName}

case class ClockDetectorConfig(
    dividerBits: Int = 8,
    maxHdmiClkFreq: HertzNumber = 165 MHz,
    sampleRate: HertzNumber = 1 kHz,
    toleranceBits: Int = 5
)

case class ClockDetector(
    config: ClockDetectorConfig
) extends Component {
  import config._

  val divisor = (1 << dividerBits)
  val maxDividedHdmiClkFreq = (maxHdmiClkFreq / divisor)
  val minClockDetectorFreq = maxDividedHdmiClkFreq * 10
  assert(
    ClockDomain.current.frequency.getValue > minClockDetectorFreq,
    "HDMI ClockDetector must have a faster" +
      " clock than the divided HDMI clock"
  )

  val counterWidth = log2Up(
    (maxDividedHdmiClkFreq / sampleRate)
      .setScale(0, RoundingMode.CEILING)
      .toBigInt
  ) + 1

  val sampleRateWidth = log2Up(sampleRate.toInt)

  val io = new Bundle {
    val hdmiTmdsClk = in Bool ()
    val divisor = out UInt ((dividerBits + 1) bits)
    val sampleRate = out UInt (sampleRateWidth bits)
    val count = out UInt (counterWidth bits)
    val counterUpdate = out Bool ()
    val tolerance = in UInt (toleranceBits bits)
    val freqChanged = out Bool ()
  }

  io.divisor := divisor
  io.sampleRate := sampleRate.toInt

  val tmdsClkDomain = ClockDomain(
    clock = io.hdmiTmdsClk,
    config = ClockDomainConfig(resetKind = BOOT)
  )

  val tmdsClkArea = new ClockingArea(tmdsClkDomain) {
    val tmdsClkDivider = CounterFreeRun(stateCount = divisor)
  }

  val tmdsDividedClkCC =
    BufferCC(tmdsClkArea.tmdsClkDivider.msb, randBoot = true)
  val counter = Counter(counterWidth bits)
  when(tmdsDividedClkCC.rise(initAt = False)) {
    counter.increment()
  }

  val toleranceLower = io.count - io.tolerance
  val toleranceUpper = io.count + io.tolerance

  io.count.setAsReg() init (0)
  val timer = Timeout(sampleRate.toTime)
  io.counterUpdate := timer
  io.freqChanged.setAsReg() init (False) := False
  when(timer) {
    io.count := counter
    counter.clear()
    timer.clear()

    when(counter < toleranceLower || counter > toleranceUpper) {
      io.freqChanged := True
    }
  }

  def drive(busIf: BusIf, defaultTolerance: Int = 1) = new Area {
    val divisorReg =
      busIf.newReg(doc = "Clock detector divisor").setName("clkDetDivisor")
    divisorReg.field(
      UInt((dividerBits + 1) bits),
      AccessType.RO,
      doc = "Divisor value"
    )(SymbolName("value")) := io.divisor

    val sampleRateReg = busIf
      .newReg(doc = "Clock detector sample rate")
      .setName("clkDetSampleRate")
    sampleRateReg.field(
      UInt(sampleRateWidth bits),
      AccessType.RO,
      doc = "Sample rate in Hz"
    )(SymbolName("value")) := io.sampleRate

    val countReg =
      busIf
        .newReg(doc = "Clock detector frequency count")
        .setName("clkDetCount")
    countReg.field(
      UInt(counterWidth bits),
      AccessType.RO,
      doc = "Frequency counter value"
    )(SymbolName("value")) := io.count

    val toleranceReg =
      busIf.newReg(doc = "Clock detector tolerance").setName("clkDetTolerance")
    io.tolerance := toleranceReg.field(
      UInt(toleranceBits bits),
      AccessType.RW,
      resetValue = defaultTolerance,
      doc = "Tolerance for frequency change detection"
    )(SymbolName("value"))
  }
}
