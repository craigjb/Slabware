package slabware.hdmirx

import spinal.core._
import spinal.lib._
import spinal.lib.bus.regif.{AccessType, SymbolName}

import slabware.{BusDefinition, SvdPeripheral}

class HdmiRx[B <: BusDefinition.Bus](
    busDef: BusDefinition[B],
    edidBinPath: String
) extends Component {
  val AddressWidth = 10
  val DataWidth = 32

  val io = new Bundle {
    val bus = slave(busDef.createBus(AddressWidth, DataWidth))
    val hdmi = slave(new HdmiIo())
  }

  val busIf = busDef.createBusInterface(io.bus, (0, 0x100))

  val controlReg = busIf.newReg(doc = "Control").setName("control")
  io.hdmi.hpd := controlReg.field(
    Bool(),
    AccessType.RW,
    resetValue = 0,
    doc = "Hot plug detect enable"
  )(SymbolName("hpdEnable"))

  val statusReg = busIf.newReg(doc = "Status").setName("status")
  statusReg.field(Bool(), AccessType.RO, doc = "Cable detect")(
    SymbolName("cableDetect")
  ) := !io.hdmi.cableDetect

  val hdmiTmdsClk = Bool()
  val hdmiTmdsClkBuf =
    IBufDsGte2(clkIn = io.hdmi.clk, clkOut = hdmiTmdsClk, enable = True)

  val clockDetector = ClockDetector()
  clockDetector.io.hdmiTmdsClk := hdmiTmdsClk
  clockDetector.drive(busIf)

  val edid = new Edid(edidBinPath = edidBinPath)
  io.hdmi.ddc <> edid.io.ddc

  def svd(name: String, baseAddress: BigInt) = {
    SvdPeripheral(
      busIf,
      name,
      baseAddress,
      description = "HDMI Receiver"
    )
  }
}
