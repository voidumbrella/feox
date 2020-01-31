use crate::cpu::{Cpu, ByteSrc, ByteDest, WordSrc, WordDest};
use crate::cpu::registers::Reg16;
use crate::cpu::decode::JumpCond;
use crate::emulator::Emulator;

impl Cpu {
    fn cond_met(&self, cond: JumpCond) -> bool {
        match cond {
            JumpCond::Always => true,
            JumpCond::NotZero => !self.regs.zero(),
            JumpCond::Zero => self.regs.zero(),
            JumpCond::NotCarry => !self.regs.carry(),
            JumpCond::Carry => self.regs.carry(),
        }
    }

    pub fn halt(&mut self, _: &mut Emulator) {
        self.halted = true;
    }

    pub fn stop(&mut self, _: &mut Emulator) {
        panic!("STOP")
    }

    pub fn di(&mut self, _: &mut Emulator) {
        self.interrupt_enabled = false;
    }

    pub fn ei(&mut self, _: &mut Emulator) {
        self.interrupt_enabled = true;
    }

    pub fn load<D: ByteDest, S: ByteSrc>(&mut self, emulator: &mut Emulator, dest: D, src: S) {
        let value = src.read(self, emulator);
        dest.write(self, emulator, value);
    }

    pub fn add<T: ByteSrc>(&mut self, emulator: &mut Emulator, source: T) {
        let addend = source.read(self, emulator);
        let (new_value, overflowed) = self.regs.a.overflowing_add(addend);

        self.regs.set_zero(new_value == 0);
        self.regs.set_sub(false);
        self.regs.set_half_carry((self.regs.a & 0xF) + (addend & 0xF) > 0xF);
        self.regs.set_carry(overflowed);

        self.regs.a = new_value;
    }

    pub fn adc<T: ByteSrc>(&mut self, emulator: &mut Emulator, source: T) {
        let carry = if self.regs.carry() { 1 } else { 0 };
        let addend = source.read(self, emulator);

        // Two separate additions are needed to properly check for carry:
        // If add 0xFF with carry bit the set, adding the addends first
        // will result in addition by 0x00 and end up not setting the carry flag.
        let (temp, overflowed) = self.regs.a.overflowing_add(addend);
        let (new_value, overflowed_2) = temp.overflowing_add(carry);

        self.regs.set_zero(new_value == 0);
        self.regs.set_sub(false);
        self.regs.set_half_carry((self.regs.a & 0xF) + (addend & 0xF) + carry > 0xF);
        self.regs.set_carry(overflowed || overflowed_2);

        self.regs.a = new_value;
    }

    pub fn sub<T: ByteSrc>(&mut self, emulator: &mut Emulator, source: T) {
        let subtrahend = source.read(self, emulator);
        let (new_value, overflowed) = self.regs.a.overflowing_sub(subtrahend);

        self.regs.set_zero(new_value == 0);
        self.regs.set_sub(true);
        self.regs.set_half_carry(self.regs.a & 0xF < subtrahend & 0xF);
        self.regs.set_carry(overflowed);

        self.regs.a = new_value;
    }

    pub fn sbc<T: ByteSrc>(&mut self, emulator: &mut Emulator, source: T) {
        let carry = if self.regs.carry() { 1 } else { 0 };
        let subtrahend = source.read(self, emulator);

        // See `adc`
        let (temp, overflowed) = self.regs.a.overflowing_sub(subtrahend);
        let (new_value, overflowed_2) = temp.overflowing_sub(carry);

        self.regs.set_zero(new_value == 0);
        self.regs.set_sub(true);
        self.regs.set_half_carry(
            (self.regs.a & 0xF).wrapping_sub(subtrahend & 0xF).wrapping_sub(carry)
            & (0b10000) != 0);
        self.regs.set_carry(overflowed || overflowed_2);

        self.regs.a = new_value;
    }

    pub fn and<T: ByteSrc>(&mut self, emulator: &mut Emulator, source: T) {
        let new_value = self.regs.a & source.read(self, emulator);

        self.regs.set_zero(new_value == 0);
        self.regs.set_sub(false);
        self.regs.set_half_carry(true);
        self.regs.set_carry(false);

        self.regs.a = new_value;
    }

    pub fn xor<T: ByteSrc>(&mut self, emulator: &mut Emulator, source: T) {
        let new_value = self.regs.a ^ source.read(self, emulator);

        self.regs.set_zero(new_value == 0);
        self.regs.set_sub(false);
        self.regs.set_half_carry(false);
        self.regs.set_carry(false);

        self.regs.a = new_value;
    }

    pub fn or<T: ByteSrc>(&mut self, emulator: &mut Emulator, source: T) {
        let new_value = self.regs.a | source.read(self, emulator);

        self.regs.set_zero(new_value == 0);
        self.regs.set_sub(false);
        self.regs.set_half_carry(false);
        self.regs.set_carry(false);

        self.regs.a = new_value;
    }

    pub fn cp<T: ByteSrc>(&mut self, emulator: &mut Emulator, source: T) {
        let subtrahend = source.read(self, emulator);
        let (new_value, overflowed) = self.regs.a.overflowing_sub(subtrahend);

        self.regs.set_zero(new_value == 0);
        self.regs.set_sub(true);
        self.regs.set_half_carry(self.regs.a & 0xF < subtrahend & 0xF);
        self.regs.set_carry(overflowed);
    }

    pub fn cpl(&mut self, _: &mut Emulator) {
        self.regs.a = !self.regs.a;
        self.regs.set_sub(true);
        self.regs.set_half_carry(true);
    }

    pub fn scf(&mut self, _: &mut Emulator) {
        self.regs.set_sub(false);
        self.regs.set_half_carry(false);
        self.regs.set_carry(true);
    }

    pub fn ccf(&mut self, _: &mut Emulator) {
        self.regs.set_sub(false);
        self.regs.set_half_carry(false);
        self.regs.set_carry(!self.regs.carry());
    }

    pub fn inc<T: ByteSrc + ByteDest>(&mut self, emulator: &mut Emulator, target: T) {
        let old_value = target.read(self, emulator);
        let new_value = old_value.wrapping_add(1);

        self.regs.set_zero(new_value == 0);
        self.regs.set_sub(false);
        self.regs.set_half_carry(old_value & 0xF == 0xF);

        target.write(self, emulator, new_value);
    }

    pub fn dec<T: ByteSrc + ByteDest>(&mut self, emulator: &mut Emulator, target: T) {
        let old_value = target.read(self, emulator);
        let new_value = old_value.wrapping_sub(1);

        self.regs.set_zero(new_value == 0);
        self.regs.set_sub(true);
        self.regs.set_half_carry(old_value & 0xF == 0);

        target.write(self, emulator, new_value);
    }

    pub fn rlca(&mut self, _: &mut Emulator) {
        let a = self.regs.a;
        self.regs.a = a.rotate_left(1);

        self.regs.set_zero(false);
        self.regs.set_sub(false);
        self.regs.set_half_carry(false);
        self.regs.set_carry(a & 0b1000_0000 != 0);
    }

    pub fn rrca(&mut self, _: &mut Emulator) {
        let a = self.regs.a;
        self.regs.a = a.rotate_right(1);

        self.regs.set_zero(false);
        self.regs.set_sub(false);
        self.regs.set_half_carry(false);
        self.regs.set_carry(a & 0b0000_0001 != 0);
    }

    pub fn rla(&mut self, _: &mut Emulator) {
        let a = self.regs.a;
        self.regs.a = a << 1;
        if self.regs.carry() {
            self.regs.a |= 0b0000_0001;
        }

        self.regs.set_zero(false);
        self.regs.set_sub(false);
        self.regs.set_half_carry(false);
        self.regs.set_carry(a & 0b1000_0000 != 0);
    }

    pub fn rra(&mut self, _: &mut Emulator) {
        let a = self.regs.a;
        self.regs.a = a >> 1;
        if self.regs.carry() {
            self.regs.a |= 0b1000_0000;
        }

        self.regs.set_zero(false);
        self.regs.set_sub(false);
        self.regs.set_half_carry(false);
        self.regs.set_carry(a & 0b0000_0001 != 0);
    }

    pub fn load16<D: WordDest, S: WordSrc>(&mut self, emulator: &mut Emulator, dest: D, src: S) {
        let value = src.read(self, emulator);
        dest.write(self, emulator, value);
    }

    pub fn load16_sphl(&mut self, emulator: &mut Emulator) {
        self.regs.sp = self.regs.read_pair(Reg16::HL);
        emulator.step();
    }

    pub fn load16_hlsp_n(&mut self, emulator: &mut Emulator) {
        let offset = self.fetch(emulator) as i8 as u16;
        let sp = self.regs.sp;
        let new_hl = sp.wrapping_add(offset);
        Reg16::HL.write(self, emulator, new_hl);

        self.regs.set_zero(false);
        self.regs.set_sub(false);
        self.regs.set_half_carry((sp & 0xF) + (offset & 0xF) > 0xF);
        self.regs.set_carry((sp & 0xFF) + (offset & 0xFF) > 0xFF);
        emulator.step();
    }

    pub fn inc16<T: WordSrc + WordDest>(&mut self, emulator: &mut Emulator, target: T) {
        // 2 cycles to increment a register pair
        let old_value = target.read(self, emulator);
        let new_value = old_value.wrapping_add(1);
        target.write(self, emulator, new_value);
        emulator.step();
    }

    pub fn dec16<T: WordSrc + WordDest>(&mut self, emulator: &mut Emulator, target: T) {
        // 2 cycles to decrement a register pair
        let old_value = target.read(self, emulator);
        let new_value = old_value.wrapping_sub(1);
        target.write(self, emulator, new_value);
        emulator.step();
    }

    pub fn add16_sp_n(&mut self, emulator: &mut Emulator) {
        // 4 cycles
        let offset = self.fetch(emulator) as i8 as u16;
        let sp = self.regs.sp;

        self.regs.set_zero(false);
        self.regs.set_sub(false);
        self.regs.set_half_carry((sp & 0xF) + (offset & 0xF) > 0xF);
        self.regs.set_carry((sp & 0xFF) + (offset & 0xFF) > 0xFF);

        self.regs.sp = sp.wrapping_add(offset);
        emulator.step();
        emulator.step();
    }

    pub fn add16hl(&mut self, emulator: &mut Emulator, src: Reg16) {
        // 2 cycles
        let addend = self.regs.read_pair(src);
        let hl = self.regs.read_pair(Reg16::HL);
        let (new_value, overflowed) = hl.overflowing_add(addend);

        self.regs.set_sub(false);
        let mask_11bits = 0b00001111_11111111;
        self.regs.set_half_carry((hl & mask_11bits) + (addend & mask_11bits) > mask_11bits);
        self.regs.set_carry(overflowed);

        self.regs.write_pair(Reg16::HL, new_value);
        emulator.step();
    }

    pub fn push<T: WordSrc>(&mut self, emulator: &mut Emulator, src: T) {
        // 4 cycles
        let value = src.read(self, emulator);
        self.regs.sp = self.regs.sp.wrapping_sub(2);
        emulator.step();
        self.write_word(emulator, self.regs.sp, value);
    }

    pub fn pop<T: WordDest>(&mut self, emulator: &mut Emulator, dest: T) {
        // 3 cycles
        let value = self.read_word(emulator, self.regs.sp);
        dest.write(self, emulator, value);
        self.regs.sp = self.regs.sp.wrapping_add(2);
    }

    pub fn call(&mut self, emulator: &mut Emulator, cond: JumpCond) {
        // 6 cycles if taken,
        // 3 cycles otherwise
        let address = self.fetch_word(emulator);
        if self.cond_met(cond) {
            self.push(emulator, Reg16::PC);
            self.regs.pc = address;
        }
    }

    pub fn jr(&mut self, emulator: &mut Emulator, cond: JumpCond) {
        // 3 cycles if taken,
        // 2 cycles otherwise
        let offset = self.fetch(emulator) as i8 as u16;
        if self.cond_met(cond) {
            self.regs.pc = self.regs.pc.wrapping_add(offset);
            emulator.step();
        }
    }

    pub fn jp<T: WordSrc>(&mut self, emulator: &mut Emulator, cond: JumpCond, jump_to: T) {
        // 4 cycles if taken,
        // 3 cycles otherwise
        let new_pc = jump_to.read(self, emulator);
        if self.cond_met(cond) {
            self.regs.pc = new_pc;
            emulator.step();
        }
    }

    pub fn jp_hl(&mut self, _: &mut Emulator) {
        self.regs.pc = self.regs.read_pair(Reg16::HL);
    }

    pub fn ret(&mut self, emulator: &mut Emulator, cond: JumpCond) {
        // conditional RET takes 5 cycles (when taken)
        // unconditional RET only takes 4 cycles
        if cond != JumpCond::Always {
            emulator.step();
        }
        if self.cond_met(cond) {
            self.pop(emulator, Reg16::PC);
            emulator.step();
        }
    }

    pub fn reti(&mut self, emulator: &mut Emulator, cond: JumpCond) {
        // 4 cycles
        if self.cond_met(cond) {
            self.pop(emulator, Reg16::PC);
            emulator.step();
            self.interrupt_enabled = true;
        }
    }

    pub fn rst(&mut self, emulator: &mut Emulator, address: u16) {
        self.push(emulator, Reg16::PC);
        self.regs.pc = address;
    }

    pub fn daa(&mut self, _: &mut Emulator) {
        if !self.regs.sub() {
            if self.regs.carry() || self.regs.a > 0x99 {
                self.regs.a = self.regs.a.wrapping_add(0x60);
                self.regs.set_carry(true);
            }
            if self.regs.half_carry() || self.regs.a & 0x0f > 0x09 {
                self.regs.a = self.regs.a.wrapping_add(0x06);
            }
        } else {
            if self.regs.carry() {
                self.regs.a = self.regs.a.wrapping_sub(0x60);
            }
            if self.regs.half_carry() {
                self.regs.a = self.regs.a.wrapping_sub(0x06);
            }
        }

        self.regs.set_zero(self.regs.a == 0);
        self.regs.set_half_carry(false);
    }

    pub fn rlc<T: ByteSrc + ByteDest>(&mut self, emulator: &mut Emulator, target: T) {
        let old_value = target.read(self, emulator);
        let new_value = old_value.rotate_left(1);
        target.write(self, emulator, new_value);

        self.regs.set_zero(new_value == 0);
        self.regs.set_sub(false);
        self.regs.set_half_carry(false);
        self.regs.set_carry(old_value & 0b1000_0000 != 0);
    }

    pub fn rrc<T: ByteSrc + ByteDest>(&mut self, emulator: &mut Emulator, target: T) {
        let old_value = target.read(self, emulator);
        let new_value = old_value.rotate_right(1);
        target.write(self, emulator, new_value);

        self.regs.set_zero(new_value == 0);
        self.regs.set_sub(false);
        self.regs.set_half_carry(false);
        self.regs.set_carry(old_value & 0b0000_0001 != 0);
    }

    pub fn rl<T: ByteSrc + ByteDest>(&mut self, emulator: &mut Emulator, target: T) {
        let old_value = target.read(self, emulator);
        let mut new_value = old_value << 1;
        if self.regs.carry() {
            new_value |= 0b0000_0001;
        }
        target.write(self, emulator, new_value);

        self.regs.set_zero(new_value == 0);
        self.regs.set_sub(false);
        self.regs.set_half_carry(false);
        self.regs.set_carry(old_value & 0b1000_0000 != 0);
    }

    pub fn rr<T: ByteSrc + ByteDest>(&mut self, emulator: &mut Emulator, target: T) {
        let old_value = target.read(self, emulator);
        let mut new_value = old_value >> 1;
        if self.regs.carry() {
            new_value |= 0b1000_0000;
        }
        target.write(self, emulator, new_value);

        self.regs.set_zero(new_value == 0);
        self.regs.set_sub(false);
        self.regs.set_half_carry(false);
        self.regs.set_carry(old_value & 0b0000_0001 != 0);
    }

    pub fn sla<T: ByteSrc + ByteDest>(&mut self, emulator: &mut Emulator, target: T) {
        // C <- [7 <- 0] <- 0
        let old_value = target.read(self, emulator);
        let new_value = old_value << 1;
        target.write(self, emulator, new_value);

        self.regs.set_zero(new_value == 0);
        self.regs.set_sub(false);
        self.regs.set_half_carry(false);
        self.regs.set_carry(old_value & 0b1000_0000 != 0);
    }

    pub fn sra<T: ByteSrc + ByteDest>(&mut self, emulator: &mut Emulator, target: T) {
        // [0] -> [7 -> 0] -> C
        let old_value = target.read(self, emulator);
        let hi = old_value & 0b1000_0000;
        let new_value = hi | old_value >> 1;
        target.write(self, emulator, new_value);

        self.regs.set_zero(new_value == 0);
        self.regs.set_sub(false);
        self.regs.set_half_carry(false);
        self.regs.set_carry(old_value & 0b0000_0001 != 0);
    }

    pub fn swap<T: ByteSrc + ByteDest>(&mut self, emulator: &mut Emulator, target: T) {
        let old_value = target.read(self, emulator);
        let lo = old_value & 0xF;
        let hi = old_value >> 4;
        let new_value = lo << 4 | hi;
        target.write(self, emulator, new_value);

        self.regs.set_zero(new_value == 0);
        self.regs.set_sub(false);
        self.regs.set_half_carry(false);
        self.regs.set_carry(false);
    }

    pub fn srl<T: ByteSrc + ByteDest>(&mut self, emulator: &mut Emulator, target: T) {
        // 0 -> [7 -> 0] -> C
        let old_value = target.read(self, emulator);
        let new_value = old_value >> 1;
        target.write(self, emulator, new_value);

        self.regs.set_zero(new_value == 0);
        self.regs.set_sub(false);
        self.regs.set_half_carry(false);
        self.regs.set_carry(old_value & 0b0000_0001 != 0);
    }

    pub fn bit<T: ByteSrc>(&mut self, emulator: &mut Emulator, bit: u8, src: T) {
        let n = src.read(self, emulator);
        self.regs.set_zero(n & (1 << bit) == 0);
        self.regs.set_sub(false);
        self.regs.set_half_carry(true);
    }

    pub fn res<T: ByteDest + ByteSrc>(&mut self, emulator: &mut Emulator, bit: u8, target: T) {
        let n = target.read(self, emulator);
        target.write(self, emulator, n & !(1 << bit));
    }

    pub fn set<T: ByteDest + ByteSrc>(&mut self, emulator: &mut Emulator, bit: u8, target: T) {
        let n = target.read(self, emulator);
        target.write(self, emulator, n | (1 << bit));
    }
}
