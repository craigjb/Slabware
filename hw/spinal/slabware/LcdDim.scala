package slabware

import spinal.core._
import spinal.lib._
import spinal.lib.bus.misc._

object LcdDim {
  def periodReg(counterWidth: BigInt, defaultPeriod: BigInt) = {
    RegInit(U(defaultPeriod, counterWidth bits))
  }

  def dutyReg(counterWidth: BigInt, defaultDuty: Double) = {
    val period = BigInt(2).pow(counterWidth.toInt)
    val duty = (period.toDouble * defaultDuty).toInt
    RegInit(U(duty, counterWidth bits))
  }

  def apply(
      counterWidth: BigInt,
      defaultPeriod: BigInt,
      defaultDuty: Double,
      enable: Bool,
      pwmOut: Bool,
      bus: BusSlaveFactory,
      addrOffset: BigInt
  ) = {
    val dimPeriod = LcdDim.periodReg(counterWidth, defaultPeriod)
    bus.readAndWrite(dimPeriod, addrOffset, 0)

    val dimDuty = LcdDim.dutyReg(counterWidth, defaultDuty)
    bus.readAndWrite(dimDuty, addrOffset + 4, 0)

    val dim = new LcdDim(counterWidth = 8)
    dim.io.enable := enable
    pwmOut := dim.io.pwmOut
    dim.io.period := dimPeriod
    dim.io.duty := dimDuty

    dim
  }
}

class LcdDim(counterWidth: BigInt) extends Component {
  assert(counterWidth > 0, "LcdDim counter width must be at least one bit")
  assert(counterWidth <= 32, "LcdDim counter width must be ≤ 32 bits")

  val io = new Bundle {
    val pwmOut = out Bool ()
    val enable = in Bool ()
    val period = in UInt (counterWidth bits)
    val duty = in UInt (counterWidth bits)
  }

  val prescaler = RegInit(U(0, 5 bits))
  prescaler := prescaler + 1

  val counter = RegInit(U(0, counterWidth bits))
  when(!io.enable || counter >= io.period) {
    counter := 0
  } otherwise {
    when(prescaler === 0) {
      counter := counter + 1
    }
  }

  io.pwmOut := io.enable && (counter <= io.duty)
}
