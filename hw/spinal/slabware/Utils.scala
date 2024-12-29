package slabware

import spinal.core._
import java.nio.file.{Files, Paths}

object Utils {
  def read32BitMemFromFile(path: String, wordCount: Int): Seq[Bits] = {
    Files
      .readAllBytes(Paths.get(path))
      .grouped(4)
      .map(w =>
        new BigInt(
          new java.math.BigInteger(
            Array(
              0.toByte,
              0.toByte,
              0.toByte,
              0.toByte,
              w(3),
              w(2),
              w(1),
              w(0)
            )
          )
        )
      )
      .map(word => B(word, 32 bits))
      .toSeq
      .padTo(wordCount, B(0, 32 bits))
  }
}
