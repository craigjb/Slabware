package slabware

import spinal.core._
import spinal.lib._
import spinal.lib.fsm._

object SlabGrid {
  val LcdPixelFormatSetCmd = 0x3a
  val LcdPixelFormat16Bits = 0x05
  val LcdDisplayInversionOnCmd = 0x21
  val LcdMemoryAccessCtrlCmd = 0x36
  val LcdMemoryAccessInvertXY = 0xc0
  val LcdSleepOutCmd = 0x11
  val LcdDisplayOnCmd = 0x29

  sealed abstract class LcdStep {}
  object LcdStep {
    final case class Wait(rst: Boolean = true) extends LcdStep
    final case class Cmd(c: Int) extends LcdStep {}
    final case class Data(d: Int) extends LcdStep
    final case class FrameData() extends LcdStep
  }

  val LcdStartupSequence = Seq(
    LcdStep.Wait(rst = false),
    LcdStep.Wait(),
    LcdStep.Cmd(LcdPixelFormatSetCmd),
    LcdStep.Data(LcdPixelFormat16Bits),
    LcdStep.Cmd(LcdMemoryAccessCtrlCmd),
    LcdStep.Data(LcdMemoryAccessInvertXY),
    LcdStep.Cmd(LcdDisplayInversionOnCmd),
    LcdStep.Cmd(LcdSleepOutCmd),
    LcdStep.Wait(),
    LcdStep.Cmd(LcdDisplayOnCmd),
    LcdStep.Wait(),
    LcdStep.FrameData
  )

  def apply(
      lcdReset: Bool,
      scl: Bits,
      sda: Bits,
      dc: Bits,
      dsa: Bits,
      dsb: Bits,
      numSpiClusters: Int = 18
  ): SlabGrid = {
    val grid = new SlabGrid(numSpiClusters)
    lcdReset := grid.io.lcdReset

    Range(0, numSpiClusters).foreach(index => {
      grid.io
        .spiBus(index)
        .drive(scl(index), sda(index), dc(index), dsa(index), dsb(index))
    })

    grid
  }
}

class SlabGrid(
    numSpiClusters: Int = 18,
    waitTime: TimeNumber = 120 ms
) extends Component {
  import SlabGrid._

  val io = new Bundle {
    val lcdReset = out Bool ()
    val spiBus = Vec(out(LcdCluster.SpiBus()), numSpiClusters)
  }
  val reset = RegInit(False)
  io.lcdReset := reset

  val broadcastPayload = RegInit(B"9'0")
  val broadcastValid = RegInit(False)
  val broadcastStream = Stream(Bits(9 bits))
  broadcastStream.valid := broadcastValid
  broadcastStream.payload := broadcastPayload

  val broadcastForks =
    StreamFork(broadcastStream, portCount = numSpiClusters, synchronous = true)

  val patGens = Range(0, numSpiClusters).map(i => new PatGen(i))

  val frameDataEnable = RegInit(False)
  val lcdClusters = io.spiBus
    .zip(broadcastForks)
    .zip(patGens)
    .map {
      case ((spiBus, broadcastFork), patGen) => {
        val lcdCluster = new LcdCluster()
        spiBus := lcdCluster.io.spiBus
        lcdCluster.io.frameEnable := frameDataEnable
        lcdCluster.io.broadcastIn << broadcastFork
        lcdCluster.io.frameDataStream << patGen.io.frameDataOut
      }
    }

  val fsm = new StateMachine {
    reset := True
    broadcastValid := False

    val timeout = Timeout(waitTime)
    val startupStates = LcdStartupSequence.map(step => new State)
    setEntry(startupStates(0))

    startupStates.zipWithIndex.foreach {
      case (state, index) => {
        LcdStartupSequence(index) match {
          case LcdStep.Wait(rst) => {
            state.onEntry {
              timeout.clear()
            }
            state.whenIsActive {
              reset := Bool(rst)
              if (index < startupStates.length - 1) {
                when(timeout) {
                  goto(startupStates(index + 1))
                }
              }
            }
          }
          case LcdStep.Cmd(c) => {
            state.onEntry {
              broadcastPayload(7 downto 0) := c
              broadcastPayload(8) := True
              broadcastValid := True
            }
            state.whenIsActive {
              broadcastValid := True
              if (index < startupStates.length - 1) {
                when(broadcastStream.fire) {
                  goto(startupStates(index + 1))
                }
              }
            }
          }
          case LcdStep.Data(d) => {
            state.onEntry {
              broadcastPayload(7 downto 0) := d
              broadcastPayload(8) := False
              broadcastValid := True
            }
            state.whenIsActive {
              broadcastValid := True
              if (index < startupStates.length - 1) {
                when(broadcastStream.fire) {
                  goto(startupStates(index + 1))
                }
              }
            }
          }
          case LcdStep.FrameData => {
            state.onEntry {
              frameDataEnable := False
            }
          }
        }
      }
    }
  }
}
