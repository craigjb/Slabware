package slabware.hdmirx.sim

import scala.collection.mutable

import spinal.core._
import spinal.core.sim._

import slabware.hdmirx.DiffPair

object HdmiEncoder {
  def CONTROL00 = Integer.parseInt("1101010100", 2)
  def CONTROL01 = Integer.parseInt("0010101011", 2)
  def CONTROL10 = Integer.parseInt("0101010100", 2)
  def CONTROL11 = Integer.parseInt("1010101011", 2)

  def TERC4 = Seq(
    Integer.parseInt("1010011100", 2),
    Integer.parseInt("1001100011", 2),
    Integer.parseInt("1011100100", 2),
    Integer.parseInt("1011100010", 2),
    Integer.parseInt("0101110001", 2),
    Integer.parseInt("0100011110", 2),
    Integer.parseInt("0110001110", 2),
    Integer.parseInt("0100111100", 2),
    Integer.parseInt("1011001100", 2),
    Integer.parseInt("0100111001", 2),
    Integer.parseInt("0110011100", 2),
    Integer.parseInt("1011000110", 2),
    Integer.parseInt("1010001110", 2),
    Integer.parseInt("1001110001", 2),
    Integer.parseInt("0101100011", 2),
    Integer.parseInt("1011000011", 2)
  )

  def bitSeq(data: Int, len: Int): Seq[Boolean] = {
    (0 until len).map(i => (data & (1 << i)) != 0).toSeq
  }

  def encodeControl(c0: Boolean, c1: Boolean): Seq[Boolean] = {
    val encoded = (c1, c0) match {
      case (false, false) => CONTROL00
      case (false, true)  => CONTROL01
      case (true, false)  => CONTROL10
      case (true, true)   => CONTROL11
    }
    bitSeq(encoded, 10)
  }

  def encodeData(data: Int): Seq[Boolean] = {
    bitSeq(TERC4(data), 10)
  }

  def encodePixel(data: Int, disparityCount: Int): (Seq[Boolean], Int) = {
    val dBits = bitSeq(data, 8)
    var qBits = mutable.Seq.fill(10)(false)

    val numOnes = dBits.count(b => b)
    if (numOnes > 4 || (numOnes == 4 && !dBits(0))) {
      // XNOR
      qBits(0) = dBits(0)
      for (i <- 1 to 7) {
        qBits(i) = !(qBits(i - 1) ^ dBits(i))
      }
      qBits(8) = false
    } else {
      // XOR
      qBits(0) = dBits(0)
      for (i <- 1 to 7) {
        qBits(i) = qBits(i - 1) ^ dBits(i)
      }
      qBits(8) = true
    }

    val numQOnes = qBits.take(8).count(b => b)
    val numQZeros = 8 - numQOnes
    if (disparityCount == 0 || numQOnes == numQZeros) {
      if (!qBits(8)) {
        qBits(9) = true
        for (i <- 0 to 7) {
          qBits(i) = !qBits(i)
        }
      }
      val numQOnes = qBits.take(8).count(b => b)
      val numQZeros = 8 - numQOnes
      if (qBits(9)) {
        (qBits, numQZeros - numQOnes)
      } else {
        (qBits, numQOnes - numQZeros)
      }
    } else {
      if (
        (disparityCount > 0 && numQOnes > numQZeros) ||
        (disparityCount < 0 && numQZeros > numQOnes)
      ) {
        qBits(9) = true
        for (i <- 0 to 7) {
          qBits(i) = !qBits(i)
        }
        val numQOnes = qBits.take(8).count(b => b)
        val numQZeros = 8 - numQOnes
        (qBits, (if (qBits(8)) 2 else 0) + (numQZeros - numQOnes))
      } else {
        qBits(9) = false
        val numQOnes = qBits.take(8).count(b => b)
        val numQZeros = 8 - numQOnes
        (qBits, (if (qBits(8)) -2 else 0) + (numQOnes - numQZeros))
      }
    }
  }
}

class HdmiEncoder(
    pair: DiffPair,
    bitClkPeriod: TimeNumber,
    hdmiClk: ClockDomain
) {
  import HdmiEncoder._

  var disparityCount: Int = 0

  def sendBit(b: Boolean) = {
    pair.p #= b
    pair.n #= !b
  }

  def sendBits(encoded: Seq[Boolean]) = {
    hdmiClk.waitRisingEdge()
    for (i <- 0 to 9) {
      sendBit(encoded(i))
      if (i < 9) {
        sleep(bitClkPeriod)
      }
    }
  }

  def sendControl(c0: Boolean, c1: Boolean, count: Int = 1) = {
    val encoded = encodeControl(c0, c1)
    for (_ <- 0 until count) {
      sendBits(encoded)
    }
  }

  def sendData(data: Int) = {
    val encoded = encodeData(data)
    sendBits(encoded)
  }

  def sendPixel(data: Int) = {
    val (encoded, disparityDelta) = encodePixel(data, disparityCount)
    disparityCount += disparityDelta
    sendBits(encoded)
  }
}
