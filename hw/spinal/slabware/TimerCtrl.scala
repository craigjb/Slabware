package slabware

import spinal.core._
import spinal.lib._
import spinal.lib.bus.regif.SymbolName
import spinal.lib.misc.Prescaler
import spinal.lib.bus.misc.SizeMapping
import spinal.lib.bus.regif.AccessType

class TimerCtrl[B <: BusDefinition.Bus](
    busDef: BusDefinition[B],
    timerWidth: Int = 16,
    prescaleWidth: Int = 16,
    numCompares: Int = 2
) extends Component {
  assert(timerWidth <= 32, "timerWidth must be <= 32")
  assert(prescaleWidth <= 32, "prescaleWidth must be <= 32")

  val AddressWidth = 8
  val DataWidth = 32

  val io = new Bundle {
    val bus = slave(busDef.createBus(AddressWidth, DataWidth))
    val interrupt = out Bool ()
  }

  val busif = busDef.createBusInterface(io.bus, (0, 0x400))

  val prescaler = new Area {
    val prescaleReg = busif
      .newRegAt(0x4, doc = "Prescale")
      .setName("prescale")
    val value = prescaleReg.field(
      UInt(prescaleWidth bits),
      AccessType.WO,
      resetValue = 0,
      doc = "Timer prescale divisor"
    )

    val inner = Prescaler(prescaleWidth)
    inner.io.limit := value
    val out = inner.io.overflow
  }

  val controlReg = busif
    .newReg(doc = "Control")
    .setName("control")
  val enable = controlReg.field(
    Bool(),
    AccessType.RW,
    resetValue = 0,
    doc = "Timer enable"
  )
  val clear = controlReg.field(
    Bool(),
    AccessType.WO,
    resetValue = 0,
    doc = "Clear prescaler and counter"
  )
  prescaler.inner.io.clear := clear
  val interruptEnable = controlReg.field(
    Bool(),
    AccessType.RW,
    resetValue = 0,
    doc = "Interrupt enable"
  )

  val counterReg = busif
    .newReg(doc = "Counter")
    .setName("counter")
  val counter = counterReg.field(
    UInt(timerWidth bits),
    AccessType.RW,
    resetValue = 0,
    doc = "Counter value"
  )

  val overflow = enable && (counter === U(0, timerWidth bits) - 1)
  when(enable && prescaler.out) {
    counter := counter + 1
  } elsewhen (overflow) {
    counter := 0
  }
  when(clear) {
    counter := 0
    clear := False
  }

  val interruptStatus = busif
    .newReg(doc = "Interrupt status")
    .setName("interruptStatus")
  val overflowStatus = interruptStatus.field(
    Bool(),
    AccessType.W1C,
    resetValue = 0,
    doc = "Overflow interrupt status (set to clear)"
  )
  overflowStatus.setWhen(overflow)

  val interruptMask = busif
    .newReg(doc = "Interrupt mask")
    .setName("interruptMask")
  val overflowMask = interruptMask.field(
    Bool(),
    AccessType.RW,
    resetValue = 0,
    doc = "Mask overflow interrupt"
  )

  val compares = (0 until numCompares).map(i => {
    new Area {
      val reg = busif
        .newReg(doc = f"Compare $i")
        .setName(f"compare$i")
      val value = reg.field(
        UInt(timerWidth bits),
        AccessType.RW,
        resetValue = 0,
        doc = f"Compare $i value"
      )
      val fire = enable && (counter === value)

      val status = interruptStatus
        .field(
          Bool(),
          AccessType.W1C,
          resetValue = 0,
          doc = f"Compare$i interrupt status (set to clear)"
        )(SymbolName(f"compare${i}Status"))
      status.setWhen(fire)

      val mask = interruptMask.field(
        Bool(),
        AccessType.RW,
        resetValue = 0,
        doc = f"Mask compare$i interrupt"
      )(SymbolName(f"compare${i}Mask"))

      val maskedFire = fire && !mask
    }.setName(f"compare$i")
  })

  val maskedCompareFire =
    compares.foldLeft(False)((acc, x) => acc || x.maskedFire)
  io.interrupt := interruptEnable && maskedCompareFire

  def svd(name: String, baseAddress: BigInt) = {
    SvdPeripheral(
      busif,
      name,
      baseAddress,
      description = "Timer"
    )
  }
}
