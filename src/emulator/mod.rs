mod memory_map;

use crate::ppu::Ppu;
use crate::interrupts::InterruptQueue;
use crate::joypad::{Joypad, Button};
use crate::timer::Timer;

pub struct Emulator {
    cycles: u32,
    timer: Timer,
    pub ppu: Ppu,
    pub interrupts: InterruptQueue,
    pub joypad: Joypad,
    booted: bool,
    boot_rom: [u8; 0x100],
    rom: [u8; 0x7FFF - 0x0000 + 1],
    cram: [u8; 0xBFFF - 0xA000 + 1],
    wram: [u8; 0xDFFF - 0xC000 + 1],
    hram: [u8; 0xFFFF - 0xFF80 + 1],
}

impl Default for Emulator {
    fn default() -> Self {
        Self::new()
    }
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            cycles: 0,
            ppu: Ppu::new(),
            joypad: Joypad::new(),
            timer: Timer::new(),
            booted: false,
            interrupts: InterruptQueue::new(),
            boot_rom: [0; 0x100],
            rom: [0; 0x8000],
            cram: [0; 0x2000],
            wram: [0; 0x2000],
            hram: [0; 0x80],
        }
    }

    pub fn catch_up_cycles(&mut self) -> u32 {
        let cycles = self.cycles;
        self.ppu.step(cycles, &mut self.interrupts);
        self.timer.step(cycles, &mut self.interrupts);
        self.cycles = 0;
        cycles
    }

    pub fn step(&mut self) {
        self.cycles += 1;
    }

    pub fn load_bootrom<T: std::io::Read>(&mut self, src: &mut T) -> Result<usize, std::io::Error> {
        src.read(&mut self.boot_rom)
    }

    pub fn load_rom<T: std::io::Read>(&mut self, src: &mut T) -> Result<usize, std::io::Error> {
        src.read(&mut self.rom)
    }

    pub fn read_cycle(&mut self, address: u16) -> u8 {
        self.step();
        self.read_mapped(address)
    }

    pub fn joypad_press(&mut self, pressed: Button) {
        self.joypad.press_button(&mut self.interrupts, pressed);
    }

    pub fn joypad_clear(&mut self, pressed: Button) {
        self.joypad.clear_button(pressed);
    }

    fn init_dma_transfer(&mut self, offset: u8) {
        // TODO: This probably should not happen instantaneously
        // (Hence the name "init")
        let base = offset as u16 * 0x100;
        for addr in 0..0xA0 {
            let byte = self.read_mapped(base + addr);
            self.ppu.write_oam(addr, byte);
        }
    }
}
