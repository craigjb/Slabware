package slabware;

import spinal.core._
import spinal.lib._

object LcdSpi {
  case class Word() extends Bundle {
    val isCmd = Bool()
    val data = Bits(8 bits)
  }

  def wordStream = Stream(Word())

  def apply(scl: Bool, sda: Bool, dc: Bool) = {
    val spi = new LcdSpi()
    scl := spi.io.scl
    sda := spi.io.sda
    dc := spi.io.dc
    spi
  }
}

class LcdSpi extends Component {
  import LcdSpi._

  val io = new Bundle {
    val input = slave(wordStream)
    val sda = out Bool ()
    val scl = out Bool ()
    val dc = out Bool ()
  }

  val ready = RegInit(True)
  val shiftReg = RegInit(B(0, 8 bits))
  val counter = RegInit(U(0, 4 bits))
  val isData = RegInit(False)

  val transmitting = counter =/= 0

  // output MSB first
  io.sda := shiftReg.msb
  io.scl := !ClockDomain.current.readClockWire && transmitting
  io.dc := isData
  io.input.ready := ready

  val fsm = new Area {
    when(io.input.ready && io.input.valid) {
      shiftReg := io.input.payload.data
      isData := !io.input.payload.isCmd
      counter := 8
      ready := False
    } elsewhen (counter =/= 0) {
      when(counter === 2) {
        ready := True
      }
      shiftReg := shiftReg |<< 1
      counter := counter - 1
    }
  }
}
