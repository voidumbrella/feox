use crate::interrupts::{Interrupt, InterruptQueue};

pub enum Button {
    Right,
    Left,
    Up,
    Down,
    A,
    B,
    Select,
    Start,
}

#[derive(Default)]
pub struct Joypad {
    dpad: bool,
    right: bool,
    left: bool,
    up: bool,
    down: bool,

    buttons: bool,
    a: bool,
    b: bool,
    select: bool,
    start: bool,
}

impl Joypad {
    pub fn new() -> Self {
        Joypad::default()
    }

    pub fn press_button(&mut self, interrupts: &mut InterruptQueue, pressed: Button) {
        if match pressed {
            Button::Right => self.right,
            Button::Left => self.left,
            Button::Up => self.up,
            Button::Down => self.down,
            Button::A => self.a,
            Button::B => self.b,
            Button::Select => self.select,
            Button::Start => self.start,
        } == false {
            interrupts.request_interrupt(Interrupt::Joypad);
        }
        match pressed {
            Button::Right => self.right = true,
            Button::Left => self.left = true,
            Button::Up => self.up = true,
            Button::Down => self.down = true,
            Button::A => self.a = true,
            Button::B => self.b = true,
            Button::Select => self.select = true,
            Button::Start => self.start = true,
        };
    }

    pub fn clear_button(&mut self, pressed: Button) {
        match pressed {
            Button::Right => self.right = false,
            Button::Left => self.left = false,
            Button::Up => self.up = false,
            Button::Down => self.down = false,
            Button::A => self.a = false,
            Button::B => self.b = false,
            Button::Select => self.select = false,
            Button::Start => self.start = false,
        };
    }

    pub fn from_byte(&mut self, byte: u8) {
        // Note: `== 0` is intentional
        self.dpad = byte & 1 << 4 == 0;
        self.buttons = byte & 1 << 5 == 0;
    }

    pub fn as_byte(&self) -> u8 {
        let mut byte: u8 = 0xFF;
        if self.dpad && self.right { byte &= !(1 << 0) }
        if self.dpad && self.left { byte &= !(1 << 1) }
        if self.dpad && self.up { byte &= !(1 << 2) }
        if self.dpad && self.down { byte &= !(1 << 3) }

        if self.buttons && self.a { byte &= !(1 << 0) }
        if self.buttons && self.b { byte &= !(1 << 1) }
        if self.buttons && self.select { byte &= !(1 << 2) }
        if self.buttons && self.start { byte &= !(1 << 3) }
        byte
    }
}
