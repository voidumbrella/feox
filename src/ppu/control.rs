#[derive(Default)]
pub struct PpuControl {
    pub bg_on: bool, // bit 0
    pub obj_on: bool, // bit 1
    pub block_comp: bool, // bit 2; false => 8x8 dots, true => 8x16 dots
    pub bg_code_area: bool, // bit 3; false => 0x9800-0xBFF, true => 0xC000-0x9FFF 
    pub bg_data_selection: bool, // bit 4; false => 0x8800-0x97ff, true => 0x8000-0x8FFF
    pub window_on: bool, // bit 5
    pub window_code_area: bool, // bit 6; false => 0x9800-9Bff, true => 0x9C00-0x9FFF
    pub lcd_on: bool, // bit 7
}

impl PpuControl {
    pub fn from_byte(byte: u8) -> Self {
        Self {
            bg_on: byte & (1 << 0) != 1,
            obj_on: byte & (1 << 1) != 1,
            block_comp: byte & (1 << 2) != 1,
            bg_code_area: byte & (1 << 3) != 1,
            bg_data_selection: byte & (1 << 4) != 1,
            window_on: byte & (1 << 5) != 1,
            window_code_area: byte & (1 << 6) != 1,
            lcd_on: byte & (1 << 7) != 1,
        }
    }

    pub fn as_byte(&self) -> u8 {
        let mut byte: u8 = 0;
        if self.bg_on { byte |= 1 << 0 }
        if self.obj_on { byte |= 1 << 1 }
        if self.block_comp { byte |= 1 << 2 }
        if self.bg_code_area { byte |= 1 << 3 }
        if self.bg_data_selection { byte |= 1 << 4 }
        if self.window_on { byte |= 1 << 5 }
        if self.window_code_area { byte |= 1 << 6 }
        if self.lcd_on { byte |= 1 << 7 }
        byte
    }
}
