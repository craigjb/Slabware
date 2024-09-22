package slabware

import spinal.core._
import spinal.lib._
import spinal.lib.bus.regif._
import spinal.lib.bus.misc.SizeMapping
import spinal.lib.bus.amba3.apb._
import spinal.lib.bus.amba3.ahblite._

object BusDefinition {
  type Bus = Bundle with IMasterSlave
}

sealed trait BusDefinition[B <: BusDefinition.Bus] {
  def createBus(addressWidth: Int, dataWidth: Int): B
  def createBusInterface(
      bus: B,
      sizeMap: SizeMapping,
      regPre: String = "",
      withSecFireWall: Boolean = false
  ): BusIf
}

object Apb3Bus extends BusDefinition[Apb3] {
  def createBus(addressWidth: Int, dataWidth: Int) = Apb3(
    Apb3Config(addressWidth, dataWidth)
  )
  def createBusInterface(
      bus: Apb3,
      sizeMap: SizeMapping,
      regPre: String = "",
      withSecFireWall: Boolean = false
  ) =
    Apb3BusInterface(bus, sizeMap, regPre, withSecFireWall)
}

object AhbLite3Bus extends BusDefinition[AhbLite3] {
  def createBus(addressWidth: Int, dataWidth: Int) = AhbLite3(
    AhbLite3Config(addressWidth, dataWidth)
  )
  def createBusInterface(
      bus: AhbLite3,
      sizeMap: SizeMapping,
      regPre: String = "",
      withSecFireWall: Boolean = false
  ) =
    AhbLite3BusInterface(bus, sizeMap, regPre, withSecFireWall)
}
