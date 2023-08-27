use std::ops::{RangeInclusive};
use crate::flags::Flag;
use crate::instructions::*;
use crate::instructions::Instruction::*;
use crate::instructions::BitInstruction;
use crate::instructions::BitInstruction::*;
use crate::instructions::OpsTarget8::*;
use crate::instructions::OpsTarget16::*;
use crate::registers::*;
use crate::registers::Register8::*;
use crate::registers::Register16::*;

impl OpsTarget8 {
    pub fn r8(&self) -> Option<&Register8> {
        match self {
            R8(r8) => { Some(r8) }
            _ => { None }
        }
    }

    pub fn r16addr(&self) -> Option<&Register16> {
        match self {
            R16Addr8(r16addr) => { Some(r16addr) }
            _ => { None }
        }
    }
}

pub trait InstructionParser {
    fn to_instruction(&self) -> Instruction;
    fn to_bit_instruction(&self) -> BitInstruction;
}

// Matching opcodes from https://izik1.github.io/gbops/
impl InstructionParser for Opcode {
    fn to_instruction(&self) -> Instruction {
        let opcode = self;
        let op_mod8: u8 = opcode.value % 8;

        match opcode.value {
            // Column 1
            0x00 => Noop,
            0x08 => Load16 { dst: PcAddr16, src: SP.to_opst16() },
            0x10 => Stop,
            0x18 => JumpReg,
            0x20 => JumpRegIf { flag: Flag::NZ },
            0x28 => JumpRegIf { flag: Flag::Z },
            0x30 => JumpRegIf { flag: Flag::NC },
            0x38 => JumpRegIf { flag: Flag::C },
            0x01 => Load16 { dst: BC.to_opst16(), src: PcAddr16 },
            0x11 => Load16 { dst: DE.to_opst16(), src: PcAddr16 },
            0x21 => Load16 { dst: HL.to_opst16(), src: PcAddr16 },
            0x31 => Load16 { dst: SP.to_opst16(), src: PcAddr16 },
            0x09 => AddHl { src: BC },
            0x19 => AddHl { src: DE },
            0x29 => AddHl { src: HL },
            0x39 => AddHl { src: SP },
            0x02 => Load { dst: BC.to_opst8_addr(), src: A.to_opst8() },
            0x0A => Load { dst: A.to_opst8(), src: BC.to_opst8_addr() },
            0x12 => Load { dst: DE.to_opst8_addr(), src: A.to_opst8() },
            0x1A => Load { dst: A.to_opst8(), src: DE.to_opst8_addr() },
            0x22 => Load { dst: HLI.to_opst8_addr(), src: A.to_opst8() },
            0x2A => Load { dst: A.to_opst8(), src: HLI.to_opst8_addr() },
            0x32 => Load { dst: HLD.to_opst8_addr(), src: A.to_opst8() },
            0x3A => Load { dst: A.to_opst8(), src: HLD.to_opst8_addr() },
            0x03 => Inc16 { dst: BC.to_opst16() },
            0x0B => Dec16 { dst: BC.to_opst16() },
            0x13 => Inc16 { dst: DE.to_opst16() },
            0x1B => Dec16 { dst: DE.to_opst16() },
            0x23 => Inc16 { dst: HL.to_opst16() },
            0x2B => Dec16 { dst: HL.to_opst16() },
            0x33 => Inc16 { dst: SP.to_opst16() },
            0x3B => Dec16 { dst: SP.to_opst16() },
            _ if opcode.in_column(0x00..=0x3F, 0x04) => {
                Inc { dst: opcode.high_opst8() }
            }
            _ if opcode.in_column(0x00..=0x3F, 0x05) => {
                Dec { dst: opcode.high_opst8() }
            }
            _ if opcode.in_column(0x00..=0x3F, 0x06) => {
                Load { dst: opcode.high_opst8(), src: PcAddr8 }
            }
            0x07 => RotateLeftA { carry: true },
            0x0F => RotateRightA { carry: true },
            0x17 => RotateLeftA { carry: false },
            0x1F => RotateRightA { carry: false },
            0x27 => DecimalAdjustA,
            0x2F => ComplementA,
            0x37 => SetCarryFlag,
            0x3F => ComplementCarryFlag,

            // Column 2
            0x76 => Halt,
            0x40..=0x7F => {
                let src: OpsTarget8 = opcode.low_opst8();
                let dst: OpsTarget8 = opcode.high_opst8();
                if src.eq(&dst) { Noop } else { Load { dst, src } }
            }

            // Column 3
            0x80..=0x87 => AddA { src: opcode.low_opst8(), carry: false },
            0x88..=0x8F => AddA { src: opcode.low_opst8(), carry: true },
            0x90..=0x97 => SubA { src: opcode.low_opst8(), carry: false },
            0x98..=0x9F => SubA { src: opcode.low_opst8(), carry: true },
            0xA0..=0xA7 => AndA { src: opcode.low_opst8() },
            0xA8..=0xAF => XorA { src: opcode.low_opst8() },
            0xB0..=0xB7 => OrA { src: opcode.low_opst8() },
            0xB8..=0xBF => CompareA { src: opcode.low_opst8() },

            // Column 4
            0xC0 => ReturnIf { flag: Flag::NZ },
            0xC8 => ReturnIf { flag: Flag::Z },
            0xD0 => ReturnIf { flag: Flag::NC },
            0xD8 => ReturnIf { flag: Flag::C },
            0xE0 => LoadDstAddr { dst: PcAddr8, src: A.to_opst8() },
            0xE8 => AddSpAddr8ToSp,
            0xF0 => LoadSrcAddr { dst: A.to_opst8(), src: PcAddr8 },
            0xF8 => LoadSpAddSpAddr8ToHl,

            0xC1 => Pop { reg16: BC },
            0xC9 => Return,
            0xD1 => Pop { reg16: DE },
            0xD9 => ReturnInterrupt,
            0xE1 => Pop { reg16: HL },
            0xE9 => JumpHL,
            0xF1 => Pop { reg16: AF },
            0xF9 => Load16 { dst: PC, src: HL.to_opst16() },

            0xC2 => JumpIf { flag: Flag::NZ },
            0xCA => JumpIf { flag: Flag::Z },
            0xD2 => JumpIf { flag: Flag::NC },
            0xDA => JumpIf { flag: Flag::C },
            0xE2 => LoadDstAddr { dst: C.to_opst8(), src: A.to_opst8() },
            0xEA => Load { dst: PcAddr8, src: A.to_opst8() },
            0xF2 => LoadSrcAddr { dst: A.to_opst8(), src: C.to_opst8() },
            0xFA => Load { dst: A.to_opst8(), src: PcAddr8 },

            0xC3 => Jump,
            0xCB => NestedInstruction,
            0xF3 => SetInterrupts { enable: false },
            0xFB => SetInterrupts { enable: true },

            0xC4 => CallIf { flag: Flag::NZ },
            0xCC => CallIf { flag: Flag::Z },
            0xD4 => CallIf { flag: Flag::NC },
            0xDC => CallIf { flag: Flag::C },

            0xC5 => Push { reg16: BC },
            0xCD => Call,
            0xD5 => Push { reg16: DE },
            0xE5 => Push { reg16: HL },
            0xF5 => Push { reg16: AF },

            0xC6 => AddA { src: PcAddr8, carry: false },
            0xCE => AddA { src: PcAddr8, carry: true },
            0xD6 => SubA { src: PcAddr8, carry: false },
            0xDE => SubA { src: PcAddr8, carry: true },
            0xE6 => AndA { src: PcAddr8 },
            0xEE => XorA { src: PcAddr8 },
            0xF6 => OrA { src: PcAddr8 },
            0xFE => CompareA { src: PcAddr8 },

            op if (0xC0..=0xFF).contains(&op) && op_mod8 == 0x07 => {
                let op_base: u8 = (opcode.value / 8) % 8;
                Restart { addr: (op_base * 8) as u16 }
            }
            _ => panic!("Opcode {opcode:?} not parsed")
        }
    }

    fn to_bit_instruction(&self) -> BitInstruction {
        let opcode = self;
        let target: OpsTarget8 = opcode.low_opst8();
        match opcode.value {
            // TODO carry flag?
            0x00..=0x07 => RotateLeft { dst: target, carry: true },
            0x08..=0x0F => RotateRight { dst: target, carry: true },
            0x10..=0x17 => RotateLeft { dst: target, carry: false },
            0x18..=0x1F => RotateRight { dst: target, carry: false },
            0x20..=0x27 => ShiftLeftArithmetic { dst: target },
            0x28..=0x2F => ShiftRightArithmetic { dst: target },
            0x30..=0x37 => SwapNibbles { dst: target },
            0x38..=0x3F => ShiftRightLogical { dst: target },
            0x40..=0x7F => BitTest { src: target, bit: (opcode.value / 8) % 8 },
            0x80..=0xBF => BitReset { dst: target, bit: (opcode.value / 8) % 8 },
            0xC0..=0xFF => BitSet { dst: target, bit: (opcode.value / 8) % 8 },
            _ => panic!("Bit opcode {opcode:?} not parsed")
        }
    }
}

impl Register8 {
    fn to_opst8(self) -> OpsTarget8 { R8(self) }
}

impl Register16 {
    fn to_opst8_addr(self) -> OpsTarget8 {
        R16Addr8(self)
    }
    fn to_opst16(self) -> OpsTarget16 {
        R16(self)
    }
}

impl Opcode {

    // Whether the opcode is in an 8 opcode column, see instruction reference
    fn in_column(&self, range: RangeInclusive<u32>, mod8: u8) -> bool {
        range.contains(&(self.value as u32)) && self.value % 8 == mod8
    }

    // Function that can retrieve opstarget from opcode (for some opcodes)
    fn low_opst8(&self) -> OpsTarget8 {
        let bytes = self.value % 8;
        self.to_opst8_impl(bytes)
    }

    fn high_opst8(&self) -> OpsTarget8 {
        let bytes = (self.value / 8) % 8;
        self.to_opst8_impl(bytes)
    }

    fn to_opst8_impl(&self, bytes: u8) -> OpsTarget8 {
        match bytes {
            0x0 => B.to_opst8(),
            0x1 => C.to_opst8(),
            0x2 => D.to_opst8(),
            0x3 => E.to_opst8(),
            0x4 => H.to_opst8(),
            0x5 => L.to_opst8(),
            0x6 => HL.to_opst8_addr(),
            0x7 => A.to_opst8(),
            _ => panic!("Input must be in range 0x0 ..= 0x7")
        }
    }
}
