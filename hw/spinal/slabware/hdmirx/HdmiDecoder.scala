package slabware.hdmirx

import spinal.core._
import spinal.lib._

object HdmiDecoder {
  def on(raw: Bits): HdmiPayload = {
    assert(raw.getWidth == 10)
    val decoder = new HdmiDecoder()
    decoder.io.input := raw
    decoder.io.output
  }
}

class HdmiDecoder extends Component {
  val io = new Bundle {
    val input = in Bits (10 bits)
    val output = out(HdmiPayload())
  }

  when(io.input === B"10'1101010100") {
    io.output.kind := HdmiPayloadKind.Control
    io.output.c0 := False
    io.output.c1 := False
    io.output.data := B"8'0"
  } elsewhen (io.input === B"10'0010101011") {
    io.output.kind := HdmiPayloadKind.Control
    io.output.c0 := True
    io.output.c1 := False
    io.output.data := B"8'0"
  } elsewhen (io.input === B"10'0101010100") {
    io.output.kind := HdmiPayloadKind.Control
    io.output.c0 := False
    io.output.c1 := True
    io.output.data := B"8'0"
  } elsewhen (io.input === B"10'1010101011") {
    io.output.kind := HdmiPayloadKind.Control
    io.output.c0 := True
    io.output.c1 := True
    io.output.data := B"8'0"
  } otherwise {
    io.output.kind := HdmiPayloadKind.Video
    io.output.c0 := False
    io.output.c1 := False

    val flippedOrNot = Bits(10 bits)
    when(io.input(9)) {
      flippedOrNot := ~io.input
    } otherwise {
      flippedOrNot := io.input
    }
    when(io.input(8)) {
      // XOR
      io.output.data(0) := flippedOrNot(0)
      for (i <- 1 to 7) {
        io.output.data(i) := flippedOrNot(i) ^ flippedOrNot(i - 1)
      }
    } otherwise {
      // XNOR
      io.output.data(0) := flippedOrNot(0)
      for (i <- 1 to 7) {
        io.output.data(i) := ~(flippedOrNot(i) ^ flippedOrNot(i - 1))
      }
    }
  }
}
