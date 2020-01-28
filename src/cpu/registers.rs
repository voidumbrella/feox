use crate::cpu::{Cpu, ByteSrc, ByteDest, WordSrc, WordDest};
use crate::bus::Bus;

#[derive(Debug, Copy, Clone)]
pub enum Reg8 {
    A, B, C, D, E, H, L,
}

#[derive(Debug, Copy, Clone)]
pub enum Reg16 {
    AF, BC, DE, HL, PC, SP,
}

impl ByteSrc for Reg8 {
    fn read(&self, cpu: &mut Cpu, _: &mut Bus) -> u8 {
        match self {
            Reg8::A => cpu.regs.a,
            Reg8::B => cpu.regs.b,
            Reg8::C => cpu.regs.c,
            Reg8::D => cpu.regs.d,
            Reg8::E => cpu.regs.e,
            Reg8::H => cpu.regs.h,
            Reg8::L => cpu.regs.l,
        }
    }
}

impl ByteDest for Reg8 {
    fn write(&self, cpu: &mut Cpu, _: &mut Bus, value: u8) {
        match self {
            Reg8::A => cpu.regs.a = value,
            Reg8::B => cpu.regs.b = value,
            Reg8::C => cpu.regs.c = value,
            Reg8::D => cpu.regs.d = value,
            Reg8::E => cpu.regs.e = value,
            Reg8::H => cpu.regs.h = value,
            Reg8::L => cpu.regs.l = value,
        }
    }
}

impl WordSrc for Reg16 {
    fn read(&self, cpu: &mut Cpu, _: &mut Bus) -> u16 {
        cpu.regs.read_pair(*self)
    }
}


impl WordDest for Reg16 {
    fn write(&self, cpu: &mut Cpu, _: &mut Bus, value: u16) {
        cpu.regs.write_pair(*self, value);
    }
}

#[derive(Default)]
pub struct Registers {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

const ZERO_FLAG: u8 = 7;
const SUBTRACT_FLAG: u8 = 6;
const HALF_CARRY_FLAG: u8 = 5;
const CARRY_FLAG: u8 = 4;

impl Registers {
    pub fn set_zero(&mut self, flag: bool) {
        if flag { self.f |= 1 << ZERO_FLAG; }
        else { self.f &= !(1 << ZERO_FLAG); }
    }

    pub fn set_sub(&mut self, flag: bool) {
        if flag { self.f |= 1 << SUBTRACT_FLAG; }
        else { self.f &= !(1 << SUBTRACT_FLAG); }
    }

    pub fn set_half_carry(&mut self, flag: bool) {
        if flag { self.f |= 1 << HALF_CARRY_FLAG; }
        else { self.f &= !(1 << HALF_CARRY_FLAG); }
    }

    pub fn set_carry(&mut self, flag: bool) {
        if flag { self.f |= 1 << CARRY_FLAG; }
        else { self.f &= !(1 << CARRY_FLAG); }
    }

    pub fn zero(&self) -> bool {
        (self.f & 1 << ZERO_FLAG) >> ZERO_FLAG == 1
    }

    pub fn sub(&self) -> bool {
        (self.f & 1 << SUBTRACT_FLAG) >> SUBTRACT_FLAG == 1
    }

    pub fn half_carry(&self) -> bool {
        (self.f & 1 << HALF_CARRY_FLAG) >> HALF_CARRY_FLAG == 1
    }

    pub fn carry(&self) -> bool {
        (self.f & 1 << CARRY_FLAG) >> CARRY_FLAG == 1
    }

    pub fn read_pair(&self, src: Reg16) -> u16 {
        match src {
            Reg16::AF => (self.a as u16) << 8 | self.f as u16,
            Reg16::BC => (self.b as u16) << 8 | self.c as u16,
            Reg16::DE => (self.d as u16) << 8 | self.e as u16,
            Reg16::HL => (self.h as u16) << 8 | self.l as u16,
            Reg16::PC => self.pc,
            Reg16::SP => self.sp,
        }
    }

    pub fn write_pair(&mut self, dest: Reg16, value: u16) {
        let lo = (value >> 8) as u8;
        let hi = (value & 0xFF) as u8;
        match dest {
            Reg16::AF => {
                self.a = lo;
                self.f = hi & 0b11110000;
            }
            Reg16::BC => {
                self.b = lo;
                self.c = hi;
            }
            Reg16::DE => {
                self.d = lo;
                self.e = hi;
            }
            Reg16::HL => {
                self.h = lo;
                self.l = hi;
            }
            Reg16::SP => self.sp = value,
            Reg16::PC => self.pc = value,
        }
    }
}

impl std::fmt::Debug for Registers {
    fn fmt(&self, w: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(w, "Registers {{\
            af: {:#06X}, bc: {:#06X}, de: {:#06X}, hl: {:#06X}, \
            pc: {:#06X}, sp: {:#06X} }}",
               self.read_pair(Reg16::AF),
               self.read_pair(Reg16::BC),
               self.read_pair(Reg16::DE),
               self.read_pair(Reg16::HL),
               self.pc, self.sp)
    }
}
