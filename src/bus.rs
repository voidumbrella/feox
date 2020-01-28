use crate::ppu::Ppu;
use crate::interrupts::InterruptQueue;
use crate::timer::Timer;

pub struct Bus {
    cycles: usize,
    timer: Timer,
    ppu: Ppu,
    booted: bool,
    interrupts: InterruptQueue,
    boot_rom: [u8; 0x00FF - 0x0000 + 1],
    rom: [u8; 0x7FFF - 0x0000 + 1],
    cram: [u8; 0xBFFF - 0xA000 + 1],
    wram: [u8; 0xDFFF - 0xC000 + 1],
    hram: [u8; 0xFFFF - 0xFF80 + 1],
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            cycles: 0,
            ppu: Ppu::new(),
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

    pub fn interrupts(&mut self) -> &mut InterruptQueue {
        &mut self.interrupts
    }

    pub fn catch_up_cycles(&mut self) {
        for _ in 0..self.cycles {
            self.ppu.step();
            self.timer.step(&mut self.interrupts);
        }
        self.cycles = 0;
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

    fn read_mapped(&self, address: u16) -> u8 {
        match address {
            0x0000 ..= 0x00FF => {
                if self.booted {
                    self.rom[address as usize]
                } else {
                    self.boot_rom[address as usize]
                }
            }
            0x0100 ..= 0x7FFF => self.rom[address as usize],

            0x8000 ..= 0x9FFF => self.ppu.read_vram(address - 0x8000),
            0xFE00 ..= 0xFE9F => self.ppu.read_oam(address - 0xFE00),

            0xA000 ..= 0xBFFF => self.cram[(address - 0xA000) as usize],
            0xC000 ..= 0xDFFF => self.wram[(address - 0xC000) as usize],
            // Echo ram, mapped to WRAM
            0xE000 ..= 0xFDFF => self.wram[(address - 0xE000) as usize],

            // Unused addresses
            0xFEA0 ..= 0xFEFF => 0,
            0xFF00 ..= 0xFF7F => self.read_ioreg(address),
            0xFF80 ..= 0xFFFE => self.hram[(address - 0xFF80) as usize],
            0xFFFF => self.interrupts.flags_as_byte(),
        }
    }

    pub fn write_cycle(&mut self, address: u16, value: u8) {
        self.step();
        self.write_mapped(address, value);
    }

    fn write_mapped(&mut self, address: u16, value: u8) {
        match address {
            0x0000 ..= 0x7FFF => eprintln!("Writing {:#04X} to ROM {:#06X}. \
                            Either this is a bug, or this cart uses a memory mapper.", value, address),

            0x8000 ..= 0x9FFF => self.ppu.write_vram(address - 0x8000, value),
            0xFE00 ..= 0xFE9F => self.ppu.write_oam(address - 0xFE00, value),

            0xA000 ..= 0xBFFF => self.cram[(address - 0xA000) as usize] = value,
            0xC000 ..= 0xDFFF => self.wram[(address - 0xC000) as usize] = value,
            // Echo ram, mapped to WRAM
            0xE000 ..= 0xFDFF => self.wram[(address - 0xE000) as usize] = value,
            // Unused addresses
            0xFEA0 ..= 0xFEFF => (),
            0xFF00 ..= 0xFF7F => self.write_ioreg(address, value),
            0xFF80 ..= 0xFFFE => self.hram[(address - 0xFF80) as usize] = value,
            0xFFFF => self.interrupts.flags_from_byte(value),
        }
    }

    fn read_ioreg(&self, address: u16) -> u8 {
        match address {
            0xFF05 => self.timer.counter,
            0xFF06 => self.timer.modulo,

            0xFF0F => self.interrupts.as_byte(),
            0xFF40 => self.ppu.control_as_byte(),
            0xFF42 => self.ppu.scroll_y,
            0xFF43 => self.ppu.scroll_x,
            0xFF44 => 0x90, // TODO: delete this hack
            // 0xFF44 => self.ppu.current_line,
            0xFF50 => if self.booted { 1 } else { 0 },
            _ => {
                eprintln!("Reading from unknown IO register: {:#06X}", address);
                0xFF
            }
        }
    }

    fn write_ioreg(&mut self, address: u16, value: u8) {
        match address {
            // print any serial output to console for now to debug with Blargg's ROMs
            0xFF01 => print!("{}", value as char),
            0xFF02 => (), // serial stuff as well

            0xFF05 => self.timer.counter = value,
            0xFF06 => self.timer.modulo = value,
            0xFF07 => self.timer.mode_from_byte(value),

            0xFF0F => self.interrupts.from_byte(value),
            0xFF40 => self.ppu.control_from_byte(value),
            0xFF42 => self.ppu.scroll_y = value,
            0xFF43 => self.ppu.scroll_x = value,
            0xFF44 => self.ppu.current_line = 0,
            0xFF50 => self.booted = true,
            0xFFFF => self.interrupts.flags_from_byte(value),
            _ => eprintln!("Writing {:#04X} to unknown IO register: {:#06X}", value, address),
        }
    }
}
