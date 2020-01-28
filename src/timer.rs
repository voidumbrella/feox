use crate::interrupts::{Interrupt, InterruptQueue};

const MODE0: u16 = 256;
const MODE1: u16 = 4;
const MODE2: u16 = 16;
const MODE3: u16 = 128;

#[derive(Debug)]
pub struct Timer {
    pub counter: u8,
    pub modulo: u8,
    interrupt_ready: bool,
    enabled: bool,
    frequency: u16,
    cycles: u16,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            counter: 0,
            enabled: false,
            interrupt_ready: false,
            modulo: 0,
            frequency: MODE0,
            cycles: 0,
        }
    }

    pub fn step(&mut self, interrupts: &mut InterruptQueue) {
        if !self.enabled {
            return;
        }

        self.cycles += 1;

        if self.interrupt_ready {
            self.interrupt_ready = false;
            self.counter = self.modulo;
            interrupts.request_interrupt(Interrupt::Timer);
        }

        if self.cycles >= self.frequency {
            self.cycles = 0;
            let (new_counter, overflowed) = self.counter.overflowing_add(1);
            self.interrupt_ready = overflowed;
            self.counter = new_counter;
        }
    }

    pub fn mode_from_byte(&mut self, byte: u8) {
        self.enabled = (byte & 1 << 2) != 0;
        self.frequency = match byte & 0b11 {
            0 => MODE0,
            1 => MODE1,
            2 => MODE2,
            3 => MODE3,
            _ => unreachable!(),
        };
    }
}
