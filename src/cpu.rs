use std::fmt::{Debug, Formatter};
use crate::flags::{Flag, Flags};
use crate::registers::*;
use crate::instructions::*;
use crate::memory::*;
use crate::opcode_parser::*;


#[derive(Debug)]
pub struct ProgramCounter {
    pub value: u16,
}

pub struct Cpu {
    regs: Registers,
    flags: Flags,
    pub mem: Memory,
    pc: ProgramCounter,
    cycles: u16,
}

impl Debug for Cpu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cpu state:\n{:?} \n{:?} \n{:?} \nCycles: {:?} \nMemory:\n{:?}", self.regs, self.flags, self.pc, self.cycles, self.mem)
    }
}

impl Cpu {
    pub fn new(mem: Memory) -> Self {
        Cpu {
            regs: Registers::new(),
            flags: Flags::new(),
            mem,
            pc: ProgramCounter { value: 0 },
            cycles: 0,
        }
    }

    pub fn step(&mut self) {
        let opcode = Opcode { value: self.read_pcaddr8() };
        let instruction = opcode.to_instruction();
        // println!("Executing instruction {opcode:?}, {instruction:?}");
        self.execute_instruction(&instruction);
        self.update_cycles(&instruction, &opcode);
    }

    fn execute_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Noop => {}
            Instruction::Stop => {} // Todo switch speed
            Instruction::Halt => {} // Todo halt,

            Instruction::AddA { src, carry } => self.add_a(src, carry),
            Instruction::AddHl { src } => self.add_hl(src),
            Instruction::AddSpAddr8ToSp => self.add_8_to_sp(),
            Instruction::SubA { src, carry } => self.sub_a(src, carry),
            Instruction::AndA { src } => self.and_a(src),
            Instruction::XorA { src } => self.xor_a(src),
            Instruction::OrA { src } => self.or_a(src),
            Instruction::CompareA { src } => self.compare_a(src),
            Instruction::Inc { dst } => self.inc(dst),
            Instruction::Inc16 { dst } => self.inc16(dst),
            Instruction::Dec { dst } => self.dec(dst),
            Instruction::Dec16 { dst } => self.dec16(dst),
            Instruction::RotateLeftA { carry } => self.rotate_left_a(carry),
            Instruction::RotateRightA { carry } => self.rotate_right_a(carry),

            Instruction::Load { dst, src } => self.load(dst, src),
            Instruction::Load16 { dst, src } => self.load16(dst, src),
            Instruction::LoadDstAddr { dst, src } => self.load_dst_addr(dst, src),
            Instruction::LoadSrcAddr { dst, src } => self.load_src_addr(dst, src),
            Instruction::LoadSpAddSpAddr8ToHl => self.load_sp_add_spaddr8_to_hl(),

            Instruction::Push { reg16 } => self.push(reg16),
            Instruction::Pop { reg16 } => self.pop(reg16),
            Instruction::Jump => self.jump(),
            Instruction::JumpIf { flag } => self.jump_if(flag),
            Instruction::JumpHL => self.jump_hl(),
            Instruction::JumpReg => self.jump_reg(),
            Instruction::JumpRegIf { flag } => self.jump_reg_if(flag),
            Instruction::Call => self.call(),
            Instruction::CallIf { flag } => self.call_if(flag),
            Instruction::Return => self.ret(),
            Instruction::ReturnIf { flag } => self.ret_if(flag),
            Instruction::ReturnInterrupt => self.ret_interrupt(),
            Instruction::Restart { addr } => self.restart(addr),

            Instruction::DecimalAdjustA => todo!(),
            Instruction::ComplementA => todo!(),
            Instruction::SetCarryFlag => self.set_carry_flag(),
            Instruction::ComplementCarryFlag => self.complement_carry_flag(),
            Instruction::SetInterrupts { enable } => todo!(),

            Instruction::NestedInstruction => {
                let opcode = Opcode { value: self.read_pcaddr8() };
                let bit_instruction = opcode.to_bit_instruction();
                println!("Executing bit instruction {opcode:?}, {bit_instruction:?}");
                self.execute_bit_instruction(&bit_instruction);
                // TODO cycles
            }
            _ => panic!("Instruction {instruction:?} not implemented")
        }
    }

    fn update_cycles(&mut self, instruction: &Instruction, opcode: &Opcode) {
        self.cycles += instruction.cycles(opcode, &self.flags) as u16;

        if self.cycles >= 456 {
            self.cycles = 0;

            let scanline: u8 = self.mem.read_addr8(0xFF44);
            let value = if scanline > 153 { 0 } else { scanline + 1 };

            self.mem.write_addr8(0xFF44, value);

            if value == 144 {
                // Todo interrupt
            }
        }
    }

    fn add_a(&mut self, src: &OpsTarget8, carry: &bool) {
        let a: u8 = self.regs.a;
        let b: u8 = self.read_opst8(src);
        let c: bool = *carry && self.flags.c;

        let (value, overflow) = a.carrying_add(b, c);
        self.regs.a = value;

        self.flags.z = value == 0;
        self.flags.n = false;
        self.flags.h = (a << 4).carrying_add(b << 4, c).1;
        self.flags.c = overflow;
    }

    fn add_hl(&mut self, src: &Register16) {
        let a: u16 = self.regs.hl();
        let b: u16 = self.regs.read_reg16(src);

        let (value, overflow) = a.overflowing_add(b);
        self.regs.hl_w(value);

        self.flags.z = value == 0;
        self.flags.n = false;
        self.flags.h = (a << 4).overflowing_add(b << 4).1;
        self.flags.c = overflow;
    }

    fn add_8_to_sp(&mut self) {
        // TODO i8
        let a: u16 = self.pc.value;
        let b: u16 = self.read_pcaddr8() as u16;

        let (value, overflow) = a.overflowing_add(b);
        self.regs.sp = value;

        self.flags.z = value == 0;
        self.flags.n = false;
        self.flags.h = (a << 4).overflowing_add(b << 4).1;
        self.flags.c = overflow;
    }

    fn sub_a(&mut self, src: &OpsTarget8, carry: &bool) {
        let a: u8 = self.regs.a;
        let b: u8 = self.read_opst8(src);
        let c: u8 = if *carry && self.flags.c { 1 } else { 0 };

        let (tmp, overflow1) = a.overflowing_sub(b);
        let (value, overflow2) = tmp.overflowing_sub(c);
        self.regs.a = value;

        let (half_tmp, half_overflow1) = (a << 4).overflowing_sub(b << 4);
        let half_overflow2: bool = half_tmp.overflowing_sub(c).1;

        self.flags.z = value == 0;
        self.flags.n = true;
        self.flags.h = half_overflow1 || half_overflow2;
        self.flags.c = overflow1 || overflow2;
    }

    fn and_a(&mut self, src: &OpsTarget8) {
        let value = self.regs.a & self.read_opst8(src);
        self.regs.a = value;

        self.flags.z = value == 0;
        self.flags.n = false;
        self.flags.h = true;
        self.flags.c = false;
    }

    fn xor_a(&mut self, src: &OpsTarget8) {
        let value = self.regs.a ^ self.read_opst8(src);
        self.regs.a = value;

        self.flags.z = value == 0;
        self.flags.n = false;
        self.flags.h = false;
        self.flags.c = false;
    }

    fn or_a(&mut self, src: &OpsTarget8) {
        let value = self.regs.a | self.read_opst8(src);
        self.regs.a = value;

        self.flags.z = value == 0;
        self.flags.n = false;
        self.flags.h = true;
        self.flags.c = false;
    }

    fn compare_a(&mut self, src: &OpsTarget8) {
        let a = self.regs.a;
        self.sub_a(src, &false);
        self.regs.a = a;
    }

    fn inc(&mut self, dst: &OpsTarget8) {
        let a: u8 = self.read_opst8(dst);
        let value = a.wrapping_add(1);
        self.write_opst8(dst, value);

        self.flags.z = value == 0;
        self.flags.n = false;
        self.flags.h = (a << 4).overflowing_add(1).1;
    }

    fn inc16(&mut self, dst: &OpsTarget16) {
        let a: u16 = self.read_opst16(dst);
        let value = a.wrapping_add(1);
        self.write_opst16(dst, value);

        self.flags.z = value == 0;
        self.flags.n = false;
        self.flags.h = (a << 4).overflowing_add(1).1;
    }

    fn dec(&mut self, dst: &OpsTarget8) {
        let a: u8 = self.read_opst8(dst);
        let value = a.wrapping_sub(1);
        self.write_opst8(dst, value);

        self.flags.z = value == 0;
        self.flags.n = true;
        self.flags.h = (a << 4).overflowing_sub(1).1;
    }

    fn dec16(&mut self, dst: &OpsTarget16) {
        let a: u16 = self.read_opst16(dst);
        let value = a.wrapping_sub(1);
        self.write_opst16(dst, value);

        self.flags.z = value == 0;
        self.flags.n = true;
        self.flags.h = (a << 4).overflowing_sub(1).1;
    }

    fn rotate_left_a(&mut self, carry: &bool) {
        let a = self.regs.a;
        let (mut value, overflow) = a.overflowing_shl(1);
        let c = if *carry { self.flags.c } else { overflow };

        if c { value = value | 1 }

        self.flags.z = value == 0;
        self.flags.n = false;
        self.flags.h = false;
        self.flags.c = c;

        self.regs.a = value
    }

    fn rotate_right_a(&mut self, carry: &bool) {
        let a = self.regs.a;
        let (mut value, overflow) = a.overflowing_shr(1);
        let c = if *carry { self.flags.c } else { overflow };

        if c { value = value | 1 }

        self.flags.z = value == 0;
        self.flags.n = false;
        self.flags.h = false;
        self.flags.c = c;

        self.regs.a = value
    }

    fn load(&mut self, dst: &OpsTarget8, src: &OpsTarget8) {
        let value: u8 = self.read_opst8(src);
        self.write_opst8(dst, value);
    }

    fn load16(&mut self, dst: &OpsTarget16, src: &OpsTarget16) {
        let value: u16 = self.read_opst16(src);
        self.write_opst16(dst, value);
    }

    fn load_dst_addr(&mut self, dst: &OpsTarget8, src: &OpsTarget8) {
        let addr: u16 = 0xFF00 + self.read_opst8(dst) as u16;
        let value: u8 = self.read_opst8(src);
        self.write_addr8(addr, value);
    }

    fn load_src_addr(&mut self, dst: &OpsTarget8, src: &OpsTarget8) {
        let addr: u16 = 0xFF00 + self.read_opst8(src) as u16;
        let value: u8 = self.read_addr8(addr);
        self.write_opst8(dst, value);
    }

    fn load_sp_add_spaddr8_to_hl(&mut self) {
        // TODO i8
        let value: u16 = self.regs.sp + self.read_pcaddr8() as u16;
        self.regs.hl_w(value);
    }

    fn push(&mut self, reg16: &Register16) {
        // Todo simplify stack
        let value: u16 = self.regs.read_reg16(reg16);
        self.regs.sp -= 2;
        self.mem.write_addr16(self.regs.sp, value);
    }

    fn pop(&mut self, reg16: &Register16) {
        let value: u16 = self.mem.read_addr16(self.regs.sp);
        self.regs.sp += 2;
        self.regs.write_reg16(reg16, value)
    }

    fn jump(&mut self) {
        self.pc.value = self.read_pcaddr16()
    }

    fn jump_if(&mut self, flag: &Flag) {
        if self.flags.flag(flag) {
            self.jump();
        } else {
            self.pc.value += 2
        }
    }

    fn jump_hl(&mut self) {
        self.pc.value = self.regs.hl()
    }

    fn jump_reg(&mut self) {
        let b: i8 = self.read_pcaddr8() as i8;
        let a: u16 = self.pc.value;
        let value: u16 = (a as i32 + b as i32) as u16;
        self.pc.value = value;
    }

    fn jump_reg_if(&mut self, flag: &Flag) {
        if self.flags.flag(flag) {
            self.jump_reg()
        } else {
            self.pc.value += 1;
        }
    }

    fn call(&mut self) {
        // Push current pc value to stack
        self.regs.sp -= 2;
        self.mem.write_addr16(self.regs.sp, self.pc.value + 2);
        self.pc.value = self.read_pcaddr16();
    }

    fn call_if(&mut self, flag: &Flag) {
        if self.flags.flag(flag) {
            self.call()
        } else {
            self.pc.value += 2;
        }
    }

    fn ret(&mut self) {
        // Pop value at top of stack to sp
        self.pc.value = self.mem.read_addr16(self.regs.sp);
        self.regs.sp += 2
    }

    fn ret_if(&mut self, flag: &Flag) {
        // Pop value at top of stack to sp
        if self.flags.flag(flag) {
            self.ret()
        } else {
            // TODO increase pc?
        }
    }

    fn ret_interrupt(&mut self) {
        // Todo self.pc = pop
        // Enable interrupts
    }

    fn restart(&mut self, addr: &u16) {
        // Todo push
        self.pc.value = *addr;
    }

    // TODO

    fn set_carry_flag(&mut self) {
        self.flags.c = true;
    }

    fn complement_carry_flag(&mut self) {
        self.flags.c = !self.flags.c;
    }

    // TODO

    fn execute_bit_instruction(&mut self, instruction: &BitInstruction) {
        match instruction {
            BitInstruction::RotateLeft { .. } => {}
            BitInstruction::RotateRight { .. } => {}
            BitInstruction::ShiftLeftArithmetic { .. } => {}
            BitInstruction::ShiftRightArithmetic { .. } => {}
            BitInstruction::SwapNibbles { .. } => {}
            BitInstruction::ShiftRightLogical { .. } => {}
            BitInstruction::BitTest { src, bit } => self.bit_test(src, bit),
            BitInstruction::BitReset { .. } => {}
            BitInstruction::BitSet { dst, bit } => self.bit_set(dst, bit)
        }
    }

    fn bit_test(&mut self, dst: &OpsTarget8, bit: &u8) {
        let a = self.read_opst8(dst);
        let value = a & (1 << bit);

        self.flags.z = value == 0;
        self.flags.n = false;
        self.flags.h = true;
    }

    fn bit_set(&mut self, dst: &OpsTarget8, bit: &u8) {
        let a = self.read_opst8(dst);
        let value = a | (1 << bit);
        self.write_opst8(dst, value);
    }
}


trait MemoryOperations {
    fn read_opst8(&mut self, opstarget: &OpsTarget8) -> u8;
    fn write_opst8(&mut self, opstarget: &OpsTarget8, value: u8);
    fn read_opst16(&mut self, opstarget: &OpsTarget16) -> u16;
    fn write_opst16(&mut self, opstarget: &OpsTarget16, value: u16);

    fn read_addr8(&self, addr: u16) -> u8;
    fn write_addr8(&mut self, addr: u16, value: u8);

    fn read_pcaddr8(&mut self) -> u8;
    fn write_pcaddr8(&mut self, value: u8);
    fn read_pcaddr16(&mut self) -> u16;
    fn write_pcaddr16(&mut self, value: u16);
}

impl MemoryOperations for Cpu {
    fn read_opst8(&mut self, opst8: &OpsTarget8) -> u8 {
        match opst8 {
            OpsTarget8::R8(r8) => { self.regs.read_reg8(&r8) }
            OpsTarget8::R16Addr8(r16) => { self.mem.read_addr8(self.regs.read_reg16(r16)) }
            OpsTarget8::PcAddr8 => { self.read_pcaddr8() }
        }
    }

    fn write_opst8(&mut self, opst8: &OpsTarget8, value: u8) {
        match opst8 {
            OpsTarget8::R8(r8) => { self.regs.write_reg8(&r8, value); }
            OpsTarget8::R16Addr8(r16) => { self.mem.write_addr8(self.regs.read_reg16(r16), value); }
            OpsTarget8::PcAddr8 => { self.write_pcaddr8(value) }
        }
    }

    fn read_opst16(&mut self, opst16: &OpsTarget16) -> u16 {
        match opst16 {
            OpsTarget16::R16(r16) => { self.regs.read_reg16(&r16) }
            OpsTarget16::PC => { self.pc.value } // Todo increment?
            OpsTarget16::PcAddr16 => { self.read_pcaddr16() }
        }
    }

    fn write_opst16(&mut self, opst16: &OpsTarget16, value: u16) {
        match opst16 {
            OpsTarget16::R16(r16) => { self.regs.write_reg16(&r16, value); }
            OpsTarget16::PC => { self.pc.value = value } // Todo increment?
            OpsTarget16::PcAddr16 => { self.write_pcaddr16(value) }
        }
    }

    fn read_addr8(&self, addr: u16) -> u8 {
        self.mem.read_addr8(addr)
    }

    fn write_addr8(&mut self, addr: u16, value: u8) {
        self.mem.write_addr8(addr, value);
    }

    fn read_pcaddr8(&mut self) -> u8 {
        let v = self.mem.read_addr8(self.pc.value);
        self.pc.value += 1;
        v
    }

    fn write_pcaddr8(&mut self, value: u8) {
        self.mem.write_addr8(self.pc.value, value);
        self.pc.value += 1;
    }

    fn read_pcaddr16(&mut self) -> u16 {
        let v = self.mem.read_addr16(self.pc.value);
        self.pc.value += 2;
        v
    }

    fn write_pcaddr16(&mut self, value: u16) {
        self.mem.write_addr16(self.pc.value, value);
        self.pc.value += 2;
    }
}
