package slabware

import spinal.core._
import spinal.lib._
import spinal.lib.bus.regif.AccessType
import spinal.lib.bus.regif.SymbolName
import spinal.lib.fsm._

class GridDebouncer(
    numColumns: Int,
    sampleCount: Int,
    sampleRate: HertzNumber
) extends Component {
  val io = new Bundle {
    val columnLines = in(Vec.fill(numColumns)(Bool()))
    val restart = in Bool ()
    val done = out Bool ()
    val columnOuts = out(Vec.fill(numColumns)(Bool()))
  }
  io.done.setAsReg() init (False)
  io.columnOuts.foreach(colOut => colOut.setAsReg() init (False))

  val samples = Vec.fill(numColumns)(RegInit(B(0, (sampleCount - 1) bits)))
  val sampleTimer = Timeout(sampleRate)
  val sampleIndex = Counter(sampleCount)

  when(io.restart) {
    io.done := False
    io.columnOuts.foreach(colOut => colOut.clear())
    samples := Vec.fill(numColumns)(B(0, (sampleCount - 1) bits))
    sampleIndex.clear()
    sampleTimer.clear()
  } otherwise {
    when(sampleTimer && !io.done) {
      sampleIndex.increment()

      when(sampleIndex.willOverflow) {
        io.done := True
        io.columnOuts.zip(samples).zip(io.columnLines).foreach {
          case ((colOut, sampleReg), colLine) => {
            colOut := sampleReg.andR && colLine
          }
        }
      } otherwise {
        samples.zip(io.columnLines).foreach {
          case (sampleReg, columnLine) => {
            sampleReg := columnLine ## (sampleReg >> 1)
          }
        }
      }
    }
  }
}

class SlabGridCtrl[B <: BusDefinition.Bus](
    busDef: BusDefinition[B],
    numColumns: Int = 18,
    numRows: Int = 8,
    debounceSampleCount: Int = 10,
    debounceSampleRate: HertzNumber = 2 kHz
) extends Component {
  val AddressWidth = 8
  val DataWidth = 32

  val io = new Bundle {
    val bus = slave(busDef.createBus(AddressWidth, DataWidth))
    val gridEnable = out Bool ()
    val btnRow = out Bits (numRows bits)
    val btnCol = in Bits (numColumns bits)
  }

  val busIf = busDef.createBusInterface(io.bus, (0, 1 kB))

  val scanRate = debounceSampleRate / debounceSampleCount / numRows

  val ctrlRegs = new Area {
    val ctrlReg = busIf.newReg(doc = "SlabGrid control").setName("Control")
    val enable = ctrlReg.field(
      Bool(),
      AccessType.RW,
      resetValue = 0,
      doc = "SlabGrid enable"
    )
    io.gridEnable := enable

    val scanRateReg =
      busIf.newReg(doc = "Grid key scan rate").setName("ScanRate")
    val scanRateInt = scanRate.toDouble.round.toInt
    val rate = scanRateReg.field(
      UInt(log2Up(scanRateInt) bits),
      AccessType.RO,
      doc = f"Key scan rate (${scanRateInt} Hz)"
    )
    rate := scanRateInt
  }

  val colState = Vec.fill(numColumns)(RegInit(B(0, numRows bits)))

  val keyStates =
    (0 until numColumns)
      .map(i =>
        new Area {
          val colReg =
            busIf
              .newReg(doc = f"Column $i key state")
              .setName(f"Col${i}KeyState")
          val state = colReg.field(
            Bits(numRows bits),
            AccessType.RO,
            doc = f"Colum $i key state (row 0 is bit 0)"
          )
          state := colState(i)
        }
      )
      .toSeq

  val debouncer =
    new GridDebouncer(numColumns, debounceSampleCount, debounceSampleRate)
  debouncer.io.columnLines := (~io.btnCol).asBools
  val restartDebounce = RegInit(False)
  debouncer.io.restart := restartDebounce

  val rowOneCold = RegInit(
    B(numRows bits, (numRows - 1) -> false, default -> true)
  )
  io.btnRow := rowOneCold
  val rowOneHot = ~rowOneCold

  val fsm = new StateMachine {
    restartDebounce := False

    val openRow: State = new State with EntryPoint {
      onEntry {
        restartDebounce := True
      }
      whenIsActive {
        rowOneCold := rowOneCold.rotateLeft(1)
        goto(sampleRow)
      }
    }
    val sampleRow: State = new State {
      whenIsActive {
        when(debouncer.io.done) {
          colState.zip(debouncer.io.columnOuts).foreach {
            case (stateReg, colOut) => {
              when(colOut) {
                stateReg := stateReg | rowOneHot
              } otherwise {
                stateReg := stateReg & rowOneCold
              }
            }
          }
          goto(openRow)
        }
      }
    }
  }

  def svd(name: String, baseAddress: BigInt) = {
    SvdPeripheral(
      busIf,
      name,
      baseAddress,
      description = "SlabGrid control"
    )
  }
}
