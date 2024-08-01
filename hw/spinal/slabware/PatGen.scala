package slabware

import spinal.core._
import spinal.lib._

object PatGen {
  val LcdWidth = 128
  val LcdHeight = 128
  val LcdsPerCluster = 4
  val LcdCountX = 18
  val LcdCountY = 8
  val GlobalXBits = log2Up(LcdWidth * LcdCountX)
  val GlobalYBits = log2Up(LcdHeight * LcdCountY)
  val PrescaleTimerBits = 19
  val GridWidth = LcdWidth * LcdCountX
  val GridHeight = LcdHeight * LcdCountY
  val BallSpeed = 2
  val BallSize = 16
}

class PatGen(lcdClusterIndex: Int) extends Component {
  import PatGen._

  val io = new Bundle {
    val frameDataOut = master(Stream(Bits(32 bits)))
  }

  val lcdColIndex = lcdClusterIndex / 2
  val lcdRowIndex = (lcdClusterIndex % 2) * 4
  val leftX = lcdColIndex * LcdWidth
  val topY = lcdRowIndex * LcdHeight

  val dataStream = Stream(Bits(32 bits))

  val prescaleTimer = Counter(bitCount = PrescaleTimerBits bits)
  prescaleTimer.increment()

  val timer = Counter(bitCount = (GlobalXBits + 5) bits)
  when(prescaleTimer.willOverflow) {
    timer.increment()
  }
  val time = timer.value.asBits(0, GlobalXBits bits).asUInt

  // Each cluster is 1x4 LCDs (128x512 pixels)
  val xCounter = Counter(stateCount = 128 / 2)
  val yCounter = Counter(stateCount = 512)

  when(dataStream.fire) {
    xCounter.increment()
    when(xCounter.willOverflow) {
      yCounter.increment()
    }
  }

  val globalX = (xCounter.value << 1).resize(GlobalXBits bits) + leftX
  val globalY = yCounter.value.resize(GlobalYBits bits) + topY

  val xMod0 = (globalX + time) % 64
  val r0 = ((xMod0 >= 32) ? (63 - xMod0) | xMod0)(0, 5 bits)
  val yMod = ((globalY >> 1) + time) % 64
  val b0 = ((yMod >= 32) ? (63 - yMod) | yMod)(0, 5 bits)
  val xyMod0 = (globalX - globalY + time) % 128
  val g0 = ((xyMod0 >= 64) ? (127 - xyMod0) | xyMod0)(0, 6 bits)

  // val b0 = ((((globalX + time) ^ globalY.resized) % 9) << 2).asBits(0, 5 bits);
  // val b0 = globalY.asBits(0, 5 bits)
  // val g0 = B(0, 6 bits)
  // val r0 = (((globalX ^ (globalY.resized + time)) % 5) << 3).asBits(0, 5 bits);
  // val r0 = (globalX + time).asBits(0, 5 bits)

  val xMod1 = (globalX + 1 + time) % 64
  val r1 = ((xMod1 >= 32) ? (63 - xMod1) | xMod1)(0, 5 bits)
  val b1 = b0
  val xyMod1 = (globalX + 1 - globalY + time) % 128
  val g1 = ((xyMod1 >= 64) ? (127 - xyMod1) | xyMod1)(0, 6 bits)

  // val b1 =
  //   ((((globalX + 1 + time) ^ globalY.resized) % 9) << 2).asBits(0, 5 bits);
  // val g1 = B(0, 6 bits)
  // val r1 =
  //   ((((globalX + 1) ^ (globalY.resized + time)) % 5) << 3).asBits(0, 5 bits);

  val ballX = RegInit(U(0, GlobalXBits bits))
  val ballY = RegInit(U(0, GlobalYBits bits))
  val movingRight = RegInit(True)
  val movingDown = RegInit(True)

  when(prescaleTimer.willOverflow) {
    when(movingRight) {
      ballX := ballX + BallSpeed
      when(ballX >= GridWidth - BallSize) {
        movingRight := False
      }
    } otherwise {
      ballX := ballX - BallSpeed
      when(ballX <= BallSpeed) {
        movingRight := True
      }
    }
    when(movingDown) {
      ballY := ballY + BallSpeed
      when(ballY >= GridHeight - BallSize) {
        movingDown := False
      }
    } otherwise {
      ballY := ballY - BallSpeed
      when(ballY <= BallSpeed) {
        movingDown := True
      }
    }
  }

  when(
    globalX >= ballX && globalX < ballX + BallSize &&
      globalY >= ballY && globalY < ballY + BallSize
  ) {
    dataStream.payload := B"32'xFFFFFFFF"
  } otherwise {
    dataStream.payload := b0 ## g0 ## r0 ## b1 ## g1 ## r1
  }

  dataStream.valid := True
  io.frameDataOut << dataStream.stage()
}
