package slabware

import spinal.core._
import spinal.lib._
import spinal.lib.fsm.{EntryPoint, State, StateMachine}
import spinal.lib.com.i2c._
import spinal.lib.bus.regif.AccessType

class I2cCtrl[B <: BusDefinition.Bus](
    busDef: BusDefinition[B],
    generics: I2cSlaveMemoryMappedGenerics
) extends Component {

  val AddressWidth = 8
  val DataWidth = 32

  val io = new Bundle {
    val bus = slave(busDef.createBus(AddressWidth, DataWidth))
    val i2c = master(I2c())
  }

  val i2cCtrl = new I2cSlave(generics.ctrlGenerics)
  val busif = busDef.createBusInterface(io.bus, (0, 0x400))

  val bridge = new Area {
    import generics._
    import i2cCtrl.io._

    val frameReset = False

    val i2cBuffer = I2c()
    i2cBuffer <> i2c

    val rxData = new Area {
      val event = RegNext(False) init (False)

      val RXDATA = busif
        .newRegAt(0x08, doc = "Receive data")
        .setName("rxData")
      val value = RXDATA.fieldAt(
        0,
        Bits(8 bits),
        AccessType.RO,
        doc = "Receive data value"
      )
      val valueReg = Reg(Bits(8 bits))
      value := valueReg
      val valid = RXDATA.fieldAt(
        8,
        Bool(),
        AccessType.RC,
        resetValue = 0,
        doc = "Receive data valid (cleared on read)"
      )
      val listen =
        RXDATA.fieldAt(
          9,
          Bool(),
          AccessType.WO,
          resetValue = 0,
          doc = "Listen for receive data"
        )
    }

    val rxAck = new Area {
      val RXACK = busif
        .newRegAt(0x0c, doc = "Receive acknowledge")
        .setName("rxAck")
      val value = RXACK.fieldAt(
        0,
        Bool(),
        AccessType.RO,
        doc = "Receive acknowledge value"
      )
      val valueReg = Reg(Bool())
      value := valueReg
      val valid = RXACK.fieldAt(
        8,
        Bool(),
        AccessType.RC,
        resetValue = 0,
        doc = "Receive acknowledge valid (cleared on read)"
      )
      val listen = RXACK.fieldAt(
        9,
        Bool(),
        AccessType.WO,
        resetValue = 0,
        doc = "Listen for receive acknowledge"
      )
    }

    val txData = new Area {
      val TXDATA = busif
        .newRegAt(0x00, doc = "Transmit data")
        .setName("txData")
      val value = TXDATA.fieldAt(
        0,
        Bits(8 bits),
        AccessType.WO,
        doc = "Transmit data value"
      )
      val valid = TXDATA.fieldAt(
        8,
        Bool(),
        AccessType.RW,
        resetValue = 1,
        doc = "Transmit data valid"
      )
      val enable = TXDATA.fieldAt(
        9,
        Bool(),
        AccessType.RW,
        resetValue = 0,
        doc = "Transmit data enable"
      )
      val repeat = TXDATA.fieldAt(
        10,
        Bool(),
        AccessType.WO,
        resetValue = 1,
        doc = "Transmit data repeat"
      )
      val disableOnDataConflict = TXDATA.fieldAt(
        11,
        Bool(),
        AccessType.WO,
        doc = "Disable on data conflict"
      )
      val forceDisable = False
    }

    val txAck = new Area {
      val TXACK = busif
        .newRegAt(0x04, doc = "Transmit acknowledge")
        .setName("txAck")
      val value = TXACK.fieldAt(
        0,
        Bool(),
        AccessType.WO,
        doc = "Transmit acknowledge value"
      )
      val valid = TXACK.fieldAt(
        8,
        Bool(),
        AccessType.RW,
        resetValue = 1,
        doc = "Transmit acknowledge valid"
      )
      val enable = TXACK.fieldAt(
        9,
        Bool(),
        AccessType.RW,
        resetValue = 0,
        doc = "Transmit acknowledge enable"
      )
      val repeat = TXACK.fieldAt(
        10,
        Bool(),
        AccessType.WO,
        resetValue = 1,
        doc = "Transmit acknowledge repeat"
      )
      val disableOnDataConflict = TXACK.fieldAt(
        11,
        Bool(),
        AccessType.WO,
        doc = "Disable on data conflict"
      )
      val forceAck = False
    }

    val masterLogic = genMaster generate new Area {
      val MSTAT = busif
        .newRegAt(0x40, doc = "Master status")
        .setName("masterStatus")
      val busy = MSTAT.fieldAt(0, Bool(), AccessType.RO, doc = "Is busy?")
      val start =
        MSTAT.fieldAt(
          4,
          Bool(),
          AccessType.W1S,
          resetValue = 0,
          doc = "Order a start (set on set)"
        )
      val stop =
        MSTAT.fieldAt(
          5,
          Bool(),
          AccessType.W1S,
          resetValue = 0,
          doc = "Order a stop (set on set)"
        )
      val drop =
        MSTAT.fieldAt(
          6,
          Bool(),
          AccessType.W1S,
          resetValue = 0,
          doc = "Order a drop (set on set)"
        )
      val recover =
        MSTAT.fieldAt(
          7,
          Bool(),
          AccessType.W1S,
          resetValue = 0,
          doc = "Order a recover (set on set)"
        )
      val startDropped = MSTAT.fieldAt(
        9,
        Bool(),
        AccessType.W1C,
        resetValue = 0,
        doc = "Timeout during start"
      )
      val stopDropped = MSTAT.fieldAt(
        10,
        Bool(),
        AccessType.W1C,
        resetValue = 0,
        doc = "Timeout during stop"
      )
      val recoverDropped = MSTAT.fieldAt(
        11,
        Bool(),
        AccessType.W1C,
        resetValue = 0,
        doc = "Timeout during recover"
      )

      val timer = new Area {
        val value = Reg(UInt(masterGenerics.timerWidth bits))

        val TLOW = busif.newRegAt(0x50, doc = "I2C low timing")
        val tLow =
          TLOW.fieldAt(0, value, AccessType.WO, doc = "# of cycles low")
        val THIGH = busif.newRegAt(0x54, doc = "I2C high timing")
        val tHigh =
          THIGH.fieldAt(0, value, AccessType.WO, doc = "# of cycles high")
        val TBUF = busif.newRegAt(0x58, doc = "I2C idle timing")
        val tBuf =
          TBUF.fieldAt(0, value, AccessType.WO, doc = "# of cycles idle")

        val done = value === 0

        value := value - (!done).asUInt
      }

      val txReady = Bool() // Say if the tx buffer is ready to continue

      val fsm = new StateMachine {
        val dropped = new Area {
          val start = startDropped
          val stop = stopDropped
          val recover = recoverDropped
          val trigger = False
        }
        always {
          when(
            drop || (!isActive(
              IDLE
            ) && (bus.cmd.kind === I2cSlaveCmdMode.DROP || i2cCtrl.io.timeout))
          ) {
            start := False
            stop := False
            drop := False
            recover := False
            dropped.start setWhen (start)
            dropped.stop setWhen (stop)
            dropped.trigger := True
            goto(TBUF)
          }
        }

        val inFrameLate = Reg(
          Bool()
        ) setWhen (!internals.sclRead) clearWhen (!internals.inFrame) // Allow to catch up a start sequance until SCL is low
        val outOfSync =
          !internals.inFrame && (!internals.sdaRead || !internals.sclRead)
        val IDLE: State = new State with EntryPoint {
          whenIsActive {
            when(internals.inFrame.fall(False)) {
              goto(TBUF)
            } elsewhen (start && !inFrameLate) {
              txData.valid := False
              goto(START1)
            } elsewhen (recover) {
              goto(LOW)
            }
          }
        }

        val START1: State = new State {
          whenIsActive {
            when(!outOfSync) {
              goto(START2)
            }
          }
        }

        val START2: State = new State {
          onEntry {
            timer.value := timer.tHigh
          }
          whenIsActive {
            i2cBuffer.sda.write := False
            when(timer.done || !internals.sclRead) {
              goto(START3)
            }
          }
        }

        val START3: State = new State {
          onEntry {
            timer.value := timer.tLow
          }
          whenIsActive {
            i2cBuffer.sda.write := False
            i2cBuffer.scl.write := False
            when(timer.done) {
              start := False
              goto(LOW)
            }
          }
        }

        val LOW: State = new State {
          onEntry {
            timer.value := timer.tLow
          }
          whenIsActive {
            when(timer.done) {
              when(stop && !inAckState || recover && internals.sdaRead) {
                i2cBuffer.scl.write := False
                txData.forceDisable := True
                goto(STOP1)
              } elsewhen (start && !inAckState) {
                i2cBuffer.scl.write := False
                txData.forceDisable := True
                goto(RESTART)
              } otherwise {
                when(internals.sclRead) {
                  goto(HIGH)
                }
              }
            } otherwise {
              i2cBuffer.scl.write := False
            }
          }
        }

        val HIGH: State = new State {
          onEntry {
            timer.value := timer.tHigh
          }
          whenIsActive {
            when(timer.done || !internals.sclRead) {
              goto(LOW)
            }
          }
        }

        val RESTART: State = new State {
          whenIsActive {
            when(!internals.sclRead) { // Check for slave clock stretching
              timer.value := timer.tHigh
            } elsewhen (timer.done) {
              goto(START1)
            }
          }
        }

        val STOP1: State = new State {
          onEntry {
            timer.value := timer.tHigh
          }
          whenIsActive {
            i2cBuffer.scl.write := False
            i2cBuffer.sda.write := False
            when(timer.done) {
              goto(STOP2)
            }
          }
        }

        val STOP2: State = new State {
          whenIsActive {
            i2cBuffer.sda.write := False
            when(!internals.sclRead) {
              timer.value := timer.tHigh
            } elsewhen (timer.done) {
              goto(STOP3)
            }
          }
        }

        val STOP3: State = new State {
          whenIsActive {
            when(internals.sdaRead) {
              stop := False
              recover := False
              goto(TBUF)
            }
          }
        }

        val TBUF: State = new State {
          onEntry {
            timer.value := timer.tBuf
          }
          whenIsActive {
            when(timer.done) {
              goto(IDLE)
            }
          }
        }

        busy := !this.isActive(IDLE) && !this.isActive(TBUF)
      }
    }

    val dataCounter = RegInit(U"000")
    val inAckState = RegInit(False)
    val wasntAck = RegInit(False)

    if (genMaster)
      masterLogic.txReady := inAckState ? txAck.valid | txData.valid

    when(!inAckState) {
      bus.rsp.valid := txData.valid && !(rxData.valid && rxData.listen) && bus.cmd.kind === I2cSlaveCmdMode.DRIVE
      bus.rsp.enable := txData.enable
      bus.rsp.data := txData.value(7 - dataCounter)

      when(txData.forceDisable) {
        bus.rsp.valid := True
        bus.rsp.enable := False
      }
    } otherwise {
      bus.rsp.valid := txAck.valid && !(rxAck.valid && rxAck.listen) && bus.cmd.kind === I2cSlaveCmdMode.DRIVE
      bus.rsp.enable := txAck.enable
      bus.rsp.data := txAck.value
      when(txAck.forceAck) {
        bus.rsp.valid := True
        bus.rsp.enable := True
        bus.rsp.data := False
      }
    }

    val isMasterMode =
      if (masterLogic != null) masterLogic.busy else False

    when(wasntAck && !isMasterMode) {
      bus.rsp.valid := bus.cmd.kind === I2cSlaveCmdMode.DRIVE
      bus.rsp.enable := False
    }

    switch(bus.cmd.kind) {
      is(I2cSlaveCmdMode.START) {
        frameReset := True
      }
      is(I2cSlaveCmdMode.RESTART) {
        frameReset := True
      }
      is(I2cSlaveCmdMode.STOP) {
        frameReset := True
      }
      is(I2cSlaveCmdMode.DROP) {
        frameReset := True
      }
      is(I2cSlaveCmdMode.READ) {
        when(!inAckState) {
          rxData.valueReg(7 - dataCounter) := bus.cmd.data
          dataCounter := dataCounter + 1

          when(bus.rsp.data =/= bus.cmd.data) {
            txData.enable clearWhen (txData.disableOnDataConflict)
            txAck.enable clearWhen (txAck.disableOnDataConflict)
          }
          when(dataCounter === 7) {
            rxData.valid setWhen (rxData.listen)
            rxData.event := True
            inAckState := True
            when(txData.valid && !txData.repeat) {
              txData.valid := False
            }
          }
        } otherwise {
          rxAck.valid setWhen (rxAck.listen)
          rxAck.valueReg := bus.cmd.data
          inAckState := False
          wasntAck := bus.cmd.data

          when(txAck.valid && !txAck.repeat) {
            txAck.valid := False
          }
        }
      }
    }

    when(frameReset) {
      inAckState := False
      dataCounter := 0
      wasntAck := False
    }

    when(
      bus.cmd.kind === I2cSlaveCmdMode.STOP || bus.cmd.kind === I2cSlaveCmdMode.DROP
    ) {
      txData.valid := True
      txData.enable := False
      txData.repeat := True
      txData.forceDisable := False
      txData.disableOnDataConflict := False

      txAck.valid := True
      txAck.enable := False
      txAck.repeat := True
      txAck.disableOnDataConflict := False

      rxData.listen := False
      rxAck.listen := False
    }

    val SCD = busif
      .newRegAt(0x28, doc = "Sampling clock")
      .setName("samplingClockDivider")
    val samplingClockDivider = SCD.fieldAt(
      0,
      config.samplingClockDivider,
      AccessType.WO,
      resetValue = 0,
      doc = "Sampling clock divider"
    )
    config.samplingClockDivider := samplingClockDivider

    val TIMEOUT = busif.newRegAt(0x2c, doc = "Timeout")
    val timeout = TIMEOUT.fieldAt(
      0,
      config.timeout,
      AccessType.WO,
      resetValue = 0,
      doc = "Timeout"
    )
    config.timeout := timeout

    val TSUDATA = busif
      .newRegAt(0x30, doc = "TSU Data")
      .setName("tsuData")
    val tsuData = TSUDATA.fieldAt(
      0,
      config.tsuData,
      AccessType.WO,
      resetValue = 0
    )
    config.tsuData := tsuData

    val timeoutClear = RegNext(False)
    config.timeoutClear := timeoutClear

    when(TIMEOUT.hitDoWrite) {
      timeoutClear := True
    }
    if (genMaster)
      config.timeoutClear setWhen (!internals.inFrame && !masterLogic.busy)

    val slaveStatus = new Area {
      val SSTAT = busif
        .newRegAt(0x44, doc = "Slave status")
        .setName("slaveStatus")
      val inFrame = SSTAT.fieldAt(0, Bool(), AccessType.RO, doc = "In Frame")
      inFrame := internals.inFrame
      val sdaRead = SSTAT.fieldAt(1, Bool(), AccessType.RO, doc = "SDA read")
      sdaRead := internals.sdaRead
      val sclRead = SSTAT.fieldAt(2, Bool(), AccessType.RO, doc = "SCL read")
      sclRead := internals.sclRead
    }

    if (genMaster) masterLogic.fsm.build()
    val slaveOverride = new Area {
      val SOVERRIDE = busif
        .newRegAt(0x48, doc = "Slave override")
        .setName("slaveOverride")
      val sda = SOVERRIDE.fieldAt(
        1,
        Bool(),
        AccessType.RW,
        resetValue = 1,
        doc = "Force the SDA pin low when cleared"
      )
      val scl = SOVERRIDE.fieldAt(
        2,
        Bool(),
        AccessType.RW,
        resetValue = 1,
        doc = "Force the SCL pin low when cleared"
      )
      i2cBuffer.sda.write clearWhen (!sda)
      i2cBuffer.scl.write clearWhen (!scl)
    }
  }

  io.i2c.scl.write := RegNext(bridge.i2cBuffer.scl.write) init (True)
  io.i2c.sda.write := RegNext(bridge.i2cBuffer.sda.write) init (True)
  bridge.i2cBuffer.scl.read := io.i2c.scl.read
  bridge.i2cBuffer.sda.read := io.i2c.sda.read

  def svd(name: String, baseAddress: BigInt) = {
    SvdPeripheral(
      busif,
      name,
      baseAddress,
      description = "I2C controller"
    )
  }
}
