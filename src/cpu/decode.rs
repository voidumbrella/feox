use crate::cpu::{Cpu, ByteSrc, ByteDest, WordSrc, WordDest};
use crate::cpu::registers::{Reg8, Reg16};
use crate::bus::Bus;

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
    fn read(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.fetch(bus)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct ImmediateWord;
impl WordSrc for ImmediateWord {
    fn read(&self, cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        cpu.fetch_word(bus)
    }
}

impl ByteSrc for Addr {
    fn read(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        let address = match self {
            Addr::HL => cpu.regs.read_pair(Reg16::HL),
            Addr::BC => cpu.regs.read_pair(Reg16::BC),
            Addr::DE => cpu.regs.read_pair(Reg16::DE),
            Addr::LDH => 0xFF00 + cpu.fetch(bus) as u16,
            Addr::LDHC => 0xFF00 + cpu.regs.c as u16,
            Addr::Immediate => cpu.fetch_word(bus),
        };
        cpu.read_byte(bus, address)
    }
}

impl ByteDest for Addr {
    fn write(&self, cpu: &mut Cpu, bus: &mut Bus, value: u8) {
        let address = match self {
            Addr::HL => cpu.regs.read_pair(Reg16::HL),
            Addr::BC => cpu.regs.read_pair(Reg16::BC),
            Addr::DE => cpu.regs.read_pair(Reg16::DE),
            Addr::LDH => 0xFF00 + cpu.fetch(bus) as u16,
            Addr::LDHC => 0xFF00 + cpu.regs.c as u16,
            Addr::Immediate => cpu.fetch_word(bus),
        };
        cpu.write_byte(bus, address, value)
    }
}

impl WordSrc for Addr {
    fn read(&self, cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        let address = match self {
            Addr::HL => cpu.regs.read_pair(Reg16::HL),
            Addr::BC => cpu.regs.read_pair(Reg16::BC),
            Addr::DE => cpu.regs.read_pair(Reg16::DE),
            Addr::LDH => unreachable!(),
            Addr::LDHC => unreachable!(),
            Addr::Immediate => cpu.fetch_word(bus),
        };
        cpu.read_word(bus, address)
    }
}

impl WordDest for Addr {
    fn write(&self, cpu: &mut Cpu, bus: &mut Bus, value: u16) {
        let address = match self {
            Addr::Immediate => cpu.fetch_word(bus),
            _ => unreachable!(),
        };
        cpu.write_word(bus, address, value);
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
    pub fn decode(&mut self, bus: &mut Bus) {
        // println!("{:#04X}", opcode);
        match self.opcode {
            // nop
            0x00 => (),

            // 8-bit loads
            0x02 => self.load(bus, Addr::BC, Reg8::A),
            0x0A => self.load(bus, Reg8::A, Addr::BC),
            0x12 => self.load(bus, Addr::DE, Reg8::A),
            0x1A => self.load(bus, Reg8::A, Addr::DE),
            0x22 => {
                self.load(bus, Addr::HL, Reg8::A);
                let hl = self.regs.read_pair(Reg16::HL);
                self.regs.write_pair(Reg16::HL, hl.wrapping_add(1));
            }
            0x2A => {
                self.load(bus, Reg8::A, Addr::HL);
                let hl = self.regs.read_pair(Reg16::HL);
                self.regs.write_pair(Reg16::HL, hl.wrapping_add(1));
            }
            0x32 => {
                self.load(bus, Addr::HL, Reg8::A);
                let hl = self.regs.read_pair(Reg16::HL);
                self.regs.write_pair(Reg16::HL, hl.wrapping_sub(1));
            }
            0x3A => {
                self.load(bus, Reg8::A, Addr::HL);
                let hl = self.regs.read_pair(Reg16::HL);
                self.regs.write_pair(Reg16::HL, hl.wrapping_sub(1));
            }
            0x06 => self.load(bus, Reg8::B, ImmediateByte),
            0x0E => self.load(bus, Reg8::C, ImmediateByte),
            0x16 => self.load(bus, Reg8::D, ImmediateByte),
            0x1E => self.load(bus, Reg8::E, ImmediateByte),
            0x26 => self.load(bus, Reg8::H, ImmediateByte),
            0x2E => self.load(bus, Reg8::L, ImmediateByte),
            0x36 => self.load(bus, Addr::HL, ImmediateByte),
            0x3E => self.load(bus, Reg8::A, ImmediateByte),

            0x40 => self.load(bus, Reg8::B, Reg8::B),
            0x41 => self.load(bus, Reg8::B, Reg8::C),
            0x42 => self.load(bus, Reg8::B, Reg8::D),
            0x43 => self.load(bus, Reg8::B, Reg8::E),
            0x44 => self.load(bus, Reg8::B, Reg8::H),
            0x45 => self.load(bus, Reg8::B, Reg8::L),
            0x46 => self.load(bus, Reg8::B, Addr::HL),
            0x47 => self.load(bus, Reg8::B, Reg8::A),

            0x48 => self.load(bus, Reg8::C, Reg8::B),
            0x49 => self.load(bus, Reg8::C, Reg8::C),
            0x4A => self.load(bus, Reg8::C, Reg8::D),
            0x4B => self.load(bus, Reg8::C, Reg8::E),
            0x4C => self.load(bus, Reg8::C, Reg8::H),
            0x4D => self.load(bus, Reg8::C, Reg8::L),
            0x4E => self.load(bus, Reg8::C, Addr::HL),
            0x4F => self.load(bus, Reg8::C, Reg8::A),

            0x50 => self.load(bus, Reg8::D, Reg8::B),
            0x51 => self.load(bus, Reg8::D, Reg8::C),
            0x52 => self.load(bus, Reg8::D, Reg8::D),
            0x53 => self.load(bus, Reg8::D, Reg8::E),
            0x54 => self.load(bus, Reg8::D, Reg8::H),
            0x55 => self.load(bus, Reg8::D, Reg8::L),
            0x56 => self.load(bus, Reg8::D, Addr::HL),
            0x57 => self.load(bus, Reg8::D, Reg8::A),

            0x58 => self.load(bus, Reg8::E, Reg8::B),
            0x59 => self.load(bus, Reg8::E, Reg8::C),
            0x5A => self.load(bus, Reg8::E, Reg8::D),
            0x5B => self.load(bus, Reg8::E, Reg8::E),
            0x5C => self.load(bus, Reg8::E, Reg8::H),
            0x5D => self.load(bus, Reg8::E, Reg8::L),
            0x5E => self.load(bus, Reg8::E, Addr::HL),
            0x5F => self.load(bus, Reg8::E, Reg8::A),

            0x60 => self.load(bus, Reg8::H, Reg8::B),
            0x61 => self.load(bus, Reg8::H, Reg8::C),
            0x62 => self.load(bus, Reg8::H, Reg8::D),
            0x63 => self.load(bus, Reg8::H, Reg8::E),
            0x64 => self.load(bus, Reg8::H, Reg8::H),
            0x65 => self.load(bus, Reg8::H, Reg8::L),
            0x66 => self.load(bus, Reg8::H, Addr::HL),
            0x67 => self.load(bus, Reg8::H, Reg8::A),

            0x68 => self.load(bus, Reg8::L, Reg8::B),
            0x69 => self.load(bus, Reg8::L, Reg8::C),
            0x6A => self.load(bus, Reg8::L, Reg8::D),
            0x6B => self.load(bus, Reg8::L, Reg8::E),
            0x6C => self.load(bus, Reg8::L, Reg8::H),
            0x6D => self.load(bus, Reg8::L, Reg8::L),
            0x6E => self.load(bus, Reg8::L, Addr::HL),
            0x6F => self.load(bus, Reg8::L, Reg8::A),

            0x70 => self.load(bus, Addr::HL, Reg8::B),
            0x71 => self.load(bus, Addr::HL, Reg8::C),
            0x72 => self.load(bus, Addr::HL, Reg8::D),
            0x73 => self.load(bus, Addr::HL, Reg8::E),
            0x74 => self.load(bus, Addr::HL, Reg8::H),
            0x75 => self.load(bus, Addr::HL, Reg8::L),
            0x77 => self.load(bus, Addr::HL, Reg8::A),

            0x78 => self.load(bus, Reg8::A, Reg8::B),
            0x79 => self.load(bus, Reg8::A, Reg8::C),
            0x7A => self.load(bus, Reg8::A, Reg8::D),
            0x7B => self.load(bus, Reg8::A, Reg8::E),
            0x7C => self.load(bus, Reg8::A, Reg8::H),
            0x7D => self.load(bus, Reg8::A, Reg8::L),
            0x7E => self.load(bus, Reg8::A, Addr::HL),
            0x7F => self.load(bus, Reg8::A, Reg8::A),

            0xE0 => self.load(bus, Addr::LDH, Reg8::A),
            0xE2 => self.load(bus, Addr::LDHC, Reg8::A),
            0xF0 => self.load(bus, Reg8::A, Addr::LDH),
            0xF2 => self.load(bus, Reg8::A, Addr::LDHC),

            0xEA => self.load(bus, Addr::Immediate, Reg8::A),
            0xFA => self.load(bus, Reg8::A, Addr::Immediate),

            // 8-bit adds
            0x80 => self.add(bus, Reg8::B),
            0x81 => self.add(bus, Reg8::C),
            0x82 => self.add(bus, Reg8::D),
            0x83 => self.add(bus, Reg8::E),
            0x84 => self.add(bus, Reg8::H),
            0x85 => self.add(bus, Reg8::L),
            0x86 => self.add(bus, Addr::HL),
            0x87 => self.add(bus, Reg8::A),
            0xC6 => self.add(bus, ImmediateByte),

            // 8-bit adds + carry bit
            0x88 => self.adc(bus, Reg8::B),
            0x89 => self.adc(bus, Reg8::C),
            0x8A => self.adc(bus, Reg8::D),
            0x8B => self.adc(bus, Reg8::E),
            0x8C => self.adc(bus, Reg8::H),
            0x8D => self.adc(bus, Reg8::L),
            0x8E => self.adc(bus, Addr::HL),
            0x8F => self.adc(bus, Reg8::A),
            0xCE => self.adc(bus, ImmediateByte),

            // 8-bit subs
            0x90 => self.sub(bus, Reg8::B),
            0x91 => self.sub(bus, Reg8::C),
            0x92 => self.sub(bus, Reg8::D),
            0x93 => self.sub(bus, Reg8::E),
            0x94 => self.sub(bus, Reg8::H),
            0x95 => self.sub(bus, Reg8::L),
            0x96 => self.sub(bus, Addr::HL),
            0x97 => self.sub(bus, Reg8::A),
            0xD6 => self.sub(bus, ImmediateByte),

            // 8-bit subs + carry bit
            0x98 => self.sbc(bus, Reg8::B),
            0x99 => self.sbc(bus, Reg8::C),
            0x9A => self.sbc(bus, Reg8::D),
            0x9B => self.sbc(bus, Reg8::E),
            0x9C => self.sbc(bus, Reg8::H),
            0x9D => self.sbc(bus, Reg8::L),
            0x9E => self.sbc(bus, Addr::HL),
            0x9F => self.sbc(bus, Reg8::A),
            0xDE => self.sbc(bus, ImmediateByte),

            // 8-bit increment
            0x04 => self.inc(bus, Reg8::B),
            0x0C => self.inc(bus, Reg8::C),
            0x14 => self.inc(bus, Reg8::D),
            0x1C => self.inc(bus, Reg8::E),
            0x24 => self.inc(bus, Reg8::H),
            0x2C => self.inc(bus, Reg8::L),
            0x34 => self.inc(bus, Addr::HL),
            0x3C => self.inc(bus, Reg8::A),

            // 8-bit decrement
            0x05 => self.dec(bus, Reg8::B),
            0x0D => self.dec(bus, Reg8::C),
            0x15 => self.dec(bus, Reg8::D),
            0x1D => self.dec(bus, Reg8::E),
            0x25 => self.dec(bus, Reg8::H),
            0x2D => self.dec(bus, Reg8::L),
            0x35 => self.dec(bus, Addr::HL),
            0x3D => self.dec(bus, Reg8::A),
            
            // 16-bit loads
            0x01 => self.load16(bus, Reg16::BC, ImmediateWord),
            0x11 => self.load16(bus, Reg16::DE, ImmediateWord),
            0x21 => self.load16(bus, Reg16::HL, ImmediateWord),
            0x31 => self.load16(bus, Reg16::SP, ImmediateWord),
            0x08 => self.load16(bus, Addr::Immediate, Reg16::SP),
            0xF9 => self.load16_sphl(bus),

            // 16-bit add to hl
            0x09 => self.add16hl(bus, Reg16::BC),
            0x19 => self.add16hl(bus, Reg16::DE),
            0x29 => self.add16hl(bus, Reg16::HL),
            0x39 => self.add16hl(bus, Reg16::SP),

            // 16-bit stack pointer instructions
            0xE8 => self.add16_sp_n(bus),
            0xF8 => self.load16_hlsp_n(bus),

            // 16-bit increments
            0x03 => self.inc16(bus, Reg16::BC),
            0x13 => self.inc16(bus, Reg16::DE),
            0x23 => self.inc16(bus, Reg16::HL),
            0x33 => self.inc16(bus, Reg16::SP),

            // 16-bit decrements
            0x0B => self.dec16(bus, Reg16::BC),
            0x1B => self.dec16(bus, Reg16::DE),
            0x2B => self.dec16(bus, Reg16::HL),
            0x3B => self.dec16(bus, Reg16::SP),

            // relative jumps
            0x18 => self.jr(bus, JumpCond::Always),
            0x20 => self.jr(bus, JumpCond::NotZero),
            0x28 => self.jr(bus, JumpCond::Zero),
            0x30 => self.jr(bus, JumpCond::NotCarry),
            0x38 => self.jr(bus, JumpCond::Carry),

            // absolute jumps
            0xC2 => self.jp(bus, JumpCond::NotZero, ImmediateWord),
            0xC3 => self.jp(bus, JumpCond::Always, ImmediateWord),
            0xCA => self.jp(bus, JumpCond::Zero, ImmediateWord),
            0xD2 => self.jp(bus, JumpCond::NotCarry, ImmediateWord),
            0xDA => self.jp(bus, JumpCond::Carry, ImmediateWord),
            0xE9 => self.jp_hl(bus),

            // pop from stack
            0xC1 => self.pop(bus, Reg16::BC),
            0xD1 => self.pop(bus, Reg16::DE),
            0xE1 => self.pop(bus, Reg16::HL),
            0xF1 => self.pop(bus, Reg16::AF),

            // push to stack
            0xC5 => self.push(bus, Reg16::BC),
            0xD5 => self.push(bus, Reg16::DE),
            0xE5 => self.push(bus, Reg16::HL),
            0xF5 => self.push(bus, Reg16::AF),

            // call
            0xC4 => self.call(bus, JumpCond::NotZero),
            0xCC => self.call(bus, JumpCond::Zero),
            0xCD => self.call(bus, JumpCond::Always),
            0xD4 => self.call(bus, JumpCond::NotCarry),
            0xDC => self.call(bus, JumpCond::Carry),

            // return
            0xC0 => self.ret(bus, JumpCond::NotZero),
            0xC8 => self.ret(bus, JumpCond::Zero),
            0xC9 => self.ret(bus, JumpCond::Always),
            0xD0 => self.ret(bus, JumpCond::NotCarry),
            0xD8 => self.ret(bus, JumpCond::Carry),
            0xD9 => self.reti(bus, JumpCond::Always),

            // call reset vetor
            0xC7 => self.rst(bus, 0x00),
            0xCF => self.rst(bus, 0x08),
            0xD7 => self.rst(bus, 0x10),
            0xDF => self.rst(bus, 0x18),
            0xE7 => self.rst(bus, 0x20),
            0xEF => self.rst(bus, 0x28),
            0xF7 => self.rst(bus, 0x30),
            0xFF => self.rst(bus, 0x38),

            // bitwise and
            0xA0 => self.and(bus, Reg8::B),
            0xA1 => self.and(bus, Reg8::C),
            0xA2 => self.and(bus, Reg8::D),
            0xA3 => self.and(bus, Reg8::E),
            0xA4 => self.and(bus, Reg8::H),
            0xA5 => self.and(bus, Reg8::L),
            0xA6 => self.and(bus, Addr::HL),
            0xA7 => self.and(bus, Reg8::A),
            0xE6 => self.and(bus, ImmediateByte),

            // bitwise xor
            0xA8 => self.xor(bus, Reg8::B),
            0xA9 => self.xor(bus, Reg8::C),
            0xAA => self.xor(bus, Reg8::D),
            0xAB => self.xor(bus, Reg8::E),
            0xAC => self.xor(bus, Reg8::H),
            0xAD => self.xor(bus, Reg8::L),
            0xAE => self.xor(bus, Addr::HL),
            0xAF => self.xor(bus, Reg8::A),
            0xEE => self.xor(bus, ImmediateByte),

            // bitwise or
            0xB0 => self.or(bus, Reg8::B),
            0xB1 => self.or(bus, Reg8::C),
            0xB2 => self.or(bus, Reg8::D),
            0xB3 => self.or(bus, Reg8::E),
            0xB4 => self.or(bus, Reg8::H),
            0xB5 => self.or(bus, Reg8::L),
            0xB6 => self.or(bus, Addr::HL),
            0xB7 => self.or(bus, Reg8::A),
            0xF6 => self.or(bus, ImmediateByte),

            // rotations
            0x07 => self.rlca(bus, ),
            0x17 => self.rla(bus, ),
            0x0F => self.rrca(bus, ),
            0x1F => self.rra(bus, ),

            // compare
            0xB8 => self.cp(bus, Reg8::B),
            0xB9 => self.cp(bus, Reg8::C),
            0xBA => self.cp(bus, Reg8::D),
            0xBB => self.cp(bus, Reg8::E),
            0xBC => self.cp(bus, Reg8::H),
            0xBD => self.cp(bus, Reg8::L),
            0xBE => self.cp(bus, Addr::HL),
            0xBF => self.cp(bus, Reg8::A),
            0xFE => self.cp(bus, ImmediateByte),

            // complement A register
            0x2F => self.cpl(bus),

            // set carry flag
            0x37 => self.scf(bus),

            // complement carry flag
            0x3F => self.ccf(bus),

            // misc
            0x10 => self.stop(bus),
            0x27 => self.daa(bus),
            0x76 => self.halt(bus),
            0xF3 => self.di(bus),
            0xFB => self.ei(bus),

            // prefixed instructions
            0xCB => {
                self.opcode = self.fetch(bus);
                self.prefixed_decode(bus);
            }

            0xD3 | 0xDB | 0xDD | 0xE3 | 0xE4 | 0xEB | 0xEC | 0xED | 0xF4 | 0xFC | 0xFD =>
                panic!("Invalid opcode {:#X}", self.opcode),
        }
    }

    pub fn prefixed_decode(&mut self, bus: &mut Bus) {
        match self.opcode {
            0x00 => self.rlc(bus, Reg8::B),
            0x01 => self.rlc(bus, Reg8::C),
            0x02 => self.rlc(bus, Reg8::D),
            0x03 => self.rlc(bus, Reg8::E),
            0x04 => self.rlc(bus, Reg8::H),
            0x05 => self.rlc(bus, Reg8::L),
            0x06 => self.rlc(bus, Addr::HL),
            0x07 => self.rlc(bus, Reg8::A),
            0x08 => self.rrc(bus, Reg8::B),
            0x09 => self.rrc(bus, Reg8::C),
            0x0A => self.rrc(bus, Reg8::D),
            0x0B => self.rrc(bus, Reg8::E),
            0x0C => self.rrc(bus, Reg8::H),
            0x0D => self.rrc(bus, Reg8::L),
            0x0E => self.rrc(bus, Addr::HL),
            0x0F => self.rrc(bus, Reg8::A),
            0x10 => self.rl(bus, Reg8::B),
            0x11 => self.rl(bus, Reg8::C),
            0x12 => self.rl(bus, Reg8::D),
            0x13 => self.rl(bus, Reg8::E),
            0x14 => self.rl(bus, Reg8::H),
            0x15 => self.rl(bus, Reg8::L),
            0x16 => self.rl(bus, Addr::HL),
            0x17 => self.rl(bus, Reg8::A),
            0x18 => self.rr(bus, Reg8::B),
            0x19 => self.rr(bus, Reg8::C),
            0x1A => self.rr(bus, Reg8::D),
            0x1B => self.rr(bus, Reg8::E),
            0x1C => self.rr(bus, Reg8::H),
            0x1D => self.rr(bus, Reg8::L),
            0x1E => self.rr(bus, Addr::HL),
            0x1F => self.rr(bus, Reg8::A),
            0x20 => self.sla(bus, Reg8::B),
            0x21 => self.sla(bus, Reg8::C),
            0x22 => self.sla(bus, Reg8::D),
            0x23 => self.sla(bus, Reg8::E),
            0x24 => self.sla(bus, Reg8::H),
            0x25 => self.sla(bus, Reg8::L),
            0x26 => self.sla(bus, Addr::HL),
            0x27 => self.sla(bus, Reg8::A),
            0x28 => self.sra(bus, Reg8::B),
            0x29 => self.sra(bus, Reg8::C),
            0x2A => self.sra(bus, Reg8::D),
            0x2B => self.sra(bus, Reg8::E),
            0x2C => self.sra(bus, Reg8::H),
            0x2D => self.sra(bus, Reg8::L),
            0x2E => self.sra(bus, Addr::HL),
            0x2F => self.sra(bus, Reg8::A),
            0x30 => self.swap(bus, Reg8::B),
            0x31 => self.swap(bus, Reg8::C),
            0x32 => self.swap(bus, Reg8::D),
            0x33 => self.swap(bus, Reg8::E),
            0x34 => self.swap(bus, Reg8::H),
            0x35 => self.swap(bus, Reg8::L),
            0x36 => self.swap(bus, Addr::HL),
            0x37 => self.swap(bus, Reg8::A),
            0x38 => self.srl(bus, Reg8::B),
            0x39 => self.srl(bus, Reg8::C),
            0x3A => self.srl(bus, Reg8::D),
            0x3B => self.srl(bus, Reg8::E),
            0x3C => self.srl(bus, Reg8::H),
            0x3D => self.srl(bus, Reg8::L),
            0x3E => self.srl(bus, Addr::HL),
            0x3F => self.srl(bus, Reg8::A),
            0x40 => self.bit(bus, 0, Reg8::B),
            0x41 => self.bit(bus, 0, Reg8::C),
            0x42 => self.bit(bus, 0, Reg8::D),
            0x43 => self.bit(bus, 0, Reg8::E),
            0x44 => self.bit(bus, 0, Reg8::H),
            0x45 => self.bit(bus, 0, Reg8::L),
            0x46 => self.bit(bus, 0, Addr::HL),
            0x47 => self.bit(bus, 0, Reg8::A),
            0x48 => self.bit(bus, 1, Reg8::B),
            0x49 => self.bit(bus, 1, Reg8::C),
            0x4A => self.bit(bus, 1, Reg8::D),
            0x4B => self.bit(bus, 1, Reg8::E),
            0x4C => self.bit(bus, 1, Reg8::H),
            0x4D => self.bit(bus, 1, Reg8::L),
            0x4E => self.bit(bus, 1, Addr::HL),
            0x4F => self.bit(bus, 1, Reg8::A),
            0x50 => self.bit(bus, 2, Reg8::B),
            0x51 => self.bit(bus, 2, Reg8::C),
            0x52 => self.bit(bus, 2, Reg8::D),
            0x53 => self.bit(bus, 2, Reg8::E),
            0x54 => self.bit(bus, 2, Reg8::H),
            0x55 => self.bit(bus, 2, Reg8::L),
            0x56 => self.bit(bus, 2, Addr::HL),
            0x57 => self.bit(bus, 2, Reg8::A),
            0x58 => self.bit(bus, 3, Reg8::B),
            0x59 => self.bit(bus, 3, Reg8::C),
            0x5A => self.bit(bus, 3, Reg8::D),
            0x5B => self.bit(bus, 3, Reg8::E),
            0x5C => self.bit(bus, 3, Reg8::H),
            0x5D => self.bit(bus, 3, Reg8::L),
            0x5E => self.bit(bus, 3, Addr::HL),
            0x5F => self.bit(bus, 3, Reg8::A),
            0x60 => self.bit(bus, 4, Reg8::B),
            0x61 => self.bit(bus, 4, Reg8::C),
            0x62 => self.bit(bus, 4, Reg8::D),
            0x63 => self.bit(bus, 4, Reg8::E),
            0x64 => self.bit(bus, 4, Reg8::H),
            0x65 => self.bit(bus, 4, Reg8::L),
            0x66 => self.bit(bus, 4, Addr::HL),
            0x67 => self.bit(bus, 4, Reg8::A),
            0x68 => self.bit(bus, 5, Reg8::B),
            0x69 => self.bit(bus, 5, Reg8::C),
            0x6A => self.bit(bus, 5, Reg8::D),
            0x6B => self.bit(bus, 5, Reg8::E),
            0x6C => self.bit(bus, 5, Reg8::H),
            0x6D => self.bit(bus, 5, Reg8::L),
            0x6E => self.bit(bus, 5, Addr::HL),
            0x6F => self.bit(bus, 5, Reg8::A),
            0x70 => self.bit(bus, 6, Reg8::B),
            0x71 => self.bit(bus, 6, Reg8::C),
            0x72 => self.bit(bus, 6, Reg8::D),
            0x73 => self.bit(bus, 6, Reg8::E),
            0x74 => self.bit(bus, 6, Reg8::H),
            0x75 => self.bit(bus, 6, Reg8::L),
            0x76 => self.bit(bus, 6, Addr::HL),
            0x77 => self.bit(bus, 6, Reg8::A),
            0x78 => self.bit(bus, 7, Reg8::B),
            0x79 => self.bit(bus, 7, Reg8::C),
            0x7A => self.bit(bus, 7, Reg8::D),
            0x7B => self.bit(bus, 7, Reg8::E),
            0x7C => self.bit(bus, 7, Reg8::H),
            0x7D => self.bit(bus, 7, Reg8::L),
            0x7E => self.bit(bus, 7, Addr::HL),
            0x7F => self.bit(bus, 7, Reg8::A),
            0x80 => self.res(bus, 0, Reg8::B),
            0x81 => self.res(bus, 0, Reg8::C),
            0x82 => self.res(bus, 0, Reg8::D),
            0x83 => self.res(bus, 0, Reg8::E),
            0x84 => self.res(bus, 0, Reg8::H),
            0x85 => self.res(bus, 0, Reg8::L),
            0x86 => self.res(bus, 0, Addr::HL),
            0x87 => self.res(bus, 0, Reg8::A),
            0x88 => self.res(bus, 1, Reg8::B),
            0x89 => self.res(bus, 1, Reg8::C),
            0x8A => self.res(bus, 1, Reg8::D),
            0x8B => self.res(bus, 1, Reg8::E),
            0x8C => self.res(bus, 1, Reg8::H),
            0x8D => self.res(bus, 1, Reg8::L),
            0x8E => self.res(bus, 1, Addr::HL),
            0x8F => self.res(bus, 1, Reg8::A),
            0x90 => self.res(bus, 2, Reg8::B),
            0x91 => self.res(bus, 2, Reg8::C),
            0x92 => self.res(bus, 2, Reg8::D),
            0x93 => self.res(bus, 2, Reg8::E),
            0x94 => self.res(bus, 2, Reg8::H),
            0x95 => self.res(bus, 2, Reg8::L),
            0x96 => self.res(bus, 2, Addr::HL),
            0x97 => self.res(bus, 2, Reg8::A),
            0x98 => self.res(bus, 3, Reg8::B),
            0x99 => self.res(bus, 3, Reg8::C),
            0x9A => self.res(bus, 3, Reg8::D),
            0x9B => self.res(bus, 3, Reg8::E),
            0x9C => self.res(bus, 3, Reg8::H),
            0x9D => self.res(bus, 3, Reg8::L),
            0x9E => self.res(bus, 3, Addr::HL),
            0x9F => self.res(bus, 3, Reg8::A),
            0xA0 => self.res(bus, 4, Reg8::B),
            0xA1 => self.res(bus, 4, Reg8::C),
            0xA2 => self.res(bus, 4, Reg8::D),
            0xA3 => self.res(bus, 4, Reg8::E),
            0xA4 => self.res(bus, 4, Reg8::H),
            0xA5 => self.res(bus, 4, Reg8::L),
            0xA6 => self.res(bus, 4, Addr::HL),
            0xA7 => self.res(bus, 4, Reg8::A),
            0xA8 => self.res(bus, 5, Reg8::B),
            0xA9 => self.res(bus, 5, Reg8::C),
            0xAA => self.res(bus, 5, Reg8::D),
            0xAB => self.res(bus, 5, Reg8::E),
            0xAC => self.res(bus, 5, Reg8::H),
            0xAD => self.res(bus, 5, Reg8::L),
            0xAE => self.res(bus, 5, Addr::HL),
            0xAF => self.res(bus, 5, Reg8::A),
            0xB0 => self.res(bus, 6, Reg8::B),
            0xB1 => self.res(bus, 6, Reg8::C),
            0xB2 => self.res(bus, 6, Reg8::D),
            0xB3 => self.res(bus, 6, Reg8::E),
            0xB4 => self.res(bus, 6, Reg8::H),
            0xB5 => self.res(bus, 6, Reg8::L),
            0xB6 => self.res(bus, 6, Addr::HL),
            0xB7 => self.res(bus, 6, Reg8::A),
            0xB8 => self.res(bus, 7, Reg8::B),
            0xB9 => self.res(bus, 7, Reg8::C),
            0xBA => self.res(bus, 7, Reg8::D),
            0xBB => self.res(bus, 7, Reg8::E),
            0xBC => self.res(bus, 7, Reg8::H),
            0xBD => self.res(bus, 7, Reg8::L),
            0xBE => self.res(bus, 7, Addr::HL),
            0xBF => self.res(bus, 7, Reg8::A),
            0xC0 => self.set(bus, 0, Reg8::B),
            0xC1 => self.set(bus, 0, Reg8::C),
            0xC2 => self.set(bus, 0, Reg8::D),
            0xC3 => self.set(bus, 0, Reg8::E),
            0xC4 => self.set(bus, 0, Reg8::H),
            0xC5 => self.set(bus, 0, Reg8::L),
            0xC6 => self.set(bus, 0, Addr::HL),
            0xC7 => self.set(bus, 0, Reg8::A),
            0xC8 => self.set(bus, 1, Reg8::B),
            0xC9 => self.set(bus, 1, Reg8::C),
            0xCA => self.set(bus, 1, Reg8::D),
            0xCB => self.set(bus, 1, Reg8::E),
            0xCC => self.set(bus, 1, Reg8::H),
            0xCD => self.set(bus, 1, Reg8::L),
            0xCE => self.set(bus, 1, Addr::HL),
            0xCF => self.set(bus, 1, Reg8::A),
            0xD0 => self.set(bus, 2, Reg8::B),
            0xD1 => self.set(bus, 2, Reg8::C),
            0xD2 => self.set(bus, 2, Reg8::D),
            0xD3 => self.set(bus, 2, Reg8::E),
            0xD4 => self.set(bus, 2, Reg8::H),
            0xD5 => self.set(bus, 2, Reg8::L),
            0xD6 => self.set(bus, 2, Addr::HL),
            0xD7 => self.set(bus, 2, Reg8::A),
            0xD8 => self.set(bus, 3, Reg8::B),
            0xD9 => self.set(bus, 3, Reg8::C),
            0xDA => self.set(bus, 3, Reg8::D),
            0xDB => self.set(bus, 3, Reg8::E),
            0xDC => self.set(bus, 3, Reg8::H),
            0xDD => self.set(bus, 3, Reg8::L),
            0xDE => self.set(bus, 3, Addr::HL),
            0xDF => self.set(bus, 3, Reg8::A),
            0xE0 => self.set(bus, 4, Reg8::B),
            0xE1 => self.set(bus, 4, Reg8::C),
            0xE2 => self.set(bus, 4, Reg8::D),
            0xE3 => self.set(bus, 4, Reg8::E),
            0xE4 => self.set(bus, 4, Reg8::H),
            0xE5 => self.set(bus, 4, Reg8::L),
            0xE6 => self.set(bus, 4, Addr::HL),
            0xE7 => self.set(bus, 4, Reg8::A),
            0xE8 => self.set(bus, 5, Reg8::B),
            0xE9 => self.set(bus, 5, Reg8::C),
            0xEA => self.set(bus, 5, Reg8::D),
            0xEB => self.set(bus, 5, Reg8::E),
            0xEC => self.set(bus, 5, Reg8::H),
            0xED => self.set(bus, 5, Reg8::L),
            0xEE => self.set(bus, 5, Addr::HL),
            0xEF => self.set(bus, 5, Reg8::A),
            0xF0 => self.set(bus, 6, Reg8::B),
            0xF1 => self.set(bus, 6, Reg8::C),
            0xF2 => self.set(bus, 6, Reg8::D),
            0xF3 => self.set(bus, 6, Reg8::E),
            0xF4 => self.set(bus, 6, Reg8::H),
            0xF5 => self.set(bus, 6, Reg8::L),
            0xF6 => self.set(bus, 6, Addr::HL),
            0xF7 => self.set(bus, 6, Reg8::A),
            0xF8 => self.set(bus, 7, Reg8::B),
            0xF9 => self.set(bus, 7, Reg8::C),
            0xFA => self.set(bus, 7, Reg8::D),
            0xFB => self.set(bus, 7, Reg8::E),
            0xFC => self.set(bus, 7, Reg8::H),
            0xFD => self.set(bus, 7, Reg8::L),
            0xFE => self.set(bus, 7, Addr::HL),
            0xFF => self.set(bus, 7, Reg8::A),
        }
    }
}
