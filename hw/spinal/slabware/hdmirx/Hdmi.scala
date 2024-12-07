package slabware.hdmirx

import spinal.core._
import spinal.lib._
import spinal.lib.com.i2c._

object HdmiClk {
  def apply(clkP: Bool, clkN: Bool) = {
    val clk = new HdmiClk()
    clk.p := clkP
    clk.n := clkN
    clk
  }
}

class HdmiClk() extends Bundle {
  val p = Bool()
  val n = Bool()
}

object HdmiIo {
  def apply(
      clk: HdmiClk,
      hpd: Bool,
      cableDetect: Bool
  ) = {
    val hdmi = new HdmiIo()
    hdmi.clk := clk
    hpd := hdmi.hpd
    hdmi.cableDetect := cableDetect
    hdmi
  }
}

class HdmiIo() extends Bundle with IMasterSlave {
  val clk = new HdmiClk()
  val hpd = Bool()
  val cableDetect = Bool()
  val ddc = I2c()

  override def asMaster(): Unit = {
    out(clk, cableDetect)
    in(hpd)
    slave(ddc)
  }
}
