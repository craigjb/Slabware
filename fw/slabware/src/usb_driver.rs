use core::future::poll_fn;
use core::sync::atomic::{compiler_fence, Ordering};
use core::task::Poll;
use core::{marker::PhantomData, slice};

use bitfield::bitfield;
use embassy_sync::waitqueue::AtomicWaker;
use embassy_usb_driver::{
    self as driver, Direction, EndpointAddress, EndpointAllocError, EndpointError, EndpointInfo,
    EndpointType, Event,
};
use slab_pac::UsbCtrl;
use vcell::VolatileCell;

use crate::custom_int::{disable_custom_interrupt, enable_custom_interrupt};

pub const USB_INTERRUPT_CODE: usize = 22;

const ENDPOINT_COUNT: usize = 16;
const SETUP_DATA_OFFSET: usize = 0x40;
const ENDPOINT_RAM_OFFSET: usize = 0x48;
const ENDPOINT_DESC_WORDS: usize = 3;

static BUS_WAKER: AtomicWaker = AtomicWaker::new();
static ENDPOINT_WAKERS: [AtomicWaker; ENDPOINT_COUNT] =
    [const { AtomicWaker::new() }; ENDPOINT_COUNT];

bitfield! {
    struct EndpointRegister(u32);
    enable, set_enable: 0;
    stall, set_stall: 1;
    nack, set_nack: 2;
    data_phase, set_data_phase: 3;
    u16, head, set_head: 15, 4;
    isochronous, set_isochronous: 16;
    u16, max_packet_size, set_max_packet_size: 31, 22;
}

bitfield! {
    struct EndpointDescriptorW0(u32);
    u16, offset, _ : 15, 0;
    u8, code, set_code: 19, 16;
}

bitfield! {
struct EndpointDescriptorW1(u32);
    u16, next, set_next: 15, 4;
    u16, length, set_length: 31, 16;
}

bitfield! {
struct EndpointDescriptorW2(u32);
    direction, set_direction: 16;
    interrupt, set_interrupt: 17;
    completion_on_full, set_completion_on_full: 18;
    data1_on_completion, set_data1_on_completion: 19;
}

pub struct Driver<'a> {
    _phantom: PhantomData<&'a ()>,
    regs: UsbCtrl,
    next_endpoint: usize,
    num_ram_words: usize,
    next_ram_word: usize,
    endpoint_regs: &'a mut [VolatileCell<u32>],
}

impl<'a> Driver<'a> {
    pub fn new(usb_ctrl: UsbCtrl) -> Self {
        let num_ram_words = (1 << (usb_ctrl.info_reg().read().ram_size().bits() as usize)) >> 2;

        // Reset address and halt
        usb_ctrl.address().write(|w| unsafe { w.bits(0) });
        usb_ctrl.halt().write(|w| unsafe { w.bits(0) });

        // Disable pull-up and clear any interrupts
        usb_ctrl.config().write(|w| {
            w.pull_up_enable()
                .clear_bit()
                .interrupt_enable()
                .clear_bit()
        });
        usb_ctrl
            .interrupt()
            .write(|w| unsafe { w.bits(0xFFFFFFFF) });
        usb_ctrl.interrupt_enable().write(|w| unsafe { w.bits(0) });

        // Zero out the RAM
        unsafe {
            let ram_ptr = UsbCtrl::ptr() as *mut u32;
            for i in 0..num_ram_words {
                ram_ptr.add(i).write_volatile(0);
            }
        }

        // Endpoint regs are 0x0000 - 0x003F
        let endpoint_regs: &mut [VolatileCell<u32>] = unsafe {
            slice::from_raw_parts_mut(
                UsbCtrl::ptr().cast::<VolatileCell<u32>>() as *mut _,
                ENDPOINT_COUNT,
            )
        };

        // Zero out endpoint regs
        for reg in endpoint_regs.iter() {
            reg.set(0)
        }

        let next_ram_word = ENDPOINT_RAM_OFFSET >> 2;
        Self {
            _phantom: PhantomData,
            regs: usb_ctrl,
            next_endpoint: 1,
            num_ram_words,
            next_ram_word,
            endpoint_regs,
        }
    }

    fn alloc_endpoint<D: Dir>(
        &mut self,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval_ms: u8,
    ) -> Result<Endpoint<'a, D>, EndpointAllocError> {
        defmt::debug!(
            "[USB] Allocating type={:?} mps={:?} interval_ms={}, dir={:?}",
            ep_type,
            max_packet_size,
            interval_ms,
            D::dir()
        );

        if self.next_endpoint >= ENDPOINT_COUNT {
            defmt::error!("[USB] Allocated too many endpoints");
            return Err(EndpointAllocError);
        }

        let endpoint_index = self.next_endpoint;
        let descriptor_head = (self.next_ram_word + 3) >> 2;
        let aligned_word_addr = descriptor_head << 2;
        let packet_words = ((max_packet_size + 3) >> 2) as usize;
        let total_words = ENDPOINT_DESC_WORDS + packet_words;

        if aligned_word_addr + total_words >= self.num_ram_words {
            defmt::error!(
                "[USB] Endpoint RAM full (next_word={:?})",
                self.next_ram_word
            );
            return Err(EndpointAllocError);
        }

        let mut endpoint_reg = EndpointRegister(0);
        endpoint_reg.set_isochronous(ep_type == EndpointType::Isochronous);
        endpoint_reg.set_max_packet_size(max_packet_size);
        self.endpoint_regs[endpoint_index].set(endpoint_reg.0);

        let endpoint_info = EndpointInfo {
            addr: EndpointAddress::from_parts(endpoint_index, D::dir()),
            ep_type,
            max_packet_size,
            interval_ms,
        };

        self.next_endpoint += 1;
        self.next_ram_word = aligned_word_addr + total_words;

        defmt::trace!(
            "Allocated endpoint ep={} dir={:?} desc_word_addr={} data_word_addr={}",
            endpoint_index,
            D::dir(),
            aligned_word_addr,
            aligned_word_addr + 3
        );

        Ok(Endpoint {
            _phantom: PhantomData,
            info: endpoint_info,
            buffer: EndpointBuffer::new(aligned_word_addr, packet_words),
        })
    }

    fn alloc_control_pipe(
        &mut self,
        max_packet_size: u16,
    ) -> Result<ControlPipe, EndpointAllocError> {
        defmt::debug!("[USB] Control max packet size={:?}", max_packet_size,);

        let packet_words = ((max_packet_size + 3) >> 2) as usize;
        let total_words = ENDPOINT_DESC_WORDS + packet_words;
        let out_descriptor_head = (self.next_ram_word + 3) >> 2;
        let out_aligned_word_addr = out_descriptor_head << 2;

        let next_ram_word = out_aligned_word_addr + total_words;
        let in_descriptor_head = (next_ram_word + 3) >> 2;
        let in_aligned_word_addr = in_descriptor_head << 2;

        if in_aligned_word_addr + total_words >= self.num_ram_words {
            defmt::error!(
                "[USB] Endpoint RAM full (next_word={:?})",
                self.next_ram_word
            );
            return Err(EndpointAllocError);
        }

        let mut ep_reg = EndpointRegister(0);
        ep_reg.set_enable(true);
        ep_reg.set_stall(false);
        ep_reg.set_nack(false);
        ep_reg.set_data_phase(false);
        ep_reg.set_max_packet_size(max_packet_size);
        self.endpoint_regs[0].set(ep_reg.0);

        self.next_ram_word = in_aligned_word_addr + total_words;

        Ok(ControlPipe {
            regs: unsafe { UsbCtrl::steal() },
            max_packet_size: max_packet_size as usize,
            setup_buffer: EndpointBuffer::setup_buffer(),
            out_buffer: EndpointBuffer::new(out_aligned_word_addr, packet_words),
            in_buffer: EndpointBuffer::new(in_aligned_word_addr, packet_words),
        })
    }
}

impl<'a> driver::Driver<'a> for Driver<'a> {
    type EndpointIn = Endpoint<'a, In>;
    type EndpointOut = Endpoint<'a, Out>;
    type Bus = Bus;
    type ControlPipe = ControlPipe;

    fn alloc_endpoint_out(
        &mut self,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval_ms: u8,
    ) -> Result<Self::EndpointOut, EndpointAllocError> {
        self.alloc_endpoint(ep_type, max_packet_size, interval_ms)
    }

    fn alloc_endpoint_in(
        &mut self,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval_ms: u8,
    ) -> Result<Self::EndpointIn, EndpointAllocError> {
        self.alloc_endpoint(ep_type, max_packet_size, interval_ms)
    }

    fn start(mut self, control_max_packet_size: u16) -> (Self::Bus, Self::ControlPipe) {
        let control_pipe = self.alloc_control_pipe(control_max_packet_size).unwrap();

        enable_custom_interrupt(USB_INTERRUPT_CODE);
        self.regs
            .config()
            .write(|w| w.interrupt_enable().set_bit().pull_up_enable().set_bit());
        defmt::debug!("[USB] Started (pull-up enabled)");

        (
            Bus {
                regs: self.regs,
                connected: false,
            },
            control_pipe,
        )
    }
}

#[derive(Copy, Clone)]
enum WordAddr {
    SetupBuffer(usize),
    EndpointDescriptor(usize),
}

impl WordAddr {
    fn is_endpoint_desc(&self) -> bool {
        matches!(self, WordAddr::EndpointDescriptor(_))
    }
}

struct EndpointBuffer {
    word_addr: WordAddr,
    packet_words: usize,
}

impl EndpointBuffer {
    fn new(desc_word_addr: usize, packet_words: usize) -> Self {
        Self {
            word_addr: WordAddr::EndpointDescriptor(desc_word_addr),
            packet_words,
        }
    }

    fn setup_buffer() -> Self {
        Self {
            word_addr: WordAddr::SetupBuffer(SETUP_DATA_OFFSET >> 2),
            packet_words: 2,
        }
    }

    fn desc_word_addr(&self) -> usize {
        match self.word_addr {
            WordAddr::EndpointDescriptor(addr) => addr,
            _ => defmt::panic!(),
        }
    }

    fn data_word_addr(&self) -> usize {
        match self.word_addr {
            WordAddr::EndpointDescriptor(addr) => addr + 3,
            WordAddr::SetupBuffer(addr) => addr,
        }
    }

    /// Configures the descriptor and returns the linked list `head` for
    /// EndpointRegister or descriptor `next` field
    fn write_descriptor(
        &mut self,
        length: u16,
        dir: Direction,
        interrupt: bool,
        completion_on_full: bool,
        data1_on_completion: bool,
    ) -> u16 {
        let desc_word_addr = self.desc_word_addr();

        let mut w0 = EndpointDescriptorW0(0);
        w0.set_code(0xf);
        let mut w1 = EndpointDescriptorW1(0);
        w1.set_length(length);
        let mut w2 = EndpointDescriptorW2(0);
        w2.set_direction(match dir {
            Direction::In => true,
            Direction::Out => false,
        });
        w2.set_interrupt(interrupt);
        w2.set_completion_on_full(completion_on_full);
        w2.set_data1_on_completion(data1_on_completion);

        unsafe {
            let ptr = UsbCtrl::ptr().cast::<u32>().add(desc_word_addr) as *mut u32;
            ptr.add(0).write_volatile(w0.0);
            ptr.add(1).write_volatile(w1.0);
            ptr.add(2).write_volatile(w2.0);
        }

        // 16-byte aligned
        (desc_word_addr >> 2) as u16
    }

    fn read_descriptor_w0(&mut self) -> EndpointDescriptorW0 {
        unsafe {
            let ptr = UsbCtrl::ptr().cast::<u32>().add(self.desc_word_addr()) as *mut u32;
            EndpointDescriptorW0(ptr.read_volatile())
        }
    }

    fn read(&mut self, buf: &mut [u8]) {
        defmt::assert!(buf.len() <= self.packet_words << 2);
        let num_whole_words = buf.len() >> 2;
        let num_rem_bytes = buf.len() % 4;
        compiler_fence(Ordering::SeqCst);
        unsafe {
            let mem_ptr = UsbCtrl::ptr().cast::<u32>().add(self.data_word_addr());
            let buf_whole_words =
                slice::from_raw_parts_mut(buf.as_mut_ptr().cast::<u32>(), num_whole_words);
            for (i, word_mut) in buf_whole_words.iter_mut().enumerate() {
                *word_mut = mem_ptr.add(i).read_volatile();
            }
            if buf.len() & 0x3 > 0 {
                let rem = mem_ptr.add(num_whole_words).read_volatile().to_le_bytes();
                let rem_slice = &rem[0..num_rem_bytes];
                buf[(num_whole_words << 2)..].copy_from_slice(rem_slice);
            }
        }
        compiler_fence(Ordering::SeqCst);
    }

    fn write(&mut self, buf: &[u8]) {
        defmt::assert!(buf.len() <= self.packet_words << 2);
        defmt::assert!(self.word_addr.is_endpoint_desc());
        let num_words = (buf.len() + 3) >> 2;
        defmt::trace!(
            "[USB] EndpointBuffer write buf.len()={} num_words={}",
            buf.len(),
            num_words
        );
        unsafe {
            let mem_ptr_base = UsbCtrl::ptr().cast::<u32>().add(self.data_word_addr()) as *mut u32;
            defmt::trace!(
                "[USB] EndpointBuffer write mem_ptr={:#08X} data_word_addr={:#08X}",
                mem_ptr_base,
                self.data_word_addr()
            );
            let buf_words = slice::from_raw_parts(buf.as_ptr().cast::<u32>(), num_words);
            for (i, word) in buf_words.iter().enumerate() {
                mem_ptr_base.add(i).write_volatile(*word);
            }
        }
    }
}

pub trait Dir {
    fn dir() -> Direction;
}

pub enum In {}

impl Dir for In {
    fn dir() -> Direction {
        Direction::In
    }
}

pub enum Out {}

impl Dir for Out {
    fn dir() -> Direction {
        Direction::Out
    }
}

fn enable_endpoint_interrupt(regs: &UsbCtrl, ep_index: usize) {
    let ep_mask = 1 << ep_index;
    critical_section::with(|_| {
        regs.interrupt_enable().modify(|r, w| unsafe {
            w.enable_endpoints()
                .bits(r.enable_endpoints().bits() | ep_mask)
        });
    });
}

fn is_endpoint_interrupt_set(regs: &UsbCtrl, ep_index: usize) -> bool {
    let ep_mask = 1 << ep_index;
    regs.interrupt().read().endpoints().bits() & ep_mask != 0
}

fn disable_endpoint_interrupt(regs: &UsbCtrl, ep_index: usize) {
    let ep_mask = 1 << ep_index;
    critical_section::with(|_| {
        regs.interrupt()
            .write(|w| unsafe { w.endpoints().bits(ep_mask) });
    });
}

pub struct Endpoint<'a, DIR: Dir> {
    _phantom: PhantomData<(&'a mut UsbCtrl, DIR)>,
    info: EndpointInfo,
    buffer: EndpointBuffer,
}

impl<'a> driver::Endpoint for Endpoint<'a, In> {
    fn info(&self) -> &EndpointInfo {
        &self.info
    }

    async fn wait_enabled(&mut self) {
        let ep_index = self.info.addr.index();
        defmt::debug!("[USB] wait_enabled dir=In ep={}", ep_index);

        poll_fn(|cx| {
            ENDPOINT_WAKERS[ep_index].register(cx.waker());

            if read_endpoint_reg(ep_index).enable() {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;

        defmt::debug!("[USB] wait_enabled DONE dir=In ep={}", ep_index);
    }
}

impl<'a> driver::Endpoint for Endpoint<'a, Out> {
    fn info(&self) -> &EndpointInfo {
        &self.info
    }

    async fn wait_enabled(&mut self) {
        let ep_index = self.info.addr.index();
        defmt::debug!("[USB] wait_enabled dir=Out ep={}", ep_index);

        poll_fn(|cx| {
            ENDPOINT_WAKERS[ep_index].register(cx.waker());

            if read_endpoint_reg(ep_index).enable() {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;

        defmt::debug!("[USB] wait_enabled DONE dir=Out ep={}", ep_index);
    }
}

impl<'a> driver::EndpointIn for Endpoint<'a, In> {
    async fn write(&mut self, data: &[u8]) -> Result<(), EndpointError> {
        let ep_index = self.info.addr.index();
        defmt::debug!("[USB] write ep={} data_len={}", ep_index, data.len());

        if data.len() > self.info.max_packet_size as usize {
            return Err(EndpointError::BufferOverflow);
        }

        let is_full_packet = data.len() == self.info.max_packet_size as usize;
        let completion_on_full_flag = is_full_packet;

        let head = self.buffer.write_descriptor(
            data.len() as u16,
            Direction::In,
            true,
            completion_on_full_flag,
            false,
        );
        self.buffer.write(data);
        modify_endpoint_reg(ep_index, |ep_reg| {
            ep_reg.set_enable(true);
            ep_reg.set_stall(false);
            ep_reg.set_nack(false);
            ep_reg.set_head(head);
        });

        let regs = unsafe { UsbCtrl::steal() };
        enable_endpoint_interrupt(&regs, ep_index);

        poll_fn(|cx| {
            ENDPOINT_WAKERS[ep_index].register(cx.waker());

            if is_endpoint_interrupt_set(&regs, ep_index) || !read_endpoint_reg(0).enable() {
                disable_endpoint_interrupt(&regs, ep_index);
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;

        let desc_w0 = self.buffer.read_descriptor_w0();
        defmt::debug!(
            "[USB] write DONE ep={} code={} offset={}",
            ep_index,
            desc_w0.code(),
            desc_w0.offset()
        );

        if desc_w0.code() == 0 {
            Ok(())
        } else {
            defmt::warn!(
                "[USB] endpoint error ep={} code={}",
                ep_index,
                desc_w0.code()
            );
            Err(EndpointError::Disabled)
        }
    }
}

impl<'a> driver::EndpointOut for Endpoint<'a, Out> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, EndpointError> {
        let ep_index = self.info.addr.index();
        defmt::debug!("[USB] read ep={}", ep_index);

        let head =
            self.buffer
                .write_descriptor(buf.len() as u16, Direction::Out, true, true, false);
        modify_endpoint_reg(ep_index, |ep_reg| {
            ep_reg.set_enable(true);
            ep_reg.set_stall(false);
            ep_reg.set_nack(false);
            ep_reg.set_head(head);
        });

        let regs = unsafe { UsbCtrl::steal() };
        enable_endpoint_interrupt(&regs, ep_index);

        poll_fn(|cx| {
            ENDPOINT_WAKERS[ep_index].register(cx.waker());

            if is_endpoint_interrupt_set(&regs, ep_index) || !read_endpoint_reg(0).enable() {
                disable_endpoint_interrupt(&regs, ep_index);
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;

        let desc_w0 = self.buffer.read_descriptor_w0();
        defmt::debug!(
            "[USB] read DONE code={} offset={}",
            desc_w0.code(),
            desc_w0.offset()
        );
        if desc_w0.code() == 0 {
            self.buffer.read(buf);
            Ok(buf.len())
        } else if desc_w0.offset() as usize > buf.len() {
            Err(EndpointError::BufferOverflow)
        } else {
            defmt::warn!(
                "[USB] endpoint error ep={} code={}",
                ep_index,
                desc_w0.code()
            );
            Err(EndpointError::Disabled)
        }
    }
}

fn read_endpoint_reg(endpoint_index: usize) -> EndpointRegister {
    unsafe {
        let ptr = (UsbCtrl::ptr() as *const u32).add(endpoint_index);
        EndpointRegister(ptr.read_volatile())
    }
}

fn modify_endpoint_reg<F>(endpoint_index: usize, f: F)
where
    F: FnOnce(&mut EndpointRegister),
{
    defmt::assert!(endpoint_index < ENDPOINT_COUNT);
    let regs = unsafe { UsbCtrl::steal() };
    let ptr = unsafe { (UsbCtrl::ptr() as *mut u32).add(endpoint_index) };

    critical_section::with(|_| {
        let mut ep_reg = unsafe {
            // Halt endpoint for atomicity
            regs.halt().write(|w| {
                w.endpoint_id()
                    .bits(endpoint_index as u8)
                    .enable()
                    .set_bit()
            });
            while regs.halt().read().effective().bit_is_clear() {}
            EndpointRegister(ptr.read_volatile())
        };
        f(&mut ep_reg);
        unsafe {
            ptr.write_volatile(ep_reg.0);

            // Un-halt endpoint
            regs.halt().write(|w| w.enable().clear_bit());
        }
    })
}

pub struct Bus {
    regs: UsbCtrl,
    connected: bool,
}

impl Bus {
    fn disable_all_endpoints(&mut self) {
        critical_section::with(|_| {
            self.regs
                .interrupt_enable()
                .modify(|_, w| unsafe { w.enable_endpoints().bits(0) })
        });
        for (ep_index, ep_waker) in ENDPOINT_WAKERS.iter().enumerate() {
            modify_endpoint_reg(ep_index, |ep_reg| {
                ep_reg.set_enable(false);
                ep_reg.set_head(0);
                ep_reg.set_stall(false);
                ep_reg.set_nack(false);
                ep_reg.set_data_phase(false);
            });
            ep_waker.wake()
        }
    }

    fn clear_address(&mut self) {
        self.regs.address().write(|w| unsafe { w.bits(0) });
    }

    fn enable_bus_interrupts(&mut self) {
        critical_section::with(|_| {
            self.regs.interrupt_enable().modify(|_, w| {
                w.enable_connect()
                    .set_bit()
                    .enable_disconnect()
                    .set_bit()
                    .enable_resume()
                    .set_bit()
                    .enable_reset()
                    .set_bit()
                    .enable_suspend()
                    .set_bit()
            });
        });
    }

    fn is_bus_interrupt(isr: &slab_pac::usb_ctrl::interrupt::R) -> bool {
        isr.connect().bit_is_set()
            || isr.disconnect().bit_is_set()
            || isr.resume().bit_is_set()
            || isr.reset().bit_is_set()
            || isr.suspend().bit_is_set()
    }

    fn disable_bus_interrupts(regs: &UsbCtrl) {
        critical_section::with(|_| {
            regs.interrupt_enable().modify(|_, w| {
                w.enable_connect()
                    .clear_bit()
                    .enable_disconnect()
                    .clear_bit()
                    .enable_resume()
                    .clear_bit()
                    .enable_reset()
                    .clear_bit()
                    .enable_suspend()
                    .clear_bit()
            });
        });
    }
}

impl driver::Bus for Bus {
    async fn enable(&mut self) {}

    async fn disable(&mut self) {}

    async fn poll(&mut self) -> Event {
        self.enable_bus_interrupts();
        poll_fn(move |cx| {
            BUS_WAKER.register(cx.waker());
            let info_status = self.regs.info_reg().read();
            let isr = self.regs.interrupt().read();

            if !self.connected
                && (info_status.power_detected().bit_is_set() || isr.connect().bit_is_set())
            {
                defmt::debug!("[USB] connected");
                self.connected = true;
                self.regs
                    .interrupt()
                    .write(|w| w.connect().clear_bit_by_one());
                return Poll::Ready(Event::PowerDetected);
            } else if self.connected
                && (info_status.power_detected().bit_is_clear() || isr.disconnect().bit_is_set())
            {
                defmt::debug!("[USB] disconnected");
                self.regs
                    .interrupt()
                    .write(|w| w.disconnect().clear_bit_by_one());
                self.connected = false;
                self.clear_address();
                self.disable_all_endpoints();
                self.regs
                    .interrupt()
                    .write(|w| w.disconnect().clear_bit_by_one());
                return Poll::Ready(Event::PowerRemoved);
            }

            if isr.resume().bit_is_set() {
                defmt::debug!("[USB] resumed");
                self.regs
                    .interrupt()
                    .write(|w| w.resume().clear_bit_by_one());
                return Poll::Ready(Event::Resume);
            }

            if isr.reset().bit_is_set() {
                defmt::debug!("[USB] reset");
                self.regs
                    .interrupt()
                    .write(|w| w.reset().clear_bit_by_one());
                self.clear_address();
                self.disable_all_endpoints();

                // Enable endpoint 0 for setup
                modify_endpoint_reg(0, |ep_reg| {
                    ep_reg.set_enable(true);
                    ep_reg.set_stall(false);
                    ep_reg.set_nack(false);
                    ep_reg.set_head(0);
                    ep_reg.set_data_phase(false);
                });
                return Poll::Ready(Event::Reset);
            }

            if isr.suspend().bit_is_set() {
                defmt::debug!("[USB] suspended");
                self.regs
                    .interrupt()
                    .write(|w| w.suspend().clear_bit_by_one());
                return Poll::Ready(Event::Suspend);
            }

            Poll::Pending
        })
        .await
    }

    fn endpoint_set_enabled(&mut self, ep_addr: EndpointAddress, enabled: bool) {
        let ep_index = ep_addr.index();
        defmt::debug!("[USB] set_enabled ep={} enabled={}", ep_index, enabled);

        modify_endpoint_reg(ep_index, |ep_reg| {
            ep_reg.set_enable(enabled);
            ep_reg.set_stall(false);
            ep_reg.set_nack(false);
            ep_reg.set_head(0);
            ep_reg.set_data_phase(false);
        });
        ENDPOINT_WAKERS[ep_index].wake()
    }

    fn endpoint_set_stalled(&mut self, ep_addr: EndpointAddress, stalled: bool) {
        let ep_index = ep_addr.index();
        defmt::debug!("[USB] set_stalled ep={} stalled={}", ep_index, stalled);
        modify_endpoint_reg(ep_index, |ep_reg| {
            ep_reg.set_stall(stalled);
        });
        ENDPOINT_WAKERS[ep_index].wake()
    }

    fn endpoint_is_stalled(&mut self, ep_addr: EndpointAddress) -> bool {
        let ep_reg = read_endpoint_reg(ep_addr.index());
        ep_reg.stall()
    }

    async fn remote_wakeup(&mut self) -> Result<(), driver::Unsupported> {
        Err(driver::Unsupported)
    }
}

impl Drop for Bus {
    fn drop(&mut self) {
        disable_custom_interrupt(USB_INTERRUPT_CODE);
    }
}

pub struct ControlPipe {
    regs: UsbCtrl,
    max_packet_size: usize,
    setup_buffer: EndpointBuffer,
    out_buffer: EndpointBuffer,
    in_buffer: EndpointBuffer,
}

impl ControlPipe {
    async fn receive_status_out(&mut self) {
        defmt::trace!("[USB] control: receive_status_out");

        let head = self
            .out_buffer
            .write_descriptor(0, Direction::Out, true, true, false);
        modify_endpoint_reg(0, |ep_reg| {
            ep_reg.set_enable(true);
            ep_reg.set_stall(false);
            ep_reg.set_nack(false);
            ep_reg.set_data_phase(true);
            ep_reg.set_head(head);
        });

        defmt::trace!("[USB] control: OUT ZLP configured, assuming HW handles it.");
    }

    async fn send_status_in(&mut self, reject: bool) -> u8 {
        defmt::trace!("[USB] control: send_status_in reject={}", reject);

        let head = self
            .in_buffer
            .write_descriptor(0, Direction::In, true, true, false);
        modify_endpoint_reg(0, |ep_reg| {
            ep_reg.set_enable(true);
            ep_reg.set_stall(reject);
            ep_reg.set_nack(false);
            ep_reg.set_data_phase(true);
            ep_reg.set_head(head);
        });

        let regs = unsafe { UsbCtrl::steal() };
        enable_endpoint_interrupt(&regs, 0);

        poll_fn(|cx| {
            ENDPOINT_WAKERS[0].register(cx.waker());

            if is_endpoint_interrupt_set(&regs, 0) || !read_endpoint_reg(0).enable() {
                disable_endpoint_interrupt(&regs, 0);
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;

        let desc_w0 = self.in_buffer.read_descriptor_w0();
        defmt::trace!("[USB] control: send_status_in done code={}", desc_w0.code());
        desc_w0.code()
    }
}

impl driver::ControlPipe for ControlPipe {
    fn max_packet_size(&self) -> usize {
        self.max_packet_size
    }

    async fn setup(&mut self) -> [u8; 8] {
        defmt::debug!("[USB] Waiting for SETUP");

        modify_endpoint_reg(0, |ep_reg| {
            ep_reg.set_enable(true);
            ep_reg.set_nack(false);
            ep_reg.set_data_phase(false);
            ep_reg.set_head(0);
        });

        let regs = unsafe { UsbCtrl::steal() };
        critical_section::with(|_| {
            regs.interrupt_enable()
                .modify(|_, w| w.enable_ep0setup().set_bit());
        });

        poll_fn(|cx| {
            ENDPOINT_WAKERS[0].register(cx.waker());

            if regs.interrupt().read().ep0setup().bit_is_set() {
                regs.interrupt().write(|w| w.ep0setup().clear_bit_by_one());
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;

        let mut buf = [0; 8];
        self.setup_buffer.read(&mut buf);

        defmt::debug!("[USB] SETUP read");
        defmt::trace!("[USB] control: setup bytes={:#02x}", buf);
        buf
    }

    async fn data_out(
        &mut self,
        buf: &mut [u8],
        first: bool,
        last: bool,
    ) -> Result<usize, EndpointError> {
        defmt::debug!(
            "[USB] control: data_out len={} first={} last={}",
            buf.len(),
            first,
            last
        );

        let head =
            self.out_buffer
                .write_descriptor(buf.len() as u16, Direction::Out, true, true, false);
        modify_endpoint_reg(0, |ep_reg| {
            ep_reg.set_enable(true);
            ep_reg.set_stall(false);
            ep_reg.set_nack(false);
            ep_reg.set_head(head);
            if first {
                ep_reg.set_data_phase(true);
            }
        });

        let regs = unsafe { UsbCtrl::steal() };
        enable_endpoint_interrupt(&regs, 0);

        poll_fn(|cx| {
            ENDPOINT_WAKERS[0].register(cx.waker());

            if is_endpoint_interrupt_set(&regs, 0) || !read_endpoint_reg(0).enable() {
                disable_endpoint_interrupt(&regs, 0);
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;

        let desc_w0 = self.out_buffer.read_descriptor_w0();
        defmt::debug!(
            "[USB] control: data_out DONE code={} offset={}",
            desc_w0.code(),
            desc_w0.offset()
        );
        if desc_w0.code() == 0 {
            self.out_buffer.read(buf);
            Ok(buf.len())
        } else if desc_w0.offset() as usize > buf.len() {
            Err(EndpointError::BufferOverflow)
        } else {
            defmt::warn!("[USB] endpoint error ep=0 code={}", desc_w0.code());
            Err(EndpointError::Disabled)
        }
    }

    async fn data_in(&mut self, data: &[u8], first: bool, last: bool) -> Result<(), EndpointError> {
        defmt::debug!(
            "[USB] control: data_in len={} first={} last={}",
            data.len(),
            first,
            last
        );

        defmt::trace!("[USB] control: data_in bytes={:#02x}", data);

        if data.len() > self.max_packet_size {
            return Err(EndpointError::BufferOverflow);
        }

        let head =
            self.in_buffer
                .write_descriptor(data.len() as u16, Direction::In, true, !last, false);
        self.in_buffer.write(data);
        modify_endpoint_reg(0, |ep_reg| {
            ep_reg.set_enable(true);
            ep_reg.set_stall(false);
            ep_reg.set_nack(false);
            ep_reg.set_head(head);
            if first {
                ep_reg.set_data_phase(true);
            }
        });

        let regs = unsafe { UsbCtrl::steal() };
        enable_endpoint_interrupt(&regs, 0);
        poll_fn(|cx| {
            ENDPOINT_WAKERS[0].register(cx.waker());

            if is_endpoint_interrupt_set(&regs, 0) || !read_endpoint_reg(0).enable() {
                disable_endpoint_interrupt(&regs, 0);
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;

        let desc_w0 = self.in_buffer.read_descriptor_w0();
        if desc_w0.code() == 0 {
            defmt::debug!(
                "[USB] control: data_in successful code={} offset={}",
                desc_w0.code(),
                desc_w0.offset()
            );
        } else {
            defmt::debug!(
                "[USB] control: data_in failed code={} offset={}",
                desc_w0.code(),
                desc_w0.offset()
            );
        }

        if last {
            defmt::debug!("[USB] data_in status stage");
            self.receive_status_out().await;
        }

        if desc_w0.code() == 0 {
            Ok(())
        } else {
            defmt::warn!("[USB] endpoint error ep=0 code={}", desc_w0.code());
            Err(EndpointError::Disabled)
        }
    }

    async fn accept(&mut self) {
        // Called after SETUP (no data) or OUT data. Needs IN status.
        defmt::debug!("[USB] control: accept");
        let code = self.send_status_in(false).await;
        if code == 0 {
            defmt::debug!("[USB] control: accept IN ZLP successful code={}", code,);
        } else {
            defmt::warn!("[USB] control: accept IN ZLP failed code={}", code,);
        }
    }

    async fn reject(&mut self) {
        defmt::debug!("[USB] control: reject");
        modify_endpoint_reg(0, |ep_reg| {
            ep_reg.set_stall(true);
        });
    }

    async fn accept_set_address(&mut self, addr: u8) {
        defmt::debug!("[USB] control: setting addr={}", addr);

        self.regs
            .address()
            .write(|w| unsafe { w.trigger().set_bit().value().bits(addr) });
        defmt::debug!("[USB] control: address reg set addr={} trigger=1", addr);

        let code = self.send_status_in(false).await;
        if code == 0 {
            defmt::debug!("[USB] control: set address IN ZLP status successful");
            // hardware automatically enables the address now.
        } else {
            defmt::warn!(
                "[USB] control: set_address IN ZLP status failed code={} addr={}.",
                code,
                addr
            );
            // trigger bit gets cleared by hardware anyway,
        }
    }
}

pub fn handle_usb_interrupt() {
    let regs = unsafe { UsbCtrl::steal() };
    critical_section::with(|_| {
        let isr = regs.interrupt().read();

        if Bus::is_bus_interrupt(&isr) {
            defmt::trace!("[USB] IRQ: Bus interrupt isr={:#08x}", isr.bits());
            Bus::disable_bus_interrupts(&regs);
            BUS_WAKER.wake();
        }
        if isr.ep0setup().bit_is_set() {
            defmt::trace!("[USB] IRQ: EP0 setup isr={:#08x}", isr.bits());
            regs.interrupt_enable()
                .modify(|_, w| w.enable_ep0setup().clear_bit());
            ENDPOINT_WAKERS[0].wake();
        }
        let ep_ints = isr.endpoints().bits();
        if ep_ints != 0 {
            defmt::trace!("[USB] IRQ isr={:#08x}", isr.bits());

            let mut eps_to_disable = 0;
            for (ep_index, ep_waker) in ENDPOINT_WAKERS.iter().enumerate() {
                let ep_mask = 1 << ep_index;
                if ep_ints & ep_mask != 0 {
                    defmt::trace!("[USB] IRQ waking ep={}", ep_index);
                    eps_to_disable |= ep_mask;
                    ep_waker.wake();
                }
            }
            defmt::trace!(
                "[USB] IRQ disabling interrupts eps_to_disable={:#02x}",
                eps_to_disable
            );
            regs.interrupt_enable().modify(|r, w| unsafe {
                w.enable_endpoints()
                    .bits(r.enable_endpoints().bits() & !eps_to_disable)
            });
        }
    })
}
