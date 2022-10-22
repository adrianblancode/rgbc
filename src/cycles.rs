use crate::flags::*;
use crate::instructions::Instruction;
use crate::instructions::Instruction::*;
use crate::Opcode;

impl Instruction {
    pub fn cycles(&self, opcode: &Opcode, flags: &Flags) -> u8 {
        let op = opcode.value;
        let op_mod8 = op % 8;
        let op_row = op - op_mod8;

        match self {
            Noop => 4,
            Stop => 4,
            Halt => 4,

            AddA { .. } | SubA { .. } | AndA { .. } | XorA { .. } | OrA { .. } | CompareA { .. } => if op_mod8 == 0x6 { 8 } else { 4 },
            AddHl { .. } => 8,
            AddSpAddr8ToSp => 16,
            Inc { .. } => if op == 0x34 { 12 } else { 4 },
            Inc16 { .. } => 8,
            Dec { .. } => if op == 0x35 { 12 } else { 4 },
            Dec16 { .. } => 8,
            RotateLeftA { .. } => 4,
            RotateRightA { .. } => 4,

            Load { .. } => {
                if (0x00 .. 0x3F).contains(&op) && op_mod8 == 0x02 { 8 }
                else if op_row == 0x70 || op_mod8 == 0x6 { 8 }
                else { 4 }
            }
            Load16 { .. } => 8,
            LoadDstAddr { .. } => if op_mod8 == 0x0 { 12 } else { 8 },
            LoadSrcAddr { .. } => if op_mod8 == 0x0 { 12 } else { 8 },
            LoadSpAddSpAddr8ToHl => 12,

            Push { .. } => 16,
            Pop { .. } => 12,
            Jump => 12,
            JumpHL => 4,
            JumpIf { flag} => if flags.flag(flag) { 16 } else { 12 },
            JumpReg => 12,
            JumpRegIf { flag} => if flags.flag(flag) { 12 } else { 8 },
            Call => 24,
            CallIf { flag } => if flags.flag(flag) { 24 } else { 12 },
            Return => 16,
            ReturnIf { flag } =>  if flags.flag(flag) { 20 } else { 8 },
            ReturnInterrupt => 16,
            Restart { .. } => 16,

            DecimalAdjustA => 4,
            ComplementA => 4,
            SetCarryFlag => 4,
            ComplementCarryFlag => 4,
            SetInterrupts { .. } => 4,

            NestedInstruction => 12
        }
    }
}
