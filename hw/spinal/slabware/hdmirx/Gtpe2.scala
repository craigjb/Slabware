package slabware.hdmirx

import spinal.core._
import spinal.lib._

object IBufDsGte2 {
  def apply(clkIn: HdmiClk, clkOut: Bool, enable: Bool) = {
    val buf = new IBufDsGte2()
    buf.io.I := clkIn.p
    buf.io.IB := clkIn.n
    buf.io.CEB := !enable
    clkOut := buf.io.O
    buf
  }
}

class IBufDsGte2() extends BlackBox {
  val io = new Bundle {
    val I = in Bool ()
    val IB = in Bool ()
    val CEB = in Bool ()
    val O = out Bool ()
    val ODIV2 = out Bool ()
  }

  noIoPrefix()
  setBlackBoxName("IBUFDS_GTE2")
}
