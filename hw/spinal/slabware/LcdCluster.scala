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
  }

  def apply(
      broadcastFifoDepth: Int,
      scl: Bool,
      sda: Bool,
      dc: Bool,
      dsa: Bool,
      dsb: Bool,
      broadcastIn: Flow[Bits],
      frameEnable: Bool
  ): LcdCluster = {
    val lcdCluster = new LcdCluster(broadcastFifoDepth)
    scl := lcdCluster.io.spiBus.scl
    sda := lcdCluster.io.spiBus.sda
    dc := lcdCluster.io.spiBus.dc
    dsa := lcdCluster.io.spiBus.cs(0)
    dsb := lcdCluster.io.spiBus.cs(1)
    lcdCluster.io.broadcastIn << broadcastIn
    lcdCluster.io.frameEnable := frameEnable
    lcdCluster
  }
}

class LcdCluster(broadcastFifoDepth: Int) extends Component {
  import LcdCluster._

  val io = new Bundle {
    val spiBus = out(SpiBus())
    val broadcastBusy = out Bool ()
    val broadcastIn = slave(Flow(Bits(9 bits)))
    val frameEnable = in Bool ()
    val frameDataStream = slave(Stream(Bits(32 bits)))
  }

  io.spiBus.cs.setAsReg().init(0)

  val lcdSpi = LcdSpi(io.spiBus.scl, io.spiBus.sda, io.spiBus.dc)
  val lcdSpiValid = RegInit(False)
  val lcdSpiDataIn = RegInit(B(0, 8 bits))
  val lcdSpiIsCmd = RegInit(False)
  lcdSpi.io.input.valid := lcdSpiValid
  lcdSpi.io.input.payload.data := lcdSpiDataIn
  lcdSpi.io.input.payload.isCmd := lcdSpiIsCmd

  val broadcastFifo = StreamFifo(
    dataType = Bits(9 bits),
    depth = broadcastFifoDepth
  )
  io.broadcastBusy := broadcastFifo.io.availability === 0
  broadcastFifo.io.push << io.broadcastIn.toStream
  val broadcastStream = broadcastFifo.io.pop
  val broadcastPopReady = RegInit(False)
  broadcastStream.ready := broadcastPopReady

  val frameX = RegInit(U(0, 7 bits))
  val frameY = RegInit(U(0, 7 bits))
  val frameDataRem = RegInit(B(0, 24 bits))

  val frameDataReady = RegInit(False)
  io.frameDataStream.ready := frameDataReady

  val fsm = new StateMachine {
    broadcastPopReady := False
    lcdSpiValid := False
    frameDataReady := False

    val idle: State = new State with EntryPoint {
      onEntry {
        broadcastPopReady := True
      }
      whenIsActive {
        when(io.frameEnable) {
          io.spiBus.cs := 0
          goto(frameStartupStates.head)
        } elsewhen (broadcastStream.fire) {
          io.spiBus.cs := 0
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
          when(!lcdSpiValid && io.frameDataStream.fire) {
            frameDataRem := io.frameDataStream.payload(23 downto 0)

            lcdSpiDataIn := io.frameDataStream.payload(31 downto 24)
            lcdSpiIsCmd := False
            lcdSpiValid := True
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
        lcdSpiDataIn := frameDataRem(23 downto 16)
        lcdSpiIsCmd := False
      }
      whenIsActive {
        when(!io.frameEnable) {
          goto(idle)
        } otherwise {
          when(lcdSpi.io.input.fire) {
            goto(frameData2)
          } otherwise {
            lcdSpiValid := True
          }
        }
      }
    }

    val frameData2: State = new State {
      onEntry {
        lcdSpiValid := True
        lcdSpiDataIn := frameDataRem(15 downto 8)
        lcdSpiIsCmd := False
      }
      whenIsActive {
        when(!io.frameEnable) {
          goto(idle)
        } otherwise {
          when(lcdSpi.io.input.fire) {
            goto(frameData3)
          } otherwise {
            lcdSpiValid := True
          }
        }
      }
    }

    val frameData3: State = new State {
      onEntry {
        lcdSpiValid := True
        lcdSpiDataIn := frameDataRem(7 downto 0)
        lcdSpiIsCmd := False
      }
      whenIsActive {
        when(!io.frameEnable) {
          goto(idle)
        } otherwise {
          when(lcdSpi.io.input.fire) {
            frameX := frameX + 2
            when(frameX === LcdWidth - 2) {
              frameY := frameY + 1
            }

            when(
              frameX === LcdWidth - 2 && frameY === LcdHeight - 1
            ) {
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
