package slabware

import spinal.core._
import java.nio.file.{Files, Paths}

object Utils {
  def read32BitMemFromFile(path: String, memorySize: Int): Seq[Bits] = {
    val bytes = Files.readAllBytes(Paths.get(path))
      .padTo(memorySize, 0.toByte)
    bytes
      .map(b => (b.toInt & 0xff).toLong)
      .grouped(4)
      .map(group => {
        val word = group(0) << 24 |
          group(1) << 16 |
          group(2) << 8 |
          group(3)
        B(word, 32 bits)
      }).toSeq
  }
}
