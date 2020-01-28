mod control;

use control::PpuControl;

const VRAM_SIZE: usize = 0x2000;
const OAM_SIZE: usize = 0xA0;

#[allow(dead_code)]
enum PpuMode {
    InOam,
    InVram,
    HBlank,
    VBlank,
}

pub struct Ppu {
    cycles: usize,
    control: PpuControl,
    mode: PpuMode,
    vram: [u8; VRAM_SIZE],
    oam: [u8; OAM_SIZE],
    pub current_line: u8,
    pub scroll_y: u8,
    pub scroll_x: u8,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            cycles: 0,
            control: PpuControl::default(),
            mode: PpuMode::InOam,
            vram: [0; VRAM_SIZE],
            oam: [0; OAM_SIZE],
            current_line: 0,
            scroll_x: 0,
            scroll_y: 0,
        }
    }

    pub fn step(&mut self) {
        if !self.control.lcd_on {
            return;
        }
        self.cycles += 1;
    }

    pub fn read_vram(&self, relative_address: u16) -> u8 {
        debug_assert!((0x0000..0x2000).contains(&relative_address), "addr: {:#04X}", relative_address);
        match self.mode {
            PpuMode::InVram => 0xFF,
            _ => self.vram[relative_address as usize],
        }
    }

    pub fn write_vram(&mut self, relative_address: u16, value: u8) {
        debug_assert!((0x0000..0x2000).contains(&relative_address), "addr: {:#04X}", relative_address);
        match self.mode {
            PpuMode::InVram => (),
            _ => self.vram[relative_address as usize] = value,
        }
    }

    pub fn read_oam(&self, relative_address: u16) -> u8 {
        debug_assert!((0x0000..0x00A0).contains(&relative_address), "addr: {:#04X}", relative_address);
        match self.mode {
            PpuMode::InOam => 0xFF,
            _ => self.oam[relative_address as usize],
        }
    }

    pub fn write_oam(&mut self, relative_address: u16, value: u8) {
        debug_assert!((0x0000..0x00A0).contains(&relative_address), "addr: {:#04X}", relative_address);
        match self.mode {
            PpuMode::InOam => (),
            _ => self.oam[relative_address as usize] = value,
        }
    }

    pub fn control_from_byte(&mut self, value: u8) {
        self.control = PpuControl::from_byte(value);
    }

    pub fn control_as_byte(&self) -> u8 {
        self.control.as_byte()
    }
}
