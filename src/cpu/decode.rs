use crate::cpu::{Cpu, ByteSrc, ByteDest, WordSrc, WordDest};
use crate::cpu::registers::{Reg8, Reg16};
use crate::emulator::Emulator;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Addr {
    BC,
    DE,
    HL,
    LDH,
    LDHC,
    Immediate,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct ImmediateByte;
impl ByteSrc for ImmediateByte {
    fn read(&self, cpu: &mut Cpu, emulator: &mut Emulator) -> u8 {
        cpu.fetch(emulator)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct ImmediateWord;
impl WordSrc for ImmediateWord {
    fn read(&self, cpu: &mut Cpu, emulator: &mut Emulator) -> u16 {
        cpu.fetch_word(emulator)
    }
}

impl ByteSrc for Addr {
    fn read(&self, cpu: &mut Cpu, emulator: &mut Emulator) -> u8 {
        let address = match self {
            Addr::HL => cpu.regs.read_pair(Reg16::HL),
            Addr::BC => cpu.regs.read_pair(Reg16::BC),
            Addr::DE => cpu.regs.read_pair(Reg16::DE),
            Addr::LDH => 0xFF00 + cpu.fetch(emulator) as u16,
            Addr::LDHC => 0xFF00 + cpu.regs.c as u16,
            Addr::Immediate => cpu.fetch_word(emulator),
        };
        cpu.read_byte(emulator, address)
    }
}

impl ByteDest for Addr {
    fn write(&self, cpu: &mut Cpu, emulator: &mut Emulator, value: u8) {
        let address = match self {
            Addr::HL => cpu.regs.read_pair(Reg16::HL),
            Addr::BC => cpu.regs.read_pair(Reg16::BC),
            Addr::DE => cpu.regs.read_pair(Reg16::DE),
            Addr::LDH => 0xFF00 + cpu.fetch(emulator) as u16,
            Addr::LDHC => 0xFF00 + cpu.regs.c as u16,
            Addr::Immediate => cpu.fetch_word(emulator),
        };
        cpu.write_byte(emulator, address, value)
    }
}

impl WordSrc for Addr {
    fn read(&self, cpu: &mut Cpu, emulator: &mut Emulator) -> u16 {
        let address = match self {
            Addr::HL => cpu.regs.read_pair(Reg16::HL),
            Addr::BC => cpu.regs.read_pair(Reg16::BC),
            Addr::DE => cpu.regs.read_pair(Reg16::DE),
            Addr::LDH => unreachable!(),
            Addr::LDHC => unreachable!(),
            Addr::Immediate => cpu.fetch_word(emulator),
        };
        cpu.read_word(emulator, address)
    }
}

impl WordDest for Addr {
    fn write(&self, cpu: &mut Cpu, emulator: &mut Emulator, value: u16) {
        let address = match self {
            Addr::Immediate => cpu.fetch_word(emulator),
            _ => unreachable!(),
        };
        cpu.write_word(emulator, address, value);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum JumpCond {
    Always,
    Zero,
    NotZero,
    Carry,
    NotCarry
}

impl Cpu {
    pub fn decode(&mut self, emulator: &mut Emulator) {
        // println!("{:#04X}", opcode);
        match self.opcode {
            // nop
            0x00 => (),

            // 8-bit loads
            0x02 => self.load(emulator, Addr::BC, Reg8::A),
            0x0A => self.load(emulator, Reg8::A, Addr::BC),
            0x12 => self.load(emulator, Addr::DE, Reg8::A),
            0x1A => self.load(emulator, Reg8::A, Addr::DE),
            0x22 => {
                self.load(emulator, Addr::HL, Reg8::A);
                let hl = self.regs.read_pair(Reg16::HL);
                self.regs.write_pair(Reg16::HL, hl.wrapping_add(1));
            }
            0x2A => {
                self.load(emulator, Reg8::A, Addr::HL);
                let hl = self.regs.read_pair(Reg16::HL);
                self.regs.write_pair(Reg16::HL, hl.wrapping_add(1));
            }
            0x32 => {
                self.load(emulator, Addr::HL, Reg8::A);
                let hl = self.regs.read_pair(Reg16::HL);
                self.regs.write_pair(Reg16::HL, hl.wrapping_sub(1));
            }
            0x3A => {
                self.load(emulator, Reg8::A, Addr::HL);
                let hl = self.regs.read_pair(Reg16::HL);
                self.regs.write_pair(Reg16::HL, hl.wrapping_sub(1));
            }
            0x06 => self.load(emulator, Reg8::B, ImmediateByte),
            0x0E => self.load(emulator, Reg8::C, ImmediateByte),
            0x16 => self.load(emulator, Reg8::D, ImmediateByte),
            0x1E => self.load(emulator, Reg8::E, ImmediateByte),
            0x26 => self.load(emulator, Reg8::H, ImmediateByte),
            0x2E => self.load(emulator, Reg8::L, ImmediateByte),
            0x36 => self.load(emulator, Addr::HL, ImmediateByte),
            0x3E => self.load(emulator, Reg8::A, ImmediateByte),

            0x40 => self.load(emulator, Reg8::B, Reg8::B),
            0x41 => self.load(emulator, Reg8::B, Reg8::C),
            0x42 => self.load(emulator, Reg8::B, Reg8::D),
            0x43 => self.load(emulator, Reg8::B, Reg8::E),
            0x44 => self.load(emulator, Reg8::B, Reg8::H),
            0x45 => self.load(emulator, Reg8::B, Reg8::L),
            0x46 => self.load(emulator, Reg8::B, Addr::HL),
            0x47 => self.load(emulator, Reg8::B, Reg8::A),

            0x48 => self.load(emulator, Reg8::C, Reg8::B),
            0x49 => self.load(emulator, Reg8::C, Reg8::C),
            0x4A => self.load(emulator, Reg8::C, Reg8::D),
            0x4B => self.load(emulator, Reg8::C, Reg8::E),
            0x4C => self.load(emulator, Reg8::C, Reg8::H),
            0x4D => self.load(emulator, Reg8::C, Reg8::L),
            0x4E => self.load(emulator, Reg8::C, Addr::HL),
            0x4F => self.load(emulator, Reg8::C, Reg8::A),

            0x50 => self.load(emulator, Reg8::D, Reg8::B),
            0x51 => self.load(emulator, Reg8::D, Reg8::C),
            0x52 => self.load(emulator, Reg8::D, Reg8::D),
            0x53 => self.load(emulator, Reg8::D, Reg8::E),
            0x54 => self.load(emulator, Reg8::D, Reg8::H),
            0x55 => self.load(emulator, Reg8::D, Reg8::L),
            0x56 => self.load(emulator, Reg8::D, Addr::HL),
            0x57 => self.load(emulator, Reg8::D, Reg8::A),

            0x58 => self.load(emulator, Reg8::E, Reg8::B),
            0x59 => self.load(emulator, Reg8::E, Reg8::C),
            0x5A => self.load(emulator, Reg8::E, Reg8::D),
            0x5B => self.load(emulator, Reg8::E, Reg8::E),
            0x5C => self.load(emulator, Reg8::E, Reg8::H),
            0x5D => self.load(emulator, Reg8::E, Reg8::L),
            0x5E => self.load(emulator, Reg8::E, Addr::HL),
            0x5F => self.load(emulator, Reg8::E, Reg8::A),

            0x60 => self.load(emulator, Reg8::H, Reg8::B),
            0x61 => self.load(emulator, Reg8::H, Reg8::C),
            0x62 => self.load(emulator, Reg8::H, Reg8::D),
            0x63 => self.load(emulator, Reg8::H, Reg8::E),
            0x64 => self.load(emulator, Reg8::H, Reg8::H),
            0x65 => self.load(emulator, Reg8::H, Reg8::L),
            0x66 => self.load(emulator, Reg8::H, Addr::HL),
            0x67 => self.load(emulator, Reg8::H, Reg8::A),

            0x68 => self.load(emulator, Reg8::L, Reg8::B),
            0x69 => self.load(emulator, Reg8::L, Reg8::C),
            0x6A => self.load(emulator, Reg8::L, Reg8::D),
            0x6B => self.load(emulator, Reg8::L, Reg8::E),
            0x6C => self.load(emulator, Reg8::L, Reg8::H),
            0x6D => self.load(emulator, Reg8::L, Reg8::L),
            0x6E => self.load(emulator, Reg8::L, Addr::HL),
            0x6F => self.load(emulator, Reg8::L, Reg8::A),

            0x70 => self.load(emulator, Addr::HL, Reg8::B),
            0x71 => self.load(emulator, Addr::HL, Reg8::C),
            0x72 => self.load(emulator, Addr::HL, Reg8::D),
            0x73 => self.load(emulator, Addr::HL, Reg8::E),
            0x74 => self.load(emulator, Addr::HL, Reg8::H),
            0x75 => self.load(emulator, Addr::HL, Reg8::L),
            0x77 => self.load(emulator, Addr::HL, Reg8::A),

            0x78 => self.load(emulator, Reg8::A, Reg8::B),
            0x79 => self.load(emulator, Reg8::A, Reg8::C),
            0x7A => self.load(emulator, Reg8::A, Reg8::D),
            0x7B => self.load(emulator, Reg8::A, Reg8::E),
            0x7C => self.load(emulator, Reg8::A, Reg8::H),
            0x7D => self.load(emulator, Reg8::A, Reg8::L),
            0x7E => self.load(emulator, Reg8::A, Addr::HL),
            0x7F => self.load(emulator, Reg8::A, Reg8::A),

            0xE0 => self.load(emulator, Addr::LDH, Reg8::A),
            0xE2 => self.load(emulator, Addr::LDHC, Reg8::A),
            0xF0 => self.load(emulator, Reg8::A, Addr::LDH),
            0xF2 => self.load(emulator, Reg8::A, Addr::LDHC),

            0xEA => self.load(emulator, Addr::Immediate, Reg8::A),
            0xFA => self.load(emulator, Reg8::A, Addr::Immediate),

            // 8-bit adds
            0x80 => self.add(emulator, Reg8::B),
            0x81 => self.add(emulator, Reg8::C),
            0x82 => self.add(emulator, Reg8::D),
            0x83 => self.add(emulator, Reg8::E),
            0x84 => self.add(emulator, Reg8::H),
            0x85 => self.add(emulator, Reg8::L),
            0x86 => self.add(emulator, Addr::HL),
            0x87 => self.add(emulator, Reg8::A),
            0xC6 => self.add(emulator, ImmediateByte),

            // 8-bit adds + carry bit
            0x88 => self.adc(emulator, Reg8::B),
            0x89 => self.adc(emulator, Reg8::C),
            0x8A => self.adc(emulator, Reg8::D),
            0x8B => self.adc(emulator, Reg8::E),
            0x8C => self.adc(emulator, Reg8::H),
            0x8D => self.adc(emulator, Reg8::L),
            0x8E => self.adc(emulator, Addr::HL),
            0x8F => self.adc(emulator, Reg8::A),
            0xCE => self.adc(emulator, ImmediateByte),

            // 8-bit subs
            0x90 => self.sub(emulator, Reg8::B),
            0x91 => self.sub(emulator, Reg8::C),
            0x92 => self.sub(emulator, Reg8::D),
            0x93 => self.sub(emulator, Reg8::E),
            0x94 => self.sub(emulator, Reg8::H),
            0x95 => self.sub(emulator, Reg8::L),
            0x96 => self.sub(emulator, Addr::HL),
            0x97 => self.sub(emulator, Reg8::A),
            0xD6 => self.sub(emulator, ImmediateByte),

            // 8-bit subs + carry bit
            0x98 => self.sbc(emulator, Reg8::B),
            0x99 => self.sbc(emulator, Reg8::C),
            0x9A => self.sbc(emulator, Reg8::D),
            0x9B => self.sbc(emulator, Reg8::E),
            0x9C => self.sbc(emulator, Reg8::H),
            0x9D => self.sbc(emulator, Reg8::L),
            0x9E => self.sbc(emulator, Addr::HL),
            0x9F => self.sbc(emulator, Reg8::A),
            0xDE => self.sbc(emulator, ImmediateByte),

            // 8-bit increment
            0x04 => self.inc(emulator, Reg8::B),
            0x0C => self.inc(emulator, Reg8::C),
            0x14 => self.inc(emulator, Reg8::D),
            0x1C => self.inc(emulator, Reg8::E),
            0x24 => self.inc(emulator, Reg8::H),
            0x2C => self.inc(emulator, Reg8::L),
            0x34 => self.inc(emulator, Addr::HL),
            0x3C => self.inc(emulator, Reg8::A),

            // 8-bit decrement
            0x05 => self.dec(emulator, Reg8::B),
            0x0D => self.dec(emulator, Reg8::C),
            0x15 => self.dec(emulator, Reg8::D),
            0x1D => self.dec(emulator, Reg8::E),
            0x25 => self.dec(emulator, Reg8::H),
            0x2D => self.dec(emulator, Reg8::L),
            0x35 => self.dec(emulator, Addr::HL),
            0x3D => self.dec(emulator, Reg8::A),

            // 16-bit loads
            0x01 => self.load16(emulator, Reg16::BC, ImmediateWord),
            0x11 => self.load16(emulator, Reg16::DE, ImmediateWord),
            0x21 => self.load16(emulator, Reg16::HL, ImmediateWord),
            0x31 => self.load16(emulator, Reg16::SP, ImmediateWord),
            0x08 => self.load16(emulator, Addr::Immediate, Reg16::SP),
            0xF9 => self.load16_sphl(emulator),

            // 16-bit add to hl
            0x09 => self.add16hl(emulator, Reg16::BC),
            0x19 => self.add16hl(emulator, Reg16::DE),
            0x29 => self.add16hl(emulator, Reg16::HL),
            0x39 => self.add16hl(emulator, Reg16::SP),

            // 16-bit stack pointer instructions
            0xE8 => self.add16_sp_n(emulator),
            0xF8 => self.load16_hlsp_n(emulator),

            // 16-bit increments
            0x03 => self.inc16(emulator, Reg16::BC),
            0x13 => self.inc16(emulator, Reg16::DE),
            0x23 => self.inc16(emulator, Reg16::HL),
            0x33 => self.inc16(emulator, Reg16::SP),

            // 16-bit decrements
            0x0B => self.dec16(emulator, Reg16::BC),
            0x1B => self.dec16(emulator, Reg16::DE),
            0x2B => self.dec16(emulator, Reg16::HL),
            0x3B => self.dec16(emulator, Reg16::SP),

            // relative jumps
            0x18 => self.jr(emulator, JumpCond::Always),
            0x20 => self.jr(emulator, JumpCond::NotZero),
            0x28 => self.jr(emulator, JumpCond::Zero),
            0x30 => self.jr(emulator, JumpCond::NotCarry),
            0x38 => self.jr(emulator, JumpCond::Carry),

            // absolute jumps
            0xC2 => self.jp(emulator, JumpCond::NotZero, ImmediateWord),
            0xC3 => self.jp(emulator, JumpCond::Always, ImmediateWord),
            0xCA => self.jp(emulator, JumpCond::Zero, ImmediateWord),
            0xD2 => self.jp(emulator, JumpCond::NotCarry, ImmediateWord),
            0xDA => self.jp(emulator, JumpCond::Carry, ImmediateWord),
            0xE9 => self.jp_hl(emulator),

            // pop from stack
            0xC1 => self.pop(emulator, Reg16::BC),
            0xD1 => self.pop(emulator, Reg16::DE),
            0xE1 => self.pop(emulator, Reg16::HL),
            0xF1 => self.pop(emulator, Reg16::AF),

            // push to stack
            0xC5 => self.push(emulator, Reg16::BC),
            0xD5 => self.push(emulator, Reg16::DE),
            0xE5 => self.push(emulator, Reg16::HL),
            0xF5 => self.push(emulator, Reg16::AF),

            // call
            0xC4 => self.call(emulator, JumpCond::NotZero),
            0xCC => self.call(emulator, JumpCond::Zero),
            0xCD => self.call(emulator, JumpCond::Always),
            0xD4 => self.call(emulator, JumpCond::NotCarry),
            0xDC => self.call(emulator, JumpCond::Carry),

            // return
            0xC0 => self.ret(emulator, JumpCond::NotZero),
            0xC8 => self.ret(emulator, JumpCond::Zero),
            0xC9 => self.ret(emulator, JumpCond::Always),
            0xD0 => self.ret(emulator, JumpCond::NotCarry),
            0xD8 => self.ret(emulator, JumpCond::Carry),
            0xD9 => self.reti(emulator, JumpCond::Always),

            // call reset vetor
            0xC7 => self.rst(emulator, 0x00),
            0xCF => self.rst(emulator, 0x08),
            0xD7 => self.rst(emulator, 0x10),
            0xDF => self.rst(emulator, 0x18),
            0xE7 => self.rst(emulator, 0x20),
            0xEF => self.rst(emulator, 0x28),
            0xF7 => self.rst(emulator, 0x30),
            0xFF => self.rst(emulator, 0x38),

            // bitwise and
            0xA0 => self.and(emulator, Reg8::B),
            0xA1 => self.and(emulator, Reg8::C),
            0xA2 => self.and(emulator, Reg8::D),
            0xA3 => self.and(emulator, Reg8::E),
            0xA4 => self.and(emulator, Reg8::H),
            0xA5 => self.and(emulator, Reg8::L),
            0xA6 => self.and(emulator, Addr::HL),
            0xA7 => self.and(emulator, Reg8::A),
            0xE6 => self.and(emulator, ImmediateByte),

            // bitwise xor
            0xA8 => self.xor(emulator, Reg8::B),
            0xA9 => self.xor(emulator, Reg8::C),
            0xAA => self.xor(emulator, Reg8::D),
            0xAB => self.xor(emulator, Reg8::E),
            0xAC => self.xor(emulator, Reg8::H),
            0xAD => self.xor(emulator, Reg8::L),
            0xAE => self.xor(emulator, Addr::HL),
            0xAF => self.xor(emulator, Reg8::A),
            0xEE => self.xor(emulator, ImmediateByte),

            // bitwise or
            0xB0 => self.or(emulator, Reg8::B),
            0xB1 => self.or(emulator, Reg8::C),
            0xB2 => self.or(emulator, Reg8::D),
            0xB3 => self.or(emulator, Reg8::E),
            0xB4 => self.or(emulator, Reg8::H),
            0xB5 => self.or(emulator, Reg8::L),
            0xB6 => self.or(emulator, Addr::HL),
            0xB7 => self.or(emulator, Reg8::A),
            0xF6 => self.or(emulator, ImmediateByte),

            // rotations
            0x07 => self.rlca(emulator, ),
            0x17 => self.rla(emulator, ),
            0x0F => self.rrca(emulator, ),
            0x1F => self.rra(emulator, ),

            // compare
            0xB8 => self.cp(emulator, Reg8::B),
            0xB9 => self.cp(emulator, Reg8::C),
            0xBA => self.cp(emulator, Reg8::D),
            0xBB => self.cp(emulator, Reg8::E),
            0xBC => self.cp(emulator, Reg8::H),
            0xBD => self.cp(emulator, Reg8::L),
            0xBE => self.cp(emulator, Addr::HL),
            0xBF => self.cp(emulator, Reg8::A),
            0xFE => self.cp(emulator, ImmediateByte),

            // complement A register
            0x2F => self.cpl(emulator),

            // set carry flag
            0x37 => self.scf(emulator),

            // complement carry flag
            0x3F => self.ccf(emulator),

            // misc
            0x10 => self.stop(emulator),
            0x27 => self.daa(emulator),
            0x76 => self.halt(emulator),
            0xF3 => self.di(emulator),
            0xFB => self.ei(emulator),

            // prefixed instructions
            0xCB => {
                self.opcode = self.fetch(emulator);
                self.prefixed_decode(emulator);
            }

            0xD3 | 0xDB | 0xDD | 0xE3 | 0xE4 | 0xEB | 0xEC | 0xED | 0xF4 | 0xFC | 0xFD =>
                panic!("Invalid opcode {:#X}", self.opcode),
        }
    }

    pub fn prefixed_decode(&mut self, emulator: &mut Emulator) {
        match self.opcode {
            0x00 => self.rlc(emulator, Reg8::B),
            0x01 => self.rlc(emulator, Reg8::C),
            0x02 => self.rlc(emulator, Reg8::D),
            0x03 => self.rlc(emulator, Reg8::E),
            0x04 => self.rlc(emulator, Reg8::H),
            0x05 => self.rlc(emulator, Reg8::L),
            0x06 => self.rlc(emulator, Addr::HL),
            0x07 => self.rlc(emulator, Reg8::A),
            0x08 => self.rrc(emulator, Reg8::B),
            0x09 => self.rrc(emulator, Reg8::C),
            0x0A => self.rrc(emulator, Reg8::D),
            0x0B => self.rrc(emulator, Reg8::E),
            0x0C => self.rrc(emulator, Reg8::H),
            0x0D => self.rrc(emulator, Reg8::L),
            0x0E => self.rrc(emulator, Addr::HL),
            0x0F => self.rrc(emulator, Reg8::A),
            0x10 => self.rl(emulator, Reg8::B),
            0x11 => self.rl(emulator, Reg8::C),
            0x12 => self.rl(emulator, Reg8::D),
            0x13 => self.rl(emulator, Reg8::E),
            0x14 => self.rl(emulator, Reg8::H),
            0x15 => self.rl(emulator, Reg8::L),
            0x16 => self.rl(emulator, Addr::HL),
            0x17 => self.rl(emulator, Reg8::A),
            0x18 => self.rr(emulator, Reg8::B),
            0x19 => self.rr(emulator, Reg8::C),
            0x1A => self.rr(emulator, Reg8::D),
            0x1B => self.rr(emulator, Reg8::E),
            0x1C => self.rr(emulator, Reg8::H),
            0x1D => self.rr(emulator, Reg8::L),
            0x1E => self.rr(emulator, Addr::HL),
            0x1F => self.rr(emulator, Reg8::A),
            0x20 => self.sla(emulator, Reg8::B),
            0x21 => self.sla(emulator, Reg8::C),
            0x22 => self.sla(emulator, Reg8::D),
            0x23 => self.sla(emulator, Reg8::E),
            0x24 => self.sla(emulator, Reg8::H),
            0x25 => self.sla(emulator, Reg8::L),
            0x26 => self.sla(emulator, Addr::HL),
            0x27 => self.sla(emulator, Reg8::A),
            0x28 => self.sra(emulator, Reg8::B),
            0x29 => self.sra(emulator, Reg8::C),
            0x2A => self.sra(emulator, Reg8::D),
            0x2B => self.sra(emulator, Reg8::E),
            0x2C => self.sra(emulator, Reg8::H),
            0x2D => self.sra(emulator, Reg8::L),
            0x2E => self.sra(emulator, Addr::HL),
            0x2F => self.sra(emulator, Reg8::A),
            0x30 => self.swap(emulator, Reg8::B),
            0x31 => self.swap(emulator, Reg8::C),
            0x32 => self.swap(emulator, Reg8::D),
            0x33 => self.swap(emulator, Reg8::E),
            0x34 => self.swap(emulator, Reg8::H),
            0x35 => self.swap(emulator, Reg8::L),
            0x36 => self.swap(emulator, Addr::HL),
            0x37 => self.swap(emulator, Reg8::A),
            0x38 => self.srl(emulator, Reg8::B),
            0x39 => self.srl(emulator, Reg8::C),
            0x3A => self.srl(emulator, Reg8::D),
            0x3B => self.srl(emulator, Reg8::E),
            0x3C => self.srl(emulator, Reg8::H),
            0x3D => self.srl(emulator, Reg8::L),
            0x3E => self.srl(emulator, Addr::HL),
            0x3F => self.srl(emulator, Reg8::A),
            0x40 => self.bit(emulator, 0, Reg8::B),
            0x41 => self.bit(emulator, 0, Reg8::C),
            0x42 => self.bit(emulator, 0, Reg8::D),
            0x43 => self.bit(emulator, 0, Reg8::E),
            0x44 => self.bit(emulator, 0, Reg8::H),
            0x45 => self.bit(emulator, 0, Reg8::L),
            0x46 => self.bit(emulator, 0, Addr::HL),
            0x47 => self.bit(emulator, 0, Reg8::A),
            0x48 => self.bit(emulator, 1, Reg8::B),
            0x49 => self.bit(emulator, 1, Reg8::C),
            0x4A => self.bit(emulator, 1, Reg8::D),
            0x4B => self.bit(emulator, 1, Reg8::E),
            0x4C => self.bit(emulator, 1, Reg8::H),
            0x4D => self.bit(emulator, 1, Reg8::L),
            0x4E => self.bit(emulator, 1, Addr::HL),
            0x4F => self.bit(emulator, 1, Reg8::A),
            0x50 => self.bit(emulator, 2, Reg8::B),
            0x51 => self.bit(emulator, 2, Reg8::C),
            0x52 => self.bit(emulator, 2, Reg8::D),
            0x53 => self.bit(emulator, 2, Reg8::E),
            0x54 => self.bit(emulator, 2, Reg8::H),
            0x55 => self.bit(emulator, 2, Reg8::L),
            0x56 => self.bit(emulator, 2, Addr::HL),
            0x57 => self.bit(emulator, 2, Reg8::A),
            0x58 => self.bit(emulator, 3, Reg8::B),
            0x59 => self.bit(emulator, 3, Reg8::C),
            0x5A => self.bit(emulator, 3, Reg8::D),
            0x5B => self.bit(emulator, 3, Reg8::E),
            0x5C => self.bit(emulator, 3, Reg8::H),
            0x5D => self.bit(emulator, 3, Reg8::L),
            0x5E => self.bit(emulator, 3, Addr::HL),
            0x5F => self.bit(emulator, 3, Reg8::A),
            0x60 => self.bit(emulator, 4, Reg8::B),
            0x61 => self.bit(emulator, 4, Reg8::C),
            0x62 => self.bit(emulator, 4, Reg8::D),
            0x63 => self.bit(emulator, 4, Reg8::E),
            0x64 => self.bit(emulator, 4, Reg8::H),
            0x65 => self.bit(emulator, 4, Reg8::L),
            0x66 => self.bit(emulator, 4, Addr::HL),
            0x67 => self.bit(emulator, 4, Reg8::A),
            0x68 => self.bit(emulator, 5, Reg8::B),
            0x69 => self.bit(emulator, 5, Reg8::C),
            0x6A => self.bit(emulator, 5, Reg8::D),
            0x6B => self.bit(emulator, 5, Reg8::E),
            0x6C => self.bit(emulator, 5, Reg8::H),
            0x6D => self.bit(emulator, 5, Reg8::L),
            0x6E => self.bit(emulator, 5, Addr::HL),
            0x6F => self.bit(emulator, 5, Reg8::A),
            0x70 => self.bit(emulator, 6, Reg8::B),
            0x71 => self.bit(emulator, 6, Reg8::C),
            0x72 => self.bit(emulator, 6, Reg8::D),
            0x73 => self.bit(emulator, 6, Reg8::E),
            0x74 => self.bit(emulator, 6, Reg8::H),
            0x75 => self.bit(emulator, 6, Reg8::L),
            0x76 => self.bit(emulator, 6, Addr::HL),
            0x77 => self.bit(emulator, 6, Reg8::A),
            0x78 => self.bit(emulator, 7, Reg8::B),
            0x79 => self.bit(emulator, 7, Reg8::C),
            0x7A => self.bit(emulator, 7, Reg8::D),
            0x7B => self.bit(emulator, 7, Reg8::E),
            0x7C => self.bit(emulator, 7, Reg8::H),
            0x7D => self.bit(emulator, 7, Reg8::L),
            0x7E => self.bit(emulator, 7, Addr::HL),
            0x7F => self.bit(emulator, 7, Reg8::A),
            0x80 => self.res(emulator, 0, Reg8::B),
            0x81 => self.res(emulator, 0, Reg8::C),
            0x82 => self.res(emulator, 0, Reg8::D),
            0x83 => self.res(emulator, 0, Reg8::E),
            0x84 => self.res(emulator, 0, Reg8::H),
            0x85 => self.res(emulator, 0, Reg8::L),
            0x86 => self.res(emulator, 0, Addr::HL),
            0x87 => self.res(emulator, 0, Reg8::A),
            0x88 => self.res(emulator, 1, Reg8::B),
            0x89 => self.res(emulator, 1, Reg8::C),
            0x8A => self.res(emulator, 1, Reg8::D),
            0x8B => self.res(emulator, 1, Reg8::E),
            0x8C => self.res(emulator, 1, Reg8::H),
            0x8D => self.res(emulator, 1, Reg8::L),
            0x8E => self.res(emulator, 1, Addr::HL),
            0x8F => self.res(emulator, 1, Reg8::A),
            0x90 => self.res(emulator, 2, Reg8::B),
            0x91 => self.res(emulator, 2, Reg8::C),
            0x92 => self.res(emulator, 2, Reg8::D),
            0x93 => self.res(emulator, 2, Reg8::E),
            0x94 => self.res(emulator, 2, Reg8::H),
            0x95 => self.res(emulator, 2, Reg8::L),
            0x96 => self.res(emulator, 2, Addr::HL),
            0x97 => self.res(emulator, 2, Reg8::A),
            0x98 => self.res(emulator, 3, Reg8::B),
            0x99 => self.res(emulator, 3, Reg8::C),
            0x9A => self.res(emulator, 3, Reg8::D),
            0x9B => self.res(emulator, 3, Reg8::E),
            0x9C => self.res(emulator, 3, Reg8::H),
            0x9D => self.res(emulator, 3, Reg8::L),
            0x9E => self.res(emulator, 3, Addr::HL),
            0x9F => self.res(emulator, 3, Reg8::A),
            0xA0 => self.res(emulator, 4, Reg8::B),
            0xA1 => self.res(emulator, 4, Reg8::C),
            0xA2 => self.res(emulator, 4, Reg8::D),
            0xA3 => self.res(emulator, 4, Reg8::E),
            0xA4 => self.res(emulator, 4, Reg8::H),
            0xA5 => self.res(emulator, 4, Reg8::L),
            0xA6 => self.res(emulator, 4, Addr::HL),
            0xA7 => self.res(emulator, 4, Reg8::A),
            0xA8 => self.res(emulator, 5, Reg8::B),
            0xA9 => self.res(emulator, 5, Reg8::C),
            0xAA => self.res(emulator, 5, Reg8::D),
            0xAB => self.res(emulator, 5, Reg8::E),
            0xAC => self.res(emulator, 5, Reg8::H),
            0xAD => self.res(emulator, 5, Reg8::L),
            0xAE => self.res(emulator, 5, Addr::HL),
            0xAF => self.res(emulator, 5, Reg8::A),
            0xB0 => self.res(emulator, 6, Reg8::B),
            0xB1 => self.res(emulator, 6, Reg8::C),
            0xB2 => self.res(emulator, 6, Reg8::D),
            0xB3 => self.res(emulator, 6, Reg8::E),
            0xB4 => self.res(emulator, 6, Reg8::H),
            0xB5 => self.res(emulator, 6, Reg8::L),
            0xB6 => self.res(emulator, 6, Addr::HL),
            0xB7 => self.res(emulator, 6, Reg8::A),
            0xB8 => self.res(emulator, 7, Reg8::B),
            0xB9 => self.res(emulator, 7, Reg8::C),
            0xBA => self.res(emulator, 7, Reg8::D),
            0xBB => self.res(emulator, 7, Reg8::E),
            0xBC => self.res(emulator, 7, Reg8::H),
            0xBD => self.res(emulator, 7, Reg8::L),
            0xBE => self.res(emulator, 7, Addr::HL),
            0xBF => self.res(emulator, 7, Reg8::A),
            0xC0 => self.set(emulator, 0, Reg8::B),
            0xC1 => self.set(emulator, 0, Reg8::C),
            0xC2 => self.set(emulator, 0, Reg8::D),
            0xC3 => self.set(emulator, 0, Reg8::E),
            0xC4 => self.set(emulator, 0, Reg8::H),
            0xC5 => self.set(emulator, 0, Reg8::L),
            0xC6 => self.set(emulator, 0, Addr::HL),
            0xC7 => self.set(emulator, 0, Reg8::A),
            0xC8 => self.set(emulator, 1, Reg8::B),
            0xC9 => self.set(emulator, 1, Reg8::C),
            0xCA => self.set(emulator, 1, Reg8::D),
            0xCB => self.set(emulator, 1, Reg8::E),
            0xCC => self.set(emulator, 1, Reg8::H),
            0xCD => self.set(emulator, 1, Reg8::L),
            0xCE => self.set(emulator, 1, Addr::HL),
            0xCF => self.set(emulator, 1, Reg8::A),
            0xD0 => self.set(emulator, 2, Reg8::B),
            0xD1 => self.set(emulator, 2, Reg8::C),
            0xD2 => self.set(emulator, 2, Reg8::D),
            0xD3 => self.set(emulator, 2, Reg8::E),
            0xD4 => self.set(emulator, 2, Reg8::H),
            0xD5 => self.set(emulator, 2, Reg8::L),
            0xD6 => self.set(emulator, 2, Addr::HL),
            0xD7 => self.set(emulator, 2, Reg8::A),
            0xD8 => self.set(emulator, 3, Reg8::B),
            0xD9 => self.set(emulator, 3, Reg8::C),
            0xDA => self.set(emulator, 3, Reg8::D),
            0xDB => self.set(emulator, 3, Reg8::E),
            0xDC => self.set(emulator, 3, Reg8::H),
            0xDD => self.set(emulator, 3, Reg8::L),
            0xDE => self.set(emulator, 3, Addr::HL),
            0xDF => self.set(emulator, 3, Reg8::A),
            0xE0 => self.set(emulator, 4, Reg8::B),
            0xE1 => self.set(emulator, 4, Reg8::C),
            0xE2 => self.set(emulator, 4, Reg8::D),
            0xE3 => self.set(emulator, 4, Reg8::E),
            0xE4 => self.set(emulator, 4, Reg8::H),
            0xE5 => self.set(emulator, 4, Reg8::L),
            0xE6 => self.set(emulator, 4, Addr::HL),
            0xE7 => self.set(emulator, 4, Reg8::A),
            0xE8 => self.set(emulator, 5, Reg8::B),
            0xE9 => self.set(emulator, 5, Reg8::C),
            0xEA => self.set(emulator, 5, Reg8::D),
            0xEB => self.set(emulator, 5, Reg8::E),
            0xEC => self.set(emulator, 5, Reg8::H),
            0xED => self.set(emulator, 5, Reg8::L),
            0xEE => self.set(emulator, 5, Addr::HL),
            0xEF => self.set(emulator, 5, Reg8::A),
            0xF0 => self.set(emulator, 6, Reg8::B),
            0xF1 => self.set(emulator, 6, Reg8::C),
            0xF2 => self.set(emulator, 6, Reg8::D),
            0xF3 => self.set(emulator, 6, Reg8::E),
            0xF4 => self.set(emulator, 6, Reg8::H),
            0xF5 => self.set(emulator, 6, Reg8::L),
            0xF6 => self.set(emulator, 6, Addr::HL),
            0xF7 => self.set(emulator, 6, Reg8::A),
            0xF8 => self.set(emulator, 7, Reg8::B),
            0xF9 => self.set(emulator, 7, Reg8::C),
            0xFA => self.set(emulator, 7, Reg8::D),
            0xFB => self.set(emulator, 7, Reg8::E),
            0xFC => self.set(emulator, 7, Reg8::H),
            0xFD => self.set(emulator, 7, Reg8::L),
            0xFE => self.set(emulator, 7, Addr::HL),
            0xFF => self.set(emulator, 7, Reg8::A),
        }
    }
}
