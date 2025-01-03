package slabware

import spinal.core._
import spinal.lib._
import spinal.lib.fsm._
import spinal.lib.com.usb._
import spinal.lib.com.usb.ohci.{UsbPid}
import spinal.lib.com.usb.udc.{
  UsbDeviceCtrlParameter,
  UsbDeviceCtrl => SpinalDeviceCtrl
}
import spinal.lib.bus.regif.{AccessType, SymbolName}

class UsbDeviceCtrl[B <: BusDefinition.Bus](
    busDef: BusDefinition[B],
    p: UsbDeviceCtrlParameter
) extends Component {
  import SpinalDeviceCtrl._

  val AddressWidth = 16
  val DataWidth = 32

  val io = new Bundle {
    val bus = slave(busDef.createBus(AddressWidth, DataWidth))
    val phy = master(PhyIo())
    val interrupt = out Bool ()
  }

  val busIf =
    busDef.createBusInterface(io.bus, (0, BigInt(2).pow(AddressWidth)))

  val memory = new Area {
    val ram = Mem(Bits(32 bits), 1 << p.addressWidth - 2)

    val readPort = ram.readSyncPort
    val writePort = ram.writePortWithMask(4)

    val internal = new Area {
      val writeCmd = cloneOf(writePort)
      val readCmd = cloneOf(readPort.cmd)
      val readRsp = readCmd.stage().translateWith(readPort.rsp)

      writeCmd.mask.setWidth(4)

      def doRead(byteAddress: UInt): Unit = {
        readCmd.valid := True
        readCmd.payload := byteAddress >> 2
      }

      def doWrite(byteAddress: UInt, data: Bits, enable: Bool): Unit = {
        writeCmd.valid := enable
        writeCmd.address := byteAddress >> 2
        writeCmd.data.subdivideIn(8 bits).foreach(_ := data)
        writeCmd.mask := UIntToOh(byteAddress(1 downto 0), 4)
      }

      def doWrite(
          byteAddress: UInt,
          data: Bits,
          mask: Bits,
          enable: Bool
      ): Unit = {
        writeCmd.valid := enable
        writeCmd.address := byteAddress >> 2
        writeCmd.data := data
        writeCmd.mask := mask
      }

      readCmd.valid := False
      readCmd.payload.assignDontCare()
      writeCmd.valid := False
      writeCmd.payload.assignDontCare()
    }

    val external = new Area {
      val halt = False
      val writeCmd = Stream(writePort.payloadType)
      val readCmd = Stream(readPort.cmd.payloadType)
      val readRsp = readCmd.toFlowFire.stage.translateWith(readPort.rsp)

      val writeCmdHalted = writeCmd.haltWhen(halt)

      writeCmd.mask.setWidth(4)
    }

    readPort.cmd.valid := internal.readCmd.valid || external.readCmd.valid
    readPort.cmd.payload := internal.readCmd.valid ? internal.readCmd.payload | external.readCmd.payload
    external.readCmd.ready := !internal.readCmd.valid

    writePort.valid := internal.writeCmd.valid || external.writeCmdHalted.valid
    writePort.payload := internal.writeCmd.valid ? internal.writeCmd.payload | external.writeCmdHalted.payload
    external.writeCmdHalted.ready := !internal.writeCmd.valid
  }

  val memoryMapping = new Area {
    val mappingSize = 0x8000
    val ramIf = busIf.newRAMAt(0, mappingSize, doc = null)
    val readBuffer = memory.external.readRsp.toReg
    ramIf.bus.rdat := readBuffer
    val readState = RegInit(U"00")
    val writeState = RegInit(U"0")
    memory.external.readCmd.valid := False
    memory.external.readCmd.payload := (busIf.readAddress() >> 2).resized
    memory.external.writeCmd.valid := False
    memory.external.writeCmd.address := (busIf.writeAddress() >> 2).resized
    memory.external.writeCmd.mask := 0xf
    memory.external.writeCmd.data := busIf.writeData
    when(busIf.readAddress() < mappingSize) {
      when(busIf.askRead) {
        switch(readState) {
          is(0) {
            busIf.readHalt()
            memory.external.readCmd.valid := True
            when(memory.external.readCmd.ready) {
              readState := 1
            }
          }
          is(1) {
            busIf.readHalt()
            when(memory.external.readRsp.fire) {
              readState := 2
            }
          }
        }
      }
      when(busIf.doRead) {
        readState := 0
      }
    }
    when(busIf.writeAddress() < mappingSize) {
      when(busIf.askWrite) {
        switch(writeState) {
          is(0) {
            busIf.writeHalt()
            memory.external.writeCmd.valid := True
            when(memory.external.writeCmd.ready) {
              writeState := 1
            }
          }
        }
      }
      when(busIf.doWrite) {
        writeState := 0
      }
    }
  }

  val mapping = new Area {
    import SpinalDeviceCtrl.Regs

    val FRAME = new Area {
      val frameReg =
        busIf.newRegAt(Regs.FRAME, doc = "USB frame id").setName("Frame")
      val usbFrameId = frameReg.field(
        UInt(11 bits),
        AccessType.RO,
        doc = "Current USB frame ID"
      )
    }

    val ADDRESS = new Area {
      val addressReg =
        busIf.newRegAt(Regs.ADDRESS, doc = "USB address").setName("Address")
      val value =
        addressReg.field(
          Bits(7 bits),
          AccessType.RW,
          resetValue = 0,
          doc =
            "The device will only listen at tokens with the specified address. This field is automatically cleared on usb reset events"
        )
      val enable = addressReg.field(
        Bool(),
        AccessType.RW,
        resetValue = 0,
        doc = "Enable the USB address filtering if set"
      )
      val trigger = addressReg.field(
        Bool(),
        AccessType.RW,
        resetValue = 0,
        doc =
          "Set the enable (see above) on the next EP0 IN token completion Cleared by the hardware after any EP0 completion"
      )
    }

    val INTERRUPT = new Area {
      val interruptReg =
        busIf
          .newRegAt(Regs.INTERRUPT, doc = "Interrupt status")
          .setName("Interrupt")
      val endpoints = interruptReg.field(
        Bits(p.epCount bits),
        AccessType.W1C,
        resetValue = 0,
        doc = "Raised when an endpoint generates an interrupt"
      )
      val reset = interruptReg.field(
        Bool(),
        AccessType.W1C,
        resetValue = 0,
        doc = "Raised when a USB reset occurs"
      )
      val ep0Setup = interruptReg.field(
        Bool(),
        AccessType.W1C,
        resetValue = 0,
        doc = "Raised when endpoint 0 receives a setup transaction"
      )
      val suspend = interruptReg.field(
        Bool(),
        AccessType.W1C,
        resetValue = 0,
        doc = "Raised when a USB suspend occurs"
      )
      val resume = interruptReg.field(
        Bool(),
        AccessType.W1C,
        resetValue = 0,
        doc = "Raised when a USB resume occurs"
      )
      val connect = interruptReg.field(
        Bool(),
        AccessType.W1C,
        resetValue = 0,
        doc = "Raised when a USB connect occurs (bus power)"
      )
      val disconnect = interruptReg.field(
        Bool(),
        AccessType.W1C,
        resetValue = 0,
        doc = "Raised when a USB disconnect occurs (no bus power)"
      )
    }

    val HALT = new Area {
      val haltReg =
        busIf.newRegAt(Regs.HALT, doc = "Halt endpoint").setName("Halt")
      val endpointId = haltReg.field(
        UInt(log2Up(p.epCount) bits),
        AccessType.RW,
        resetValue = 0,
        doc = "The endpoint you want to put in sleep"
      )
      val enable = haltReg.field(
        Bool(),
        AccessType.RW,
        resetValue = 0,
        doc = "Halt is active when set, endpoint is unhalted when cleared."
      )
      val effective = haltReg.field(
        Bool(),
        AccessType.RO,
        doc =
          "After setting the enable, wait for this bit to be set by the hardware to ensure atomicity"
      )
    }

    val CONFIG = new Area {
      val configReg =
        busIf.newRegAt(Regs.CONFIG, doc = "Configuration").setName("Config")
      val pullUpEnable = configReg.field(
        Bool(),
        AccessType.RW,
        resetValue = 0,
        doc = "Enable USB device pullup on dp pin"
      )
      val interruptEnable = configReg.field(
        Bool(),
        AccessType.RW,
        resetValue = 0,
        doc = "Enable interrupts"
      )
    }

    val INFO = new Area {
      val infoReg = busIf.newRegAt(Regs.ADDRESS_WIDTH, doc = "Info")
      val ramSize = infoReg.field(
        UInt(4 bits),
        AccessType.RO,
        doc = "Internal ram address width (bits)"
      )
      ramSize := U(p.addressWidth, 4 bits)

      val powerDetected = infoReg.field(
        Bool(),
        AccessType.RO,
        doc = "USB bus power detected"
      )
      powerDetected := io.phy.power
    }
  }

  io.phy.resumeIt := False
  io.phy.tx.stream.valid := False
  io.phy.tx.stream.payload.assignDontCare()

  val done = new Area {
    val pendings = Reg(Bits(p.epCount bits))
    //    val head = Reg(UInt(p.addressWidth bits))
  }

  val regs = new Area {
    val frame = Reg(UInt(11 bits))
    mapping.FRAME.usbFrameId := frame

    val address = new Area {
      val value = mapping.ADDRESS.value
      val enable = mapping.ADDRESS.enable
      val trigger = mapping.ADDRESS.trigger
    }
    val interrupts = new Area {
      val endpoints = mapping.INTERRUPT.endpoints
      val reset = mapping.INTERRUPT.reset
      val ep0Setup = mapping.INTERRUPT.ep0Setup
      val suspend = mapping.INTERRUPT.suspend
      suspend.setWhen(io.phy.suspend.rise(False))
      val resume = mapping.INTERRUPT.resume
      resume.setWhen(io.phy.resume.valid)
      val connect = mapping.INTERRUPT.connect
      connect.setWhen(io.phy.power.rise(False))
      val disconnect = mapping.INTERRUPT.disconnect
      disconnect.setWhen(io.phy.power.fall(False))

      import mapping.CONFIG.{interruptEnable => enable}
      val pending =
        (endpoints.orR || reset || suspend || resume || connect || disconnect || ep0Setup) && enable
    }
    val halt = new Area {
      val id = mapping.HALT.endpointId
      val enable = mapping.HALT.enable

      val effective = RegInit(False)
      mapping.HALT.effective := effective
      val hit = Bool()
    }
    import mapping.CONFIG.{pullUpEnable => pullup}
    io.phy.pullup := pullup
  }

  val rxTimer = new Area {
    val counter = Reg(UInt(5 bits))
    val clear = False
    when(io.phy.tick) {
      counter := counter + 1
    }
    when(clear) {
      counter := 0
    }
    clear setWhen (io.phy.rx.active)

    def cycles(c: Int): Bool = counter === (c - 1)

    val timeout = cycles(24)
    val turnover = cycles(2)
  }

  val token = new UsbTokenRxFsm(
    rx = io.phy.rx.flow,
    rxActive = io.phy.rx.active,
    rxStuffing = io.phy.rx.stuffingError,
    timeoutClear = rxTimer.clear,
    timeoutEvent = rxTimer.timeout
  ) {
    val isSetup = pid === UsbPid.SETUP
    val isIn = pid === UsbPid.IN
  }
  regs.halt.hit := regs.halt.enable && regs.halt.id === token.endpoint

  val dataRx = new UsbDataRxFsm(
    rx = io.phy.rx.flow,
    rxActive = io.phy.rx.active,
    rxStuffing = io.phy.rx.stuffingError,
    timeoutClear = rxTimer.clear,
    timeoutEvent = rxTimer.timeout
  )

  val dataTx = new UsbDataTxFsm(tx = io.phy.tx.stream, eop = io.phy.tx.eop) {

    val startNull = False
    val input = Stream(Fragment(Bits(8 bits))) // Allow sparse transactions
    input.valid := False
    input.payload.assignDontCare()

    data.valid.removeAssignments()
    data.payload.removeAssignments()
    data << input.halfPipe().stage()
    when(data.valid && isStopped || startNull) {
      startFsm()
    }
  }

  val descAlign = 4
  val ep = new Area {
    def addressByte = addressWord << 2
    def addressWord = token.endpoint.resize(p.addressWidth - 2)

    val word = Reg(Bits(32 bits))

    val head = word(4, p.addressWidth - descAlign bits).asUInt
    val enable = word(0)
    val stall = word(Status.STALL)
    val nack = word(2)
    val dataPhase = word(3)
    val isochronous = word(16)
    val maxPacketSize = word(22, 10 bits).asUInt
    val headByte = head << descAlign
  }

  val desc = new Area {
    def addressByte = ep.headByte
    def addressWord = ep.head << (descAlign - 2)

    val words = Vec(Reg(Bits(32 bits)), 3)

    val offset = words(0)(0, p.lengthWidth bits)
    val code = words(0)(16, 4 bits)

    val next = words(1)(4, p.addressWidth - descAlign bits).asUInt
    val length = words(1)(16, p.lengthWidth bits)

    val direction = words(2)(16)
    val interrupt = words(2)(17)
    val completionOnFull = words(2)(18)
    val data1OnCompletion = words(2)(19)
    val frame = words(2)(0, 12 bits)

    val offsetIncrement = False
    when(offsetIncrement) {
      offset := B(U(offset) + 1)
    }

    assert(descAlign == 4)
    val currentByte = ((ep.head @@ U"1100") + U(offset)).resize(p.addressWidth)
    val full = offset === length
    val dataPhaseMatch =
      ep.dataPhase ? (dataRx.pid === UsbPid.DATA1) | (dataRx.pid === UsbPid.DATA0)
  }

  val byteCounter = new Area {
    val value = Reg(UInt(10 bits))
    val clear, increment = False
    val full = value === ep.maxPacketSize

    when(increment) { value := value + 1 }
    when(clear) { value := 0 }
  }
  val transferFull = desc.full || byteCounter.full

  val active = new StateMachineSlave {
    val IDLE = new State
    val TOKEN = new StateFsm(token)
    val ADDRESS_HIT = new State
    val EP_READ, EP_ANALYSE = new State
    val DESC_READ_0, DESC_READ_1, DESC_READ_2 = new State
    val DESC_ANALYSE = new State
    val DATA_RX, DATA_RX_ANALYSE = new State
    val HANDSHAKE_TX_0, HANDSHAKE_TX_1 = new State
    val DATA_TX_0, DATA_TX_1 = new State
    val HANDSHAKE_RX_0, HANDSHAKE_RX_1 = new State
    val UPDATE_SETUP, UPDATE_DESC, UPDATE_EP = new State

    setEntry(IDLE)

    val handshakePid = Reg(Bits(4 bits))
    val completion = Reg(Bool())
    val noUpdate = Reg(Bool())

    regs.halt.effective setWhen (isStopped || isActive(IDLE) || isActive(
      TOKEN
    ) || !regs.halt.hit)

    IDLE whenIsActive {
      completion := False
      noUpdate := False

      when(io.phy.rx.active) {
        goto(TOKEN)
      }
    }

    TOKEN whenCompleted {
      goto(ADDRESS_HIT)
    }

    ADDRESS_HIT whenIsActive {
      when(!token.ok) {
        goto(IDLE)
      } otherwise {
        switch(token.pid) {
          is(UsbPid.SOF) {
            regs.frame := U(token.data)
            goto(IDLE)
          }
          is(UsbPid.SETUP, UsbPid.OUT, UsbPid.IN) {
            when(
              token.address === (regs.address.enable ? regs.address.value otherwise 0)
            ) {
              memory.internal.doRead(ep.addressByte)
              when(token.pid === UsbPid.SETUP || token.pid === UsbPid.OUT) {
                dataRx.startFsm()
              }
              goto(EP_READ)
            } otherwise {
              goto(IDLE)
            }
          }
          default {
            goto(IDLE)
          }
        }
      }
    }

    EP_READ whenIsActive {
      ep.word := memory.internal.readRsp.payload
      goto(EP_ANALYSE)
    }

    EP_ANALYSE whenIsActive {
      memory.internal.doRead(desc.addressByte)
      when(!ep.enable) {
        goto(IDLE)
      } elsewhen (token.isSetup) {
        when(token.endpoint =/= 0) {
          goto(IDLE)
        } otherwise {
          ep.word(
            15 downto 4
          ) := 0 // Ensure no offset is applyed from memory address 0x40-0x47
          desc.offset := 0x40 - 12
          desc.length := 8
          desc.direction := False
          ep.dataPhase := False
          goto(DESC_ANALYSE)
        }
      } elsewhen (ep.head === 0 || ep.stall || regs.halt.hit) {
        handshakePid := ((ep.stall && !regs.halt.hit) ? B(UsbPid.STALL) | B(
          UsbPid.NAK
        )).resized
        switch(token.pid) {
          is(UsbPid.OUT) {
            noUpdate := True
            goto(DATA_RX)
          }
          is(UsbPid.IN) {
            noUpdate := True
            when(ep.isochronous) {
              goto(IDLE)
            } otherwise {
              goto(HANDSHAKE_TX_0)
            }
          }
          default {
            goto(IDLE)
          }
        }
      } otherwise {
        goto(DESC_READ_0)
      }
    }

    DESC_READ_0 whenIsActive {
      desc.words(0) := memory.internal.readRsp.payload
      memory.internal.doRead(desc.addressByte | 4)
      goto(DESC_READ_1)
    }
    DESC_READ_1 whenIsActive {
      desc.words(1) := memory.internal.readRsp.payload
      memory.internal.doRead(desc.addressByte | 8)
      goto(DESC_READ_2)
    }

    DESC_READ_2 whenIsActive {
      desc.words(2) := memory.internal.readRsp.payload
      goto(DESC_ANALYSE)
    }

    DESC_ANALYSE whenIsActive {
      byteCounter.clear := True
      switch(token.pid) {
        is(UsbPid.SETUP) {
          goto(DATA_RX)
        }
        is(UsbPid.OUT) {
          when(desc.direction) {
            goto(IDLE)
          } otherwise {
            goto(DATA_RX)
          }
        }
        is(UsbPid.IN) {
          when(!desc.direction) {
            goto(IDLE)
          } otherwise {
            when(desc.full) {
              dataTx.startNull := True
            }
            goto(DATA_TX_0)
          }
        }
        default {
          goto(IDLE)
        }
      }
    }

    dataTx.pid := ep.dataPhase ## B"011"
    val byteSel = Reg(UInt(2 bits))
    DATA_TX_0 whenIsActive {
      byteSel := U(desc.offset).resized
      when(!transferFull) {
        when(dataTx.input.ready) {
          memory.internal.doRead(desc.currentByte)
          byteCounter.increment := True
          desc.offsetIncrement := True
          goto(DATA_TX_1)
        }
      } otherwise {
        when(io.phy.tx.eop) {
          when(ep.isochronous) {
            goto(UPDATE_SETUP)
          } otherwise {
            goto(HANDSHAKE_RX_0)
          }
        }
      }
    }

    DATA_TX_1 whenIsActive {
      dataTx.input.valid := True
      dataTx.input.last := transferFull
      dataTx.input.fragment := memory.internal.readRsp.payload
        .subdivideIn(8 bits)(byteSel)
      goto(DATA_TX_0)
    }

    HANDSHAKE_RX_0 onEntry {
      rxTimer.clear := True
    }
    HANDSHAKE_RX_0 whenIsActive {
      when(io.phy.rx.flow.valid) {
        when(
          io.phy.rx.flow.payload(3 downto 0) =/= ~io.phy.rx.flow
            .payload(7 downto 4)
        ) {
          goto(IDLE)
        } elsewhen (io.phy.rx.flow.payload(3 downto 0) =/= UsbPid.ACK) {
          goto(IDLE)
        } otherwise {
          goto(HANDSHAKE_RX_1)
        }
      }
      when(rxTimer.timeout || io.phy.rx.active.fall()) {
        goto(IDLE)
      }
    }

    HANDSHAKE_RX_1 whenIsActive {
      when(!io.phy.rx.active) {
        goto(UPDATE_SETUP)
      }
      when(io.phy.rx.flow.valid) {
        goto(IDLE)
      }
    }

    val dataRxOverrun = Reg(Bool())
    DATA_RX onEntry {
      dataRxOverrun := False
    }
    DATA_RX whenIsActive {
      when(dataRx.data.valid) {
        memory.internal.doWrite(
          desc.currentByte,
          dataRx.data.payload,
          !transferFull && !noUpdate
        )
        when(transferFull && !noUpdate) {
          dataRxOverrun := True
        } otherwise {
          byteCounter.increment := True
          desc.offsetIncrement := True
        }
      }
      when(dataRx.wantExit) {
        goto(DATA_RX_ANALYSE)
      }
    }

    DATA_RX_ANALYSE whenIsActive {
      when(dataRx.hasError || dataRxOverrun) { // TODO Maybe dataRxOverrun should ACK ?
        goto(IDLE)
      } otherwise {
        when(!noUpdate) {
          handshakePid := UsbPid.ACK
        }
        when(dataRx.pid(2 downto 0) =/= B"011") {
          goto(IDLE)
        } otherwise {
          when(!ep.stall && dataRx.pid.msb =/= ep.dataPhase) {
            noUpdate := True
            handshakePid := UsbPid.ACK // Ensure that a wrong data phase is ACKED (even in the case there is no descriptor)
          }
          when(ep.isochronous) {
            goto(UPDATE_SETUP)
          } otherwise {
            goto(HANDSHAKE_TX_0)
          }
        }
      }
    }

    HANDSHAKE_TX_0 whenIsActive {
      when(rxTimer.turnover) {
        goto(HANDSHAKE_TX_1)
      }
    }
    HANDSHAKE_TX_1 whenIsActive {
      io.phy.tx.stream.valid := True
      io.phy.tx.stream.last := True
      io.phy.tx.stream.fragment := UsbPid.token(handshakePid)
      when(io.phy.tx.stream.ready) {
        goto(UPDATE_SETUP)
      }
    }

    UPDATE_SETUP whenIsActive {
      memory.external.halt := True
      memory.internal.doRead(
        desc.addressByte | 4
      ) // Fetch the next descriptor in a atomic manner to ease the software tail insertion

      when(!token.isSetup) {
        completion setWhen (!byteCounter.full || desc.completionOnFull && desc.full)
      }

      when(noUpdate) {
        goto(IDLE)
      } otherwise {
        goto(UPDATE_DESC)
      }
    }

    UPDATE_DESC whenIsActive {
      memory.external.halt := True
      desc.words(1) := memory.internal.readRsp.payload

      memory.internal.writeCmd.valid := !token.isSetup
      memory.internal.writeCmd.address := desc.addressWord
      memory.internal.writeCmd.mask := 0xf
      memory.internal.writeCmd.data := 0
      memory.internal.writeCmd.data(0, p.lengthWidth bits) := desc.offset
      memory.internal.writeCmd.data(16, 4 bits) := (completion ? B(0) | B(
        15
      )) // TODO if more error condition, update condition of completion when(!desc.full){

      goto(UPDATE_EP)
    }

    UPDATE_EP whenIsActive {
      memory.external.halt := True
      memory.internal.writeCmd.valid := True
      memory.internal.writeCmd.address := ep.addressWord
      memory.internal.writeCmd.mask := 0x3
      memory.internal.writeCmd
        .data(0, 4 bits) := B(!ep.dataPhase, ep.nack, ep.stall, ep.enable)
      memory.internal.writeCmd.data(4, 12 bits) := B(
        completion ? desc.next | ep.head
      ).resized
      when(token.isSetup) {
        memory.internal.writeCmd.data(Status.STALL) := False
        memory.internal.writeCmd.data(4, 12 bits) := 0
        regs.interrupts.ep0Setup := True
      }

      when(completion) {
        when(desc.data1OnCompletion) {
          memory.internal.writeCmd.data(3) := True
        }
        when(desc.interrupt) {
          regs.interrupts.endpoints(token.endpoint.resized) := True
        }
        when(token.endpoint === 0) {
          when(regs.address.trigger && token.isIn) {
            regs.address.enable := True
          }
          regs.address.trigger := False
        }
        when(!desc.full) { // When a descriptor is completed but not full, unlink the linked list for the software to fix things
          memory.internal.writeCmd.data(4, 12 bits) := 0
        }
      }
      goto(IDLE)
    }
  }

  val main = new StateMachine {
    val ATTACHED, POWERED, ACTIVE_INIT, ACTIVE = new State
    setEntry(ATTACHED)

    ATTACHED whenIsActive {
      when(io.phy.power) {
        goto(POWERED)
      }
    }
    POWERED whenIsActive {
      when(io.phy.reset) {
        goto(ACTIVE_INIT)
      }
    }

    ACTIVE_INIT onEntry {
      regs.interrupts.reset := True
    }
    ACTIVE_INIT whenIsActive {
      regs.address.enable := False
      when(!io.phy.reset) {
        goto(ACTIVE)
      }
    }

    ACTIVE onEntry {
      active.startFsm()
    }
    ACTIVE whenIsActive {
      when(io.phy.reset) {
        goto(ACTIVE_INIT)
      }
    }
  }

  io.interrupt := RegNext(regs.interrupts.pending) init (False)

  def svd(name: String, baseAddress: BigInt) = {
    SvdPeripheral(
      busIf,
      name,
      baseAddress,
      description = "USB device controller"
    )
  }
}
