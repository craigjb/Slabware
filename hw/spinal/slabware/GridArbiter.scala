package slabware

import spinal.core._
import spinal.lib._

import slabware.hdmirx.HdmiVideo

class GridArbiter(
    val videoClkDomain: ClockDomain,
    spiClusterIndices: Seq[Int],
    displayWidth: Int = 2304,
    displayHeight: Int = 1024,
    hBlankPixels: Int = 2,
    vBlankLines: Int = 8
) extends Component {
  val io = new Bundle {
    val videoIn = slave(Flow(HdmiVideo()))
    val frameDataOut =
      Vec.fill(spiClusterIndices.length)(
        master(Stream(Vec.fill(2)(PixelData())))
      )
    val frameDataEnable = out Bool ()
  }

  import io.videoIn

  val outClkDomain = ClockDomain.current

  val videoClkArea = new ClockingArea(videoClkDomain) {
    val resetFifos = Bool()
    val fifoResetArea = new ResetArea(resetFifos, true) {
      val fifos = spiClusterIndices.map(i => {
        val fifo = StreamFifoCC(
          dataType = Vec.fill(2)(PixelData()),
          depth = 1024,
          pushClock = ClockDomain.current,
          popClock = outClkDomain
        )
        fifo.io.pop >> io.frameDataOut(i - spiClusterIndices.min)
        val pushStream = Stream(Vec.fill(2)(PixelData()))
        pushStream.stage() >> fifo.io.push
        pushStream
      })
    }
    import fifoResetArea.fifos

    val posY = RegInit(
      U(0, log2Up(displayHeight + vBlankLines - 1) bits)
    )
    val posX = RegInit(U(0, log2Up(displayWidth + hBlankPixels - 1) bits))

    val inTopHalf =
      posY >= vBlankLines && posY < vBlankLines + (displayHeight / 2)
    val inBottomHalf = posY >= vBlankLines + (displayHeight / 2)

    when(!videoIn.valid || !videoIn.vSync) { // negative vsync
      posY.setAll()
      posX := 0
    }
    when(!videoIn.valid) {
      posX := 0
    }

    import videoIn.payload._

    when(videoIn.valid && vSync) { // negative vsync polarity
      when(hSync.rise(initAt = False)) {
        posY := posY + 1
      }
      when(hSync) {
        posX := 0
      } otherwise {
        posX := posX + pixelsValid(0).asUInt + pixelsValid(1).asUInt
      }
    }

    val pixelPosX = Seq(posX, posX + pixelsValid(0).asUInt)

    val fifoPushers = spiClusterIndices.map(i =>
      new Area {
        val fifoPush = fifos(i - spiClusterIndices.min)

        val row = i % 2
        val column = i / 2
        val leftX = hBlankPixels + (column * LcdCluster.LcdWidth)
        val rightX = leftX + LcdCluster.LcdWidth

        val pixelInColumn = Vec.fill(2)(Bool())
        for (n <- 0 to 1) {
          fifoPush.payload(n).data := Cat(
            bluePixels(n),
            greenPixels(n),
            redPixels(n)
          )

          pixelInColumn(n) := (pixelPosX(n) >= leftX) && (pixelPosX(n) < rightX)
          fifoPush.payload(n).valid := pixelInColumn(n) && pixelsValid(n)
        }
        val eitherValid =
          fifoPush.payload(0).valid || fifoPush.payload(1).valid
        val inHalf = if (row == 0) {
          inTopHalf
        } else {
          inBottomHalf
        }
        fifoPush.valid := videoIn.valid && eitherValid && inHalf && vSync
      }
    )

    val frameDataEnable = RegNext(videoIn.valid && vSync) init (False)
    resetFifos := !frameDataEnable
  }

  io.frameDataEnable :=
    BufferCC(
      videoClkArea.frameDataEnable,
      randBoot = true,
      init = False
    )
}
