#[derive(Debug)]
pub enum Interrupt {
    VBlank,
    Lcd,
    Timer,
    Joypad,
}

#[derive(Debug, Default)]
pub struct InterruptQueue {
    vblank: bool,
    lcd: bool,
    timer: bool,
    joypad: bool,

    vblank_enabled: bool,
    lcd_enabled: bool,
    timer_enabled: bool,
    joypad_enabled: bool,
}

impl InterruptQueue {
    pub fn new() -> Self {
        Self {
            vblank: false,
            lcd: false,
            timer: false,
            joypad: false,

            vblank_enabled: false,
            lcd_enabled: false,
            timer_enabled: false,
            joypad_enabled: false,
        }
    }

    pub fn peek(&self) -> bool {
        self.vblank_enabled && self.vblank ||
        self.lcd_enabled && self.lcd ||
        self.timer_enabled && self.timer ||
        self.joypad_enabled && self.joypad 
    }

    pub fn pop(&mut self) -> Option<Interrupt> {
        if self.vblank_enabled && self.vblank {
            self.vblank = false;
            Some(Interrupt::VBlank)
        } else if self.lcd_enabled && self.lcd {
            self.lcd = false;
            Some(Interrupt::Lcd)
        } else if self.timer_enabled && self.timer {
            self.timer = false;
            Some(Interrupt::Timer)
        } else if self.joypad_enabled && self.joypad {
            self.joypad = false;
            Some(Interrupt::Joypad)
        } else {
            None
        }
    }

    pub fn request_interrupt(&mut self, interrupt: Interrupt) {
        match interrupt {
            Interrupt::VBlank => self.vblank = true,
            Interrupt::Lcd => self.lcd = true,
            Interrupt::Timer => self.timer = true,
            Interrupt::Joypad => self.joypad = true,
        }
    }

    pub fn from_byte(&mut self, byte: u8) {
        self.vblank = byte & 1 << 0 != 0;
        self.lcd = byte & 1 << 1 != 0;
        self.timer = byte & 1 << 2 != 0;
        self.joypad = byte & 1 << 4 != 0;
    }

    pub fn flags_from_byte(&mut self, byte: u8) {
        self.vblank_enabled = byte & 1 << 0 != 0;
        self.lcd_enabled = byte & 1 << 1 != 0;
        self.timer_enabled = byte & 1 << 2 != 0;
        self.joypad_enabled = byte & 1 << 4 != 0;
    }

    pub fn as_byte(&self) -> u8 {
        let mut byte: u8 = 0;
        if self.vblank { byte |= 1 << 0 }
        if self.lcd { byte |= 1 << 1 }
        if self.timer { byte |= 1 << 2 }
        if self.joypad { byte |= 1 << 4 }
        byte
    }

    pub fn flags_as_byte(&self) -> u8 {
        let mut byte: u8 = 0;
        if self.vblank_enabled { byte |= 1 << 0 }
        if self.lcd_enabled { byte |= 1 << 1 }
        if self.timer_enabled { byte |= 1 << 2 }
        if self.joypad_enabled { byte |= 1 << 4 }
        byte
    }
}
