package slabware

import spinal.core._
import spinal.lib._
import spinal.lib.bus.misc._
import spinal.lib.bus.regif.AccessType

class LcdDim[B <: BusDefinition.Bus](
    busDef: BusDefinition[B],
    counterWidth: BigInt = 8,
    defaultPeriod: BigInt = BigInt(2).pow(8) - 1,
    defaultDuty: Double = 0.5
) extends Component {
  assert(counterWidth > 0, "LcdDim counter width must be at least one bit")
  assert(counterWidth <= 32, "LcdDim counter width must be ≤ 32 bits")

  val AddressWidth = 8
  val DataWidth = 32

  val io = new Bundle {
    val bus = slave(busDef.createBus(AddressWidth, DataWidth))
    val pwmOut = out Bool ()
  }

  val busif = busDef.createBusInterface(io.bus, (0, 4))

  val ctrl = new Area {
    val ctrlReg = busif.newReg(doc = "PWM control").setName("Control")
    val enable = ctrlReg.field(
      Bool(),
      AccessType.RW,
      resetValue = 0,
      doc = "PWM output enable"
    )
  }

  val period = new Area {
    val periodReg = busif.newReg(doc = "PWM period").setName("Period")
    val value = periodReg.field(
      UInt(counterWidth bits),
      AccessType.RW,
      resetValue = defaultPeriod,
      doc = "PWM period value"
    )
  }

  val duty = new Area {
    val dutyReg = busif.newReg(doc = "PWM duty").setName("Duty")
    val defaultDutyValue = (defaultPeriod.toDouble * defaultDuty).toInt
    val value = dutyReg.field(
      UInt(counterWidth bits),
      AccessType.RW,
      resetValue = defaultDutyValue,
      doc = "PWM duty value"
    )
  }

  val prescaler = RegInit(U(0, 2 bits))
  prescaler := prescaler + 1

  val counter = RegInit(U(0, counterWidth bits))
  when(!ctrl.enable || counter >= period.value) {
    counter := 0
  } otherwise {
    when(prescaler === 0) {
      counter := counter + 1
    }
  }

  io.pwmOut := ctrl.enable && (counter <= duty.value)

  def svd(name: String, baseAddress: BigInt) = {
    SvdPeripheral(
      busif,
      name,
      baseAddress,
      description = "LCD dim control"
    )
  }
}
