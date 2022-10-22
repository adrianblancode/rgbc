use std::fmt::{Debug, Formatter};
use crate::flags::Flag;
use crate::registers::*;

pub struct Opcode {
    pub value: u8,
}

impl Debug for Opcode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#04x}", self.value)
    }
}

#[derive(Debug)]
pub enum Instruction {
    Noop,
    Stop,
    Halt,

    AddA { src: OpsTarget8, carry: bool },
    AddHl { src: Register16 },
    AddSpAddr8ToSp,
    SubA { src: OpsTarget8, carry: bool },
    AndA { src: OpsTarget8 },
    XorA { src: OpsTarget8 },
    OrA { src: OpsTarget8 },
    CompareA { src: OpsTarget8 },
    Inc { dst: OpsTarget8 },
    Inc16 { dst: OpsTarget16 },
    Dec { dst: OpsTarget8 },
    Dec16 { dst: OpsTarget16 },
    RotateLeftA { carry: bool },
    RotateRightA { carry: bool },

    Load { dst: OpsTarget8, src: OpsTarget8 },
    Load16 { dst: OpsTarget16, src: OpsTarget16 },
    LoadDstAddr { dst: OpsTarget8, src: OpsTarget8 },
    LoadSrcAddr { dst: OpsTarget8, src: OpsTarget8 },
    LoadSpAddSpAddr8ToHl,

    Push { reg16: Register16 },
    Pop { reg16: Register16 },
    Jump,
    JumpIf { flag: Flag },
    JumpHL,
    JumpReg,
    JumpRegIf { flag: Flag },
    Call,
    CallIf { flag: Flag },
    Return,
    ReturnIf { flag: Flag },
    ReturnInterrupt,
    Restart { addr: u16 },

    DecimalAdjustA,
    ComplementA,
    SetCarryFlag,
    ComplementCarryFlag,
    SetInterrupts { enable: bool },

    NestedInstruction,
}

#[derive(Debug)]
pub enum BitInstruction {
    RotateLeft { dst: OpsTarget8, carry: bool },
    RotateRight { dst: OpsTarget8, carry: bool },
    ShiftLeftArithmetic { dst: OpsTarget8 },
    ShiftRightArithmetic { dst: OpsTarget8 },
    SwapNibbles { dst: OpsTarget8 },
    ShiftRightLogical { dst: OpsTarget8 },
    BitTest { src: OpsTarget8, bit: u8 },
    BitReset { dst: OpsTarget8, bit: u8 },
    BitSet { dst: OpsTarget8, bit: u8 },
}

#[derive(PartialEq, Debug)]
pub enum OpsTarget8 {
    R8(Register8),
    R16Addr8(Register16),
    PcAddr8,
}

#[derive(PartialEq, Debug)]
pub enum OpsTarget16 {
    R16(Register16),
    PC,
    PcAddr16,
}
