package slabware

import spinal.core._
import spinal.lib.io._
import spinal.lib.blackbox.xilinx.s7.IOBUF

object OpenDrainBuffer {
  def apply(io: ReadableOpenDrain[Bool]): Bool = {
    val buf = IOBUF()
    buf.I := io.write
    buf.T := io.write
    io.read := buf.O
    buf.IO
  }
}
