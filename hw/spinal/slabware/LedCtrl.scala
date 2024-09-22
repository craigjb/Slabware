package slabware

import spinal.core._
import spinal.lib._
import spinal.lib.bus.misc.SizeMapping
import spinal.lib.bus.regif.AccessType

class LedCtrl[B <: BusDefinition.Bus](
    busDef: BusDefinition[B],
    numLeds: Int
) extends Component {
  assert(numLeds <= 32, "numLeds must be <= 32")

  val AddressWidth = 8
  val DataWidth = 32

  val io = new Bundle {
    val bus = slave(busDef.createBus(AddressWidth, DataWidth))
    val leds = out Bits (numLeds bits)
  }

  val busif = busDef.createBusInterface(io.bus, (0, 4))

  val ctrlReg = busif
    .newRegAt(0x0, doc = "LED output control")
    .setName("CTRL")
  val value =
    ctrlReg.fieldAt(
      0,
      Bits(numLeds bits),
      AccessType.RW,
      doc = "LED output value"
    )
  io.leds := value

  def svd(name: String, baseAddress: BigInt) = {
    SvdPeripheral(
      busif,
      name,
      baseAddress,
      description = "LED control"
    )
  }
}
