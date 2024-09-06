package slabware

import spinal.core._
import spinal.lib._
import spinal.lib.bus.amba3.apb._

object Apb3LedCtrl{
  def getApb3Config() = Apb3Config(addressWidth = 8, dataWidth = 32)
}

case class Apb3LedCtrl(numLeds: Int) extends Component {
  assert(numLeds <= 32, "numLeds must be <= 32")

  val io = new Bundle {
    val apb  = slave(Apb3(Apb3LedCtrl.getApb3Config()))
    val leds = out Bits(numLeds bits)
  }

  val ctrl = Apb3SlaveFactory(io.apb)
  ctrl.driveAndRead(io.leds, 0)
}
