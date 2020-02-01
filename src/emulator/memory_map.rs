use crate::emulator::Emulator;
use crate::ppu::Ppu;

const UNDEFINED_BYTE: u8 = 0xFF;

impl Emulator {
    pub fn read_mapped(&self, address: u16) -> u8 {
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
            0xFEA0 ..= 0xFEFF => UNDEFINED_BYTE,
            0xFF00 ..= 0xFF7F => self.read_ioreg(address),
            0xFF80 ..= 0xFFFE => self.hram[(address - 0xFF80) as usize],
            0xFFFF => self.interrupts.flags_as_byte(),
        }
    }

    pub fn write_cycle(&mut self, address: u16, value: u8) {
        self.step();
        self.write_mapped(address, value);
    }

    pub fn write_mapped(&mut self, address: u16, value: u8) {
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

    pub fn read_ioreg(&self, address: u16) -> u8 {
        match address {
            // Unused registers
            0xFF03 | 0xFF08..=0xFF0E | 0xFF1F | 0xFF27..=0xFF2f | 0xFF4E | 0xFF57..=0xFF67 | 0xFF78..=0xFF7F
                => UNDEFINED_BYTE,

            0xFF00 => self.joypad.as_byte(),
            0xFF04 => self.timer.divider,
            0xFF05 => self.timer.counter,
            0xFF06 => self.timer.modulo,

            0xFF0F => self.interrupts.as_byte(),
            0xFF40 => self.ppu.control_as_byte(),
            0xFF41 => self.ppu.stat_as_byte(),
            0xFF42 => self.ppu.scroll_y,
            0xFF43 => self.ppu.scroll_x,
            0xFF44 => self.ppu.current_line(),
            0xFF47 => UNDEFINED_BYTE, // TODO: Read palette data
            0xFF48 => UNDEFINED_BYTE,
            0xFF49 => UNDEFINED_BYTE,

            0xFF50 => if self.booted { 1 } else { 0 },
            _ => {
                eprintln!("Reading from unknown IO register: {:#06X}", address);
                UNDEFINED_BYTE
            }
        }
    }

    pub fn write_ioreg(&mut self, address: u16, value: u8) {
        match address {
            // Unused registers
            0xFF03 | 0xFF08..=0xFF0E | 0xFF1F | 0xFF27..=0xFF2f | 0xFF4E | 0xFF57..=0xFF67 | 0xFF78..=0xFF7F
                => (),

            0xFF00 => self.joypad.from_byte(value),

            // Serial
            0xFF01 => (),
            0xFF02 => (),

            0xFF04 => self.timer.divider = 0,
            0xFF05 => self.timer.counter = value,
            0xFF06 => self.timer.modulo = value,
            0xFF07 => self.timer.mode_from_byte(value),

            0xFF0F => self.interrupts.from_byte(value),

            // Audio
            0xFF10..=0xFF3F => (),

            0xFF40 => self.ppu.control_from_byte(value),
            0xFF41 => self.ppu.stat_from_byte(value),
            0xFF42 => self.ppu.scroll_y = value,
            0xFF43 => self.ppu.scroll_x = value,
            0xFF44 => self.ppu.reset_current_line(),
            0xFF46 => self.init_dma_transfer(value),
            0xFF47 => self.ppu.palette = Ppu::palette_from_byte(value),
            0xFF48 => self.ppu.obj_palette0 = Ppu::palette_from_byte(value),
            0xFF49 => self.ppu.obj_palette1 = Ppu::palette_from_byte(value),

            0xFF50 => self.booted = true,
            0xFFFF => self.interrupts.flags_from_byte(value),
            _ => eprintln!("Writing {:#04X} to unknown IO register: {:#06X}", value, address),
        }
    }
}
