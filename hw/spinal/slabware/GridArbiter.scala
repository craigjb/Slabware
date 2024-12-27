package slabware

import spinal.core._
import spinal.lib._

import slabware.hdmirx.HdmiVideo

class GridArbiter(
    val videoClkDomain: ClockDomain,
    displayWidth: Int = 2304,
    displayHeight: Int = 1024,
    vBlankLines: Int = 8,
    numSpiClusters: Int = 36
) extends Component {
  val io = new Bundle {
    val videoIn = slave(Flow(HdmiVideo()))
    val frameDataOut =
      Vec.fill(numSpiClusters)(master(Stream(Vec.fill(2)(PixelData()))))
    val frameDataEnable = out Bool ()
  }

  val outClkDomain = ClockDomain.current

  val videoClkArea = new ClockingArea(videoClkDomain) {
    val resetFifos = Bool()
    val fifoResetArea = new ResetArea(resetFifos, true) {
      val fifos = (0 until numSpiClusters).map(i => {
        val fifo = StreamFifoCC(
          dataType = Vec.fill(2)(PixelData()),
          depth = 4 * 1024,
          pushClock = ClockDomain.current,
          popClock = outClkDomain
        )
        fifo.io.pop >> io.frameDataOut(i)
        fifo
      })
    }
    import fifoResetArea.fifos

    val posY = RegInit(
      U(0, log2Up((displayHeight + vBlankLines) - 1) bits)
    )
    val posX = RegInit(U(0, log2Up(displayWidth - 1) bits))

    val inTopHalf =
      posY >= vBlankLines && posY < vBlankLines + (displayHeight / 2)
    val inBottomHalf = posY >= vBlankLines + (displayHeight / 2)

    when(!io.videoIn.valid || !io.videoIn.vSync) { // negative vsync
      posY.setAll()
      posX := 0
    }
    when(!io.videoIn.valid) {
      posX := 0
    }

    import io.videoIn.payload._

    when(io.videoIn.valid && vSync) { // negative vsync polarity
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

    val fifoPushers = (0 until numSpiClusters).map(i =>
      new Area {
        val fifo = fifos(i)

        val row = i % 2
        val column = i / 2
        val leftX = column * LcdCluster.LcdWidth
        val rightX = leftX + LcdCluster.LcdWidth

        val pixelInColumn = Vec.fill(2)(Bool())
        for (n <- 0 to 1) {
          fifo.io.push.payload(n).data := Cat(
            bluePixels(n)(7 downto 3),
            greenPixels(n)(7 downto 2),
            redPixels(n)(7 downto 3)
          )

          pixelInColumn(n) := (pixelPosX(n) >= leftX) && (pixelPosX(n) < rightX)
          fifo.io.push.payload(n).valid := pixelInColumn(n) && pixelsValid(n)
        }
        val eitherValid =
          fifo.io.push.payload(0).valid || fifo.io.push.payload(1).valid
        val inHalf = if (row == 0) {
          inTopHalf
        } else {
          inBottomHalf
        }
        fifo.io.push.valid := io.videoIn.valid && eitherValid && inHalf && vSync
      }
    )

    val frameDataEnable = RegNext(io.videoIn.valid && vSync) init (False)
    resetFifos := !frameDataEnable
  }

  io.frameDataEnable :=
    BufferCC(
      videoClkArea.frameDataEnable,
      randBoot = true,
      init = False
    )
}
