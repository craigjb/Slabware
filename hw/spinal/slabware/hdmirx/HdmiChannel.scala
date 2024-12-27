package slabware.hdmirx

import spinal.core._
import spinal.lib._

class HdmiChannel(
    invertPolarity: Boolean = false,
    simResetSpeedup: Boolean = false,
    pll: Int,
    usrClkDomain: ClockDomain
) extends Component {
  val io = new Bundle {
    val pair = in(new DiffPair())
    val clocking = in(Gtp2eChannelClocking())
    val rxOutClk = out Bool ()
    val reset = in Bool ()
    val resetDone = out Bool ()
    val hdmiOut = Vec.fill(2)(master(Flow(HdmiPayload())))
  }

  assert((0 to 1).contains(pll), "PLL must be 0 or 1")

  val gtpChannel = new Gtpe2Channel(
    simResetSpeedup = simResetSpeedup,
    drpClkDomain = ClockDomain.current,
    // For rxDataWidth == 20:
    // usrClk2 == usrClk == rxOutClk
    rxUsrClkDomain = usrClkDomain,
    rxUsrClk2Domain = usrClkDomain,
    rxDataWidth = 20,
    rxOutDivider = 4,
    commaEnable = "10'0111111111",
    mCommaValue = "10'0101010100",
    pCommaValue = "10'0010101011"
  )

  // sequential reset
  gtpChannel.io.resetSelection := False

  gtpChannel.io.clocking <> io.clocking

  gtpChannel.io.rx.powerDown := B"2'00"
  gtpChannel.io.rx.pmaReset := False
  gtpChannel.io.rx.pcsReset := False
  io.resetDone := BufferCC(
    gtpChannel.io.rx.resetDone,
    init = False,
    randBoot = true
  )
  if (simResetSpeedup) {
    gtpChannel.io.rx.reset := io.reset
    gtpChannel.io.drp.disable()
  } else {
    val gtpReset = new Gtpe2ChannelRxReset()
    gtpReset.driveChannel(gtpChannel, io.reset)
  }

  gtpChannel.io.rx.analogFrontEnd.input <> io.pair
  gtpChannel.io.rx.polarity.invert := Bool(invertPolarity)

  gtpChannel.io.rx.clocking.staticSysClk(pll, pll)
  gtpChannel.io.rx.clocking.usrReady := True

  io.rxOutClk := gtpChannel.io.rx.fabricClockOutput.rxOutClkPma()
  gtpChannel.io.rx.fabricClockOutput.rate.disable()

  gtpChannel.io.rx.commaAlignment.detectEnable := True
  gtpChannel.io.rx.commaAlignment.slide := False

  val usrClkArea = new ClockingArea(usrClkDomain) {
    val commaAligned = RegInit(False)
    val realigned = RegInit(False)
    val commaCount = RegInit(U(0, 3 bits))

    val isComma = gtpChannel.io.rx.commaAlignment.detect
    val realign = gtpChannel.io.rx.byteAlignment.realign
    when(isComma) {
      when(commaCount =/= 7) {
        commaCount := commaCount + 1
      }
      when(realign) {
        realigned := True
      }
    } otherwise {
      commaCount := 0
      realigned := False
    }

    when(commaCount >= 4 && realigned) {
      commaAligned := True
    }

    gtpChannel.io.rx.commaAlignment.mCommaEnable := !commaAligned
    gtpChannel.io.rx.commaAlignment.pCommaEnable := !commaAligned

    val hdmiOutValid = Delay(
      gtpChannel.io.rx.resetDone && commaAligned,
      1
    )

    val rawData = Bits(20 bits)
    rawData := gtpChannel.io.rx.data(true)(19 downto 0)

    val hdmiOut0Flow = new Flow(HdmiPayload())
    hdmiOut0Flow.valid := hdmiOutValid
    hdmiOut0Flow.payload := HdmiDecoder.on(rawData(9 downto 0))
    io.hdmiOut(0) := hdmiOut0Flow.stage()

    val hdmiOut1Flow = new Flow(HdmiPayload())
    hdmiOut1Flow.valid := hdmiOutValid
    hdmiOut1Flow.payload := HdmiDecoder.on(rawData(19 downto 10))
    io.hdmiOut(1) := hdmiOut1Flow.stage()
  }

  def allValid: Bool = {
    io.hdmiOut(0).valid && io.hdmiOut(1).valid
  }

  gtpChannel.io.rx.outOfBand.disable()
  gtpChannel.io.rx.equalizer.disable()
  gtpChannel.io.rx.clockDataRecovery.disable()
  gtpChannel.io.rx.marginAnalysis.disable()
  gtpChannel.io.rx.patternChecker.disable()
  gtpChannel.io.rx.decoder8b10b.disable()
  gtpChannel.io.rx.bufferBypass.disable()
  gtpChannel.io.rx.elasticBuffer.disable()
  gtpChannel.io.rx.channelBonding.disable()
  gtpChannel.io.rx.gearbox.disable()
  gtpChannel.io.tx.disable()
  gtpChannel.io.loopback.disable()
  gtpChannel.io.digitalMonitor.disable()
}
