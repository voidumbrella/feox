mod registers;
mod decode;
mod instructions;

use registers::Registers;
use crate::emulator::Emulator;
use crate::interrupts::Interrupt;

pub struct Cpu {
    regs: Registers,
    halted: bool,
    interrupt_enabled: bool,
    opcode: u8,
}

pub trait ByteSrc {
    fn read(&self, cpu: &mut Cpu, emulator: &mut Emulator) -> u8;
}

pub trait ByteDest {
    fn write(&self, cpu: &mut Cpu, emulator: &mut Emulator, value: u8);
}

pub trait WordSrc {
    fn read(&self, cpu: &mut Cpu, emulator: &mut Emulator) -> u16;
}

pub trait WordDest {
    fn write(&self, cpu: &mut Cpu, emulator: &mut Emulator, value: u16);
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            regs: Registers::default(),
            interrupt_enabled: false,
            halted: false,
            opcode: 0x00,
        }
    }

    fn handle_interrupt(&mut self, emulator: &mut Emulator) {
        emulator.step();
        emulator.step();
        let interrupt = emulator.interrupts.pop()
            .expect("Could not retrieve interrupt from queue");
        self.push(emulator, registers::Reg16::PC);
        match interrupt {
            Interrupt::VBlank => self.regs.pc = 0x40,
            Interrupt::Lcd => self.regs.pc = 0x48,
            Interrupt::Timer => self.regs.pc = 0x50,
            Interrupt::Joypad => self.regs.pc = 0x60,
        }
        self.interrupt_enabled = false;
    }

    pub fn step(&mut self, emulator: &mut Emulator) {
        if !self.halted {
            // println!("{:#04X}", self.opcode);
            self.opcode = self.fetch(emulator);
            self.decode(emulator);
        } else {
            emulator.step();
        }

        if emulator.interrupts.peek() {
            self.halted = false;
            if self.interrupt_enabled {
                self.handle_interrupt(emulator);
            }
        }
    }

    /// Fetch the byte pointed by the program counter and advance the program counter.
    fn fetch(&mut self, emulator: &mut Emulator) -> u8 {
        let val = self.read_byte(emulator, self.regs.pc);
        self.regs.pc = self.regs.pc.wrapping_add(1);
        val
    }

    /// Fetch the word pointed by the program counter and advance the program counter by 2.
    fn fetch_word(&mut self, emulator: &mut Emulator) -> u16 {
        let lo = self.fetch(emulator);
        let hi = self.fetch(emulator);
        (hi as u16) << 8| lo as u16
    }

    fn read_byte(&self, emulator: &mut Emulator, address: u16) -> u8 {
        emulator.read_cycle(address)
    }

    fn read_word(&self, emulator: &mut Emulator, address: u16) -> u16 {
        let lo = self.read_byte(emulator, address) as u16;
        let hi = self.read_byte(emulator, address.wrapping_add(1)) as u16;
        hi << 8 | lo
    }

    fn write_byte(&mut self, emulator: &mut Emulator, address: u16, value: u8) {
        emulator.write_cycle(address, value)
    }

    fn write_word(&mut self, emulator: &mut Emulator, address: u16, value: u16) {
        let lo = (value & 0xFF) as u8;
        let hi = (value >> 8) as u8;
        self.write_byte(emulator, address.wrapping_add(1), hi);
        self.write_byte(emulator, address, lo);
    }
}

impl std::fmt::Debug for Cpu {
    fn fmt(&self, w: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(w, "CPU \n\
               \t{:?}\n\
               \tLast executed {:#04X}\n\
               \tInterrupt enabled: {}",
               self.regs, self.opcode, self.interrupt_enabled)
    }
}
