package slabware

import spinal.core._
import spinal.lib._
import spinal.lib.bus.regif.AccessType

class SlabGridCtrl[B <: BusDefinition.Bus](
    busDef: BusDefinition[B]
) extends Component {
  val AddressWidth = 8
  val DataWidth = 32

  val io = new Bundle {
    val bus = slave(busDef.createBus(AddressWidth, DataWidth))
    val gridEnable = out Bool ()
  }

  val busIf = busDef.createBusInterface(io.bus, (0, 4))

  val ctrlReg = busIf.newReg(doc = "SlabGrid control").setName("Control")
  val enable = ctrlReg.field(
    Bool(),
    AccessType.RW,
    resetValue = 0,
    doc = "SlabGrid enable"
  )

  io.gridEnable := enable

  def svd(name: String, baseAddress: BigInt) = {
    SvdPeripheral(
      busIf,
      name,
      baseAddress,
      description = "SlabGrid control"
    )
  }
}
