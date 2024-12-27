package slabware.hdmirx.test

import org.scalatest.funsuite.AnyFunSuite

import slabware.hdmirx.sim.HdmiEncoder

class HdmiEncoderTestSuite extends AnyFunSuite {
  def decodeVideo(bits: Seq[Boolean]): Int = {
    val flipped = if (bits(9)) {
      bits.take(8).map(b => !b) ++ bits.slice(8, 10)
    } else {
      bits
    }
    var value = 0
    if (flipped(0)) {
      value |= 1
    }
    if (flipped(8)) {
      // XOR
      for (i <- 1 to 7) {
        if (flipped(i) ^ flipped(i - 1)) {
          value |= (1 << i)
        }
      }
    } else {
      // XNOR
      for (i <- 1 to 7) {
        if (!(flipped(i) ^ flipped(i - 1))) {
          value |= (1 << i)
        }
      }
    }
    value
  }

  test("Control tokens") {
    assert(
      HdmiEncoder.encodeControl(false, false) ==
        Seq(false, false, true, false, true, false, true, false, true, true)
    )
    assert(
      HdmiEncoder.encodeControl(false, true) ==
        Seq(false, false, true, false, true, false, true, false, true, false)
    )
    assert(
      HdmiEncoder.encodeControl(true, false) ==
        Seq(true, true, false, true, false, true, false, true, false, false)
    )
    assert(
      HdmiEncoder.encodeControl(true, true) ==
        Seq(true, true, false, true, false, true, false, true, false, true)
    )
  }

  test("Video tokens no disparity") {
    for (i <- 0 to 255) {
      assert(decodeVideo(HdmiEncoder.encodePixel(i, 0)._1) == i)
    }
  }

  test("Video tokens with disparity") {
    var disparity = 0
    for (i <- 0 to 2304) {
      val (encoded, delta) = HdmiEncoder.encodePixel(i % 256, disparity)
      disparity += delta
      assert(decodeVideo(encoded) == i % 256)
    }
    for (i <- (0 to 2304).reverse) {
      val (encoded, delta) = HdmiEncoder.encodePixel(i % 256, disparity)
      disparity += delta
      println(disparity)
      assert(decodeVideo(encoded) == i % 256)
    }
  }
}
