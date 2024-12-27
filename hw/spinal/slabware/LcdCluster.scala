package slabware

import spinal.core._
import spinal.lib._
import spinal.lib.fsm._

object LcdCluster {
  val LcdWidth = 128
  val LcdHeight = 128

  val ColumnOffset = 2
  val RowOffset = 1

  val LcdColAddressCmd = 0x2a
  val LcdRowAddressCmd = 0x2b
  val LcdWriteCmd = 0x2c

  sealed abstract class LcdWord
  object LcdWord {
    final case class Cmd(c: Int) extends LcdWord
    final case class Data(d: Int) extends LcdWord
  }

  val FrameStartupSequence = Seq(
    LcdWord.Cmd(LcdColAddressCmd),
    LcdWord.Data(0x00),
    LcdWord.Data(ColumnOffset),
    LcdWord.Data(0x00),
    LcdWord.Data(LcdWidth + ColumnOffset - 1),
    LcdWord.Cmd(LcdRowAddressCmd),
    LcdWord.Data(0x00),
    LcdWord.Data(RowOffset),
    LcdWord.Data(0x00),
    LcdWord.Data(LcdHeight + RowOffset - 1),
    LcdWord.Cmd(LcdWriteCmd)
  )

  case class SpiBus() extends Bundle {
    val cs = UInt(2 bits)
    val scl = Bool()
    val sda = Bool()
    val dc = Bool()

    def drive(
        ioScl: Bool,
        ioSda: Bool,
        ioDc: Bool,
        ioDsa: Bool,
        ioDsb: Bool
    ) = {
      ioScl := scl
      ioSda := sda
      ioDc := dc
      ioDsa := cs(0)
      ioDsb := cs(1)
    }
  }
}

case class PixelData() extends Bundle {
  val valid = Bool()
  val data = Bits(16 bits)
}

class LcdCluster() extends Component {
  import LcdCluster._

  val io = new Bundle {
    val spiBus = out(SpiBus())
    val broadcastIn = slave(Stream(Bits(9 bits)))
    val frameEnable = in Bool ()
    val frameDataStream = slave(Stream(Vec.fill(2)(PixelData())))
  }

  io.spiBus.cs.setAsReg().init(0)

  val lcdSpi = LcdSpi(io.spiBus.scl, io.spiBus.sda, io.spiBus.dc)
  val lcdSpiValid = RegInit(False)
  val lcdSpiDataIn = RegInit(B(0, 8 bits))
  val lcdSpiIsCmd = RegInit(False)
  lcdSpi.io.input.valid := lcdSpiValid
  lcdSpi.io.input.payload.data := lcdSpiDataIn
  lcdSpi.io.input.payload.isCmd := lcdSpiIsCmd

  val broadcastStream = io.broadcastIn
  val broadcastPopReady = RegInit(False)
  broadcastStream.ready := broadcastPopReady

  val frameX = RegInit(U(0, 7 bits))
  val frameY = RegInit(U(0, 7 bits))
  val pixelDataRem = RegInit(B(0, 8 bits))

  val singlePixelStream = Stream(PixelData())
  val adapter = StreamWidthAdapter(
    io.frameDataStream,
    singlePixelStream,
    order=LOWER_FIRST
  )

  val frameDataReady = RegInit(False)
  singlePixelStream.ready := frameDataReady

  val fsm = new StateMachine {
    broadcastPopReady := False
    lcdSpiValid := False
    frameDataReady := False

    val idle: State = new State with EntryPoint {
      onEntry {
        broadcastPopReady := True
        io.spiBus.cs := 0
        frameX := 0
        frameY := 0
        frameDataReady := True
      }
      whenIsActive {
        frameDataReady := True
        when(io.frameEnable && !io.broadcastIn.valid) {
          goto(frameStartupStates.head)
        } elsewhen (broadcastStream.fire) {
          lcdSpiDataIn := broadcastStream.payload(7 downto 0)
          lcdSpiIsCmd := broadcastStream.payload(8)
          goto(broadcastLoad)
        } otherwise {
          broadcastPopReady := True
        }
      }
    }

    val broadcastLoad: State = new State {
      onEntry {
        lcdSpiValid := True
      }
      whenIsActive {
        when(lcdSpi.io.input.fire) {
          goto(broadcastWait)
        }
      }
    }

    val broadcastWait: State = new State {
      whenIsActive {
        when(lcdSpi.io.input.ready) {
          goto(broadcastNext)
        }
      }
    }

    val broadcastNext: State = new State {
      whenIsActive {
        when(io.spiBus.cs === 3) {
          goto(idle)
        } otherwise {
          io.spiBus.cs := io.spiBus.cs + 1
          goto(broadcastLoad)
        }
      }
    }

    val frameStartupStates = FrameStartupSequence.map(word => {
      new State {
        onEntry {
          word match {
            case LcdWord.Cmd(c) => {
              lcdSpiDataIn := c
              lcdSpiIsCmd := True
            }
            case LcdWord.Data(d) => {
              lcdSpiDataIn := d
              lcdSpiIsCmd := False
            }
          }
          lcdSpiValid := True
        }
      }
    })
    frameStartupStates.head.onEntry {
      frameX := 0
      frameY := 0
    }
    frameStartupStates
      .sliding(2, 1)
      .foreach(prevAndNext => {
        prevAndNext(0).whenIsActive {
          when(!io.frameEnable) {
            goto(idle)
          } elsewhen (lcdSpi.io.input.fire) {
            goto(prevAndNext(1))
          } otherwise {
            lcdSpiValid := True
          }
        }
      })
    frameStartupStates.last.whenIsActive {
      when(!io.frameEnable) {
        goto(idle)
      } elsewhen (lcdSpi.io.input.fire) {
        goto(frameData0)
      } otherwise {
        lcdSpiValid := True
      }
    }

    val frameData0: State = new State {
      onEntry {
        frameDataReady := True
        lcdSpiValid := False
      }
      whenIsActive {
        when(!io.frameEnable) {
          goto(idle)
        } otherwise {
          when(!lcdSpiValid && singlePixelStream.fire) {
            lcdSpiDataIn := singlePixelStream.payload.data(15 downto 8)
            pixelDataRem := singlePixelStream.payload.data(7 downto 0)

            when(singlePixelStream.payload.valid) {
              lcdSpiIsCmd := False
              lcdSpiValid := True
            } otherwise {
              goto(frameData0)
            }
          } elsewhen (lcdSpiValid) {
            when(lcdSpi.io.input.fire) {
              goto(frameData1)
            } otherwise {
              lcdSpiValid := True
            }
          } otherwise {
            frameDataReady := True
          }
        }
      }
    }

    val frameData1: State = new State {
      onEntry {
        lcdSpiValid := True
        lcdSpiDataIn := pixelDataRem
        lcdSpiIsCmd := False
      }
      whenIsActive {
        when(!io.frameEnable) {
          goto(idle)
        } otherwise {
          when(lcdSpi.io.input.fire) {
            frameX := frameX + 1
            val atLineEnd = (frameX === LcdWidth - 1)
            when(atLineEnd) {
              frameY := frameY + 1
            }

            when(atLineEnd && (frameY === LcdHeight - 1)) {
              goto(frameNextCs)
            } otherwise {
              goto(frameData0)
            }
          } otherwise {
            lcdSpiValid := True
          }
        }
      }
    }

    val frameNextCs: State = new State {
      whenIsActive {
        when(!io.frameEnable) {
          goto(idle)
        } otherwise {
          when(lcdSpi.io.input.ready) {
            io.spiBus.cs := io.spiBus.cs + 1
            goto(frameStartupStates.head)
          }
        }
      }
    }
  }
}
