package slabware.hdmirx

import java.nio.file.{Files, Paths}
import java.lang.Byte.toUnsignedInt

import spinal.core._
import spinal.lib._
import spinal.lib.com.i2c._
import spinal.lib.fsm._

class Edid(address: Int = 0x50, edidBinPath: String = null) extends Component {
  val io = new Bundle() {
    val ddc = master(I2c())
  }

  val EdidSize = 256
  val edidMem = Mem(Bits(8 bits), EdidSize)
  val edidBytes = if (edidBinPath != null) {
    Files
      .readAllBytes(Paths.get(edidBinPath))
      .map(b => toUnsignedInt(b))
      .toSeq
  } else {
    (0 until EdidSize).toSeq
  }
  assert(edidBytes.length == EdidSize, f"Invalid EDID size in: $edidBinPath")
  edidMem.init(edidBytes.map(b => B(b, 8 bits)))

  val bridge = new Area {
    val i2cSlave = new I2cSlave(I2cSlaveGenerics())
    import i2cSlave.io._

    config.samplingClockDivider := 5
    config.timeout := 0
    config.tsuData := 0
    config.timeoutClear := False

    val i2cBuffer = I2c()
    i2cBuffer <> i2c

    val rxData = new Area {
      val event = RegNext(False) init (False)
      val value = RegInit(B(0, 8 bits))
    }

    val rxAck = new Area {
      val event = RegNext(False) init (False)
      val value = Reg(Bool())
    }

    val txData = new Area {
      val value = RegInit(B(0, 8 bits))
      val enable = RegInit(False)
    }

    val txAck = new Area {
      val value = RegInit(True)
    }

    val frameReset = False
    val startOrRestart = False
    val dataCounter = RegInit(U"000")
    val inAckState = RegInit(False)
    val wasntAck = RegInit(False)

    when(!inAckState) {
      bus.rsp.valid := bus.cmd.kind === I2cSlaveCmdMode.DRIVE
      bus.rsp.enable := txData.enable
      bus.rsp.data := txData.value(7 - dataCounter)
    } otherwise {
      bus.rsp.valid := bus.cmd.kind === I2cSlaveCmdMode.DRIVE
      bus.rsp.enable := True
      bus.rsp.data := txAck.value
    }

    when(wasntAck) {
      bus.rsp.valid := bus.cmd.kind === I2cSlaveCmdMode.DRIVE
      bus.rsp.enable := False
    }

    switch(bus.cmd.kind) {
      is(I2cSlaveCmdMode.START) {
        frameReset := True
        startOrRestart := True
      }
      is(I2cSlaveCmdMode.RESTART) {
        frameReset := True
        startOrRestart := True
      }
      is(I2cSlaveCmdMode.STOP) {
        frameReset := True
      }
      is(I2cSlaveCmdMode.DROP) {
        frameReset := True
      }
      is(I2cSlaveCmdMode.READ) {
        when(!inAckState) {
          rxData.value(7 - dataCounter) := bus.cmd.data
          dataCounter := dataCounter + 1

          when(dataCounter === 7) {
            rxData.event := True
            inAckState := True
          }
        } otherwise {
          rxAck.event := True
          rxAck.value := bus.cmd.data
          inAckState := False
          wasntAck := bus.cmd.data
        }
      }
    }

    when(frameReset) {
      inAckState := False
      dataCounter := 0
      wasntAck := False
    }
  }

  io.ddc.scl.write := RegNext(bridge.i2cBuffer.scl.write) init (True)
  io.ddc.sda.write := RegNext(bridge.i2cBuffer.sda.write) init (True)
  bridge.i2cBuffer.scl.read := io.ddc.scl.read
  bridge.i2cBuffer.sda.read := io.ddc.sda.read

  val fsm = new StateMachine {
    val Nack = True
    val Ack = False

    val addressHit = bridge.rxData.value(7 downto 1) === B(address, 7 bits)
    val addressRead = bridge.rxData.value(0)
    val offset = RegInit(U(0, 8 bits))

    val waitForStart: State = new State with EntryPoint {
      whenIsActive {
        when(bridge.startOrRestart) {
          goto(readAddress)
        }
      }
    }

    val readAddress: State = new State {
      whenIsActive {
        when(bridge.frameReset) {
          goto(waitForStart)
        } elsewhen (bridge.rxData.event) {
          when(addressHit) {
            bridge.txAck.value := Ack
            when(addressRead) {
              goto(writeData)
            } otherwise {
              goto(readOffset)
            }
          } otherwise {
            bridge.txAck.value := Nack
            goto(waitForStart)
          }
        }
      }
    }

    val readOffset: State = new State {
      whenIsActive {
        when(bridge.frameReset) {
          goto(waitForStart)
        } elsewhen (bridge.rxData.event) {
          offset := bridge.rxData.value.asUInt
          bridge.txAck.value := Ack
          goto(waitForStart)
        }
      }
    }

    val writeData: State = new State {
      whenIsActive {
        when(bridge.frameReset) {
          goto(waitForStart)
        } elsewhen (bridge.rxAck.event || bridge.txData.enable) {
          bridge.txData.enable := True
          bridge.txData.value := edidMem(offset)
          bridge.txAck.value := Nack
          offset := offset + 1
          goto(checkDataAck)
        }
      }
    }

    val checkDataAck: State = new State {
      whenIsActive {
        when(bridge.frameReset) {
          goto(waitForStart)
        } elsewhen (bridge.rxAck.event) {
          when(bridge.rxAck.value === Ack) {
            goto(writeData)
          } otherwise {
            bridge.txData.enable := False
            goto(waitForStart)
          }
        }
      }
    }
  }
}
