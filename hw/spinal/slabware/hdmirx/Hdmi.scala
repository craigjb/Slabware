package slabware.hdmirx

import spinal.core._
import spinal.lib._
import spinal.lib.com.i2c._

case class HdmiIo() extends Bundle with IMasterSlave {
  val clk = DiffPair()
  val channels = Vec.fill(3)(DiffPair())
  val hpd = Bool()
  val cableDetect = Bool()

  override def asMaster(): Unit = {
    out(clk, channels, cableDetect)
    in(hpd)
  }
}

object HdmiPayloadKind extends SpinalEnum {
  val Control, Video = newElement()
}

case class HdmiPayload() extends Bundle {
  val kind = HdmiPayloadKind()
  val c0, c1 = Bool()
  val data = Bits(8 bits)

  def isControl: Bool = {
    kind === HdmiPayloadKind.Control
  }

  def isVideo: Bool = {
    kind === HdmiPayloadKind.Video
  }
}
