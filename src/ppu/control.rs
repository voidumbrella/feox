#[derive(Default)]
pub struct PpuControl {
    pub bg_on: bool,
    pub obj_on: bool,
    pub obj_long_blocks: bool,
    pub bg_use_upper_map: bool,
    pub tile_ram_unsigned_mode: bool,
    pub window_on: bool,
    pub window_use_upper_map: bool,
    pub lcd_on: bool,
}

impl PpuControl {
    pub fn from_byte(byte: u8) -> Self {
        Self {
            bg_on: byte & (1 << 0) != 0,
            obj_on: byte & (1 << 1) != 0,
            obj_long_blocks: byte & (1 << 2) != 0,
            bg_use_upper_map: byte & (1 << 3) != 0,
            tile_ram_unsigned_mode: byte & (1 << 4) != 0,
            window_on: byte & (1 << 5) != 0,
            window_use_upper_map: byte & (1 << 6) != 0,
            lcd_on: byte & (1 << 7) != 0,
        }
    }

    pub fn as_byte(&self) -> u8 {
        let mut byte: u8 = 0;
        if self.bg_on { byte |= 1 << 0 }
        if self.obj_on { byte |= 1 << 1 }
        if self.obj_long_blocks { byte |= 1 << 2 }
        if self.bg_use_upper_map { byte |= 1 << 3 }
        if self.tile_ram_unsigned_mode { byte |= 1 << 4 }
        if self.window_on { byte |= 1 << 5 }
        if self.window_use_upper_map { byte |= 1 << 6 }
        if self.lcd_on { byte |= 1 << 7 }
        byte
    }
}
