mod control;

use control::PpuControl;
use crate::interrupts::{Interrupt, InterruptQueue};

const VRAM_SIZE: usize = 0x2000;
const OAM_SIZE: usize = 0xA0;
const OAM_ACCESS_CYCLES: u16 = 80;
const VRAM_ACCESS_CYCLES: u16 = 43;
const HBLANK_CYCLES: u16 = 51;
const SCANLINE_CYCLES: u16 = OAM_ACCESS_CYCLES + VRAM_ACCESS_CYCLES;

const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;
const TILE_WIDTH: usize = 8;
const TILE_SIZE: usize = 16;

const COLOR0: (u8, u8, u8) = (0xE0, 0xF8, 0xD0);
const COLOR1: (u8, u8, u8) = (0x88, 0xC0, 0x70);
const COLOR2: (u8, u8, u8) = (0x34, 0x68, 0x56);
const COLOR3: (u8, u8, u8) = (0x08, 0x18, 0x20);

#[allow(dead_code)]
enum PpuMode {
    InOam,
    InVram,
    HBlank,
    VBlank,
}

pub struct Ppu {
    framebuffer: [u8; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
    cycles: u16,
    control: PpuControl,
    mode: PpuMode,
    interrupt_enabled: [bool; 4],
    vram: [u8; VRAM_SIZE],
    oam: [u8; OAM_SIZE],
    palette: [(u8, u8, u8); 4],
    current_line: u8,
    lyc: u8,
    pub scroll_y: u8,
    pub scroll_x: u8,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            framebuffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
            cycles: 0,
            control: PpuControl::default(),
            mode: PpuMode::InOam,
            interrupt_enabled: [false; 4],
            vram: [0; VRAM_SIZE],
            oam: [0; OAM_SIZE],
            palette: [COLOR0, COLOR1, COLOR2, COLOR3],
            current_line: 0,
            lyc: 0,
            scroll_x: 0,
            scroll_y: 0,
        }
    }

    pub fn step(&mut self, cycles: u16, interrupts: &mut InterruptQueue) {
        if !self.control.lcd_on {
            return;
        }

        self.cycles += cycles;
        loop {
            match self.mode {
                PpuMode::InOam => {
                    if self.cycles >= OAM_ACCESS_CYCLES {
                        self.cycles -= OAM_ACCESS_CYCLES;
                        self.mode = PpuMode::InVram;
                        if self.interrupt_enabled[2] {
                            interrupts.request_interrupt(Interrupt::Lcd);
                        }
                    } else {
                        break;
                    }
                }
                PpuMode::InVram => {
                    if self.cycles >= VRAM_ACCESS_CYCLES {
                        self.cycles -= VRAM_ACCESS_CYCLES;
                        self.mode = PpuMode::HBlank;
                        self.render_scanline();
                        if self.interrupt_enabled[0] {
                            interrupts.request_interrupt(Interrupt::Lcd);
                        }
                    } else {
                        break;
                    }
                }
                PpuMode::HBlank => {
                    if self.cycles >= HBLANK_CYCLES {
                        self.cycles -= HBLANK_CYCLES;
                        self.current_line += 1;
                        if self.current_line as usize == SCREEN_HEIGHT {
                            self.mode = PpuMode::VBlank;
                            interrupts.request_interrupt(Interrupt::VBlank);
                            if self.interrupt_enabled[1] {
                                interrupts.request_interrupt(Interrupt::Lcd);
                            }
                        } else {
                            self.mode = PpuMode::InOam;
                        }
                    } else {
                        break;
                    }
                }
                PpuMode::VBlank => {
                    if self.cycles >= SCANLINE_CYCLES {
                        self.cycles -= SCANLINE_CYCLES;
                        self.current_line += 1;
                        if self.current_line as usize == SCREEN_HEIGHT + 10 {
                            self.current_line = 0;
                            self.mode = PpuMode::InOam;
                            if self.interrupt_enabled[3] {
                                interrupts.request_interrupt(Interrupt::Lcd);
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }

    fn render_scanline(&mut self) {
        if self.control.bg_on {
            self.render_bg();
        }
    }

    fn render_bg(&mut self) {
        let y_pos = (self.current_line as usize + self.scroll_y as usize) % 256;
        let bg_base = if self.control.bg_use_upper_map { 0x1C00 } else { 0x1800 };
        let tile_row = y_pos / TILE_WIDTH;

        // Draw each tile in the scanline
        for tile_x in 0..(SCREEN_WIDTH / TILE_WIDTH) {
            let tile_col = (tile_x * TILE_WIDTH + self.scroll_x as usize) % 256 / TILE_WIDTH;

            // 32x32 tiles in background
            let tile_id = self.vram[bg_base + tile_row * 32 + tile_col];

            let tile_address = if self.control.tile_ram_unsigned_mode {
                tile_id as usize * TILE_SIZE
            } else {
                unimplemented!("TODO");
            };

            let tile_byte1 = self.vram[tile_address + (y_pos % TILE_WIDTH) * 2];
            let tile_byte2 = self.vram[tile_address + (y_pos % TILE_WIDTH) * 2 + 1];

            // Draw the 8 pixels in the tile
            for i in 0..8 {
                let inner_offset = 7 - i % 8; // which bit to use as color?
                let color_id = (tile_byte2 >> inner_offset) & 1 |
                               ((tile_byte1 >> inner_offset) & 1) << 1;

                debug_assert!((0..4).contains(&color_id));
                let (r, g, b) = self.palette[color_id as usize];
                // Get actual index of pixel on screen
                let screen_idx = self.current_line as usize * SCREEN_WIDTH + tile_x * TILE_WIDTH + i;

                self.framebuffer[screen_idx * 3 + 0] = r;
                self.framebuffer[screen_idx * 3 + 1] = g;
                self.framebuffer[screen_idx * 3 + 2] = b;
            }
        }
    }

    pub fn read_vram(&self, relative_address: u16) -> u8 {
        debug_assert!((0x0000..0x2000).contains(&relative_address), "addr: {:#04X}", relative_address);
        if !self.control.lcd_on {
            self.vram[relative_address as usize]
        } else {
            match self.mode {
                PpuMode::InVram => 0xFF,
                _ => self.vram[relative_address as usize],
            }
        }
    }

    pub fn write_vram(&mut self, relative_address: u16, value: u8) {
        debug_assert!((0x0000..0x2000).contains(&relative_address), "addr: {:#04X}", relative_address);
        if !self.control.lcd_on {
            self.vram[relative_address as usize] = value
        } else {
            match self.mode {
                PpuMode::InVram => (),
                _ => self.vram[relative_address as usize] = value,
            }
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

    pub fn control_from_byte(&mut self, byte: u8) {
        self.control = PpuControl::from_byte(byte);
    }

    pub fn control_as_byte(&self) -> u8 {
        self.control.as_byte()
    }

    pub fn current_line(&self) -> u8 {
        self.current_line as u8
    }

    pub fn reset_current_line(&mut self) {
        self.current_line = 0
    }

    pub fn framebuffer(&self) -> &[u8] {
        &self.framebuffer
    }

    pub fn palette_from_byte(&mut self, byte: u8) {
        let to_color = |b| match b {
            0 => COLOR0,
            1 => COLOR1,
            2 => COLOR2,
            3 => COLOR3,
            _ => unreachable!(),
        };
        for (i, color) in self.palette.iter_mut().enumerate() {
            *color = to_color((byte >> i * 2) & 0b11);
        }
    }

    pub fn stat_as_byte(&self) -> u8 {
        let mut byte: u8 = match self.mode {
            PpuMode::HBlank => 0,
            PpuMode::VBlank => 1,
            PpuMode::InOam => 2,
            PpuMode::InVram => 3,
        };
        if !self.control.lcd_on { byte = 1 }
        if self.lyc == self.current_line { byte |= 1 << 2 }
        if self.interrupt_enabled[0] { byte |= 1 << 3 }
        if self.interrupt_enabled[1] { byte |= 1 << 4 }
        if self.interrupt_enabled[2] { byte |= 1 << 5 }
        if self.interrupt_enabled[3] { byte |= 1 << 6 }
        byte
    }

    pub fn stat_from_byte(&mut self, byte: u8) {
        self.interrupt_enabled[0] = byte & 1 << 3 != 0;
        self.interrupt_enabled[1] = byte & 1 << 4 != 0;
        self.interrupt_enabled[2] = byte & 1 << 5 != 0;
        self.interrupt_enabled[3] = byte & 1 << 6 != 0;
    }
}
