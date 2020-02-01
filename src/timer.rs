use crate::interrupts::{Interrupt, InterruptQueue};

const MODE0: u32 = 256;
const MODE1: u32 = 4;
const MODE2: u32 = 16;
const MODE3: u32 = 128;

#[derive(Debug)]
pub struct Timer {
    pub counter: u8,
    pub modulo: u8,
    pub divider: u8,
    enabled: bool,
    frequency: u32,
    cycles: u32,
    div_cycles: u32,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            counter: 0,
            modulo: 0,
            enabled: false,
            divider: 0,
            frequency: MODE0,
            cycles: 0,
            div_cycles: 0,
        }
    }

    pub fn step(&mut self, cycles: u32, interrupts: &mut InterruptQueue) {
        self.div_cycles += cycles;

        // TODO: this isn't really how the divider register works
        const DIV_FREQUENCY: u32 = 0x64;
        while self.div_cycles >= DIV_FREQUENCY {
            self.div_cycles -= DIV_FREQUENCY;
            self.divider = self.divider.wrapping_add(1);
        }

        if !self.enabled { return; }
        self.cycles += cycles;

        while self.cycles >= self.frequency {
            self.cycles -= self.frequency;
            let (new_counter, overflowed) = self.counter.overflowing_add(1);
            if overflowed {
                self.counter = self.modulo;
                interrupts.request_interrupt(Interrupt::Timer);
            } else {
                self.counter = new_counter;
            }
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
