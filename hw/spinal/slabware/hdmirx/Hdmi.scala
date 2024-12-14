package slabware.hdmirx

import spinal.core._
import spinal.lib._
import spinal.lib.com.i2c._

case class HdmiIo(
    invertD0: Boolean = false
) extends Bundle
    with IMasterSlave {
  val clk = new DiffPair()
  val channel0 = new DiffPair(invertPolarity = invertD0)
  val hpd = Bool()
  val cableDetect = Bool()

  override def asMaster(): Unit = {
    out(clk, channel0, cableDetect)
    in(hpd)
  }
}
