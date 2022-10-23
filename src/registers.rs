use std::fmt::{Debug, Formatter};

#[derive(PartialEq, Debug)]
pub enum Register8 {
    // General registers
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(PartialEq, Debug)]
pub enum Register16 {
    // Combined registers
    AF,
    BC,
    DE,
    HL,
    // HL inc / dec
    HLI,
    HLD,
    // Stack pointer
    SP,
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
    pub sp: u16,
}

impl Debug for Registers {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "a: {:#04x}, f: {:#04x}, b: {:#04x}, c: {:#04x}, d: {:#04x}, e: {:#04x}, h: {:#04x}, l: {:#04x}, sp: {:#06x}", self.a, self.f, self.b, self.c, self.d, self.e, self.h, self.l, self.sp)
    }
}

impl Registers {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn af(&self) -> u16 {
        combine_u8(self.a, self.f)
    }

    pub fn af_w(&mut self, value: u16) {
        self.a = high_byte(value);
        self.f = low_byte(value);
    }

    pub fn bc(&self) -> u16 {
        combine_u8(self.b, self.c)
    }

    pub fn bc_w(&mut self, value: u16) {
        self.b = high_byte(value);
        self.c = low_byte(value);
    }

    pub fn de(&self) -> u16 {
        combine_u8(self.d, self.e)
    }

    pub fn de_w(&mut self, value: u16) {
        self.d = high_byte(value);
        self.e = low_byte(value);
    }

    pub fn hl(&self) -> u16 {
        combine_u8(self.h, self.l)
    }

    pub fn hl_w(&mut self, value: u16) {
        self.h = high_byte(value);
        self.l = low_byte(value);
    }

    pub fn read_reg8(&self, reg8: &Register8) -> u8 {
        match reg8 {
            Register8::A => self.a,
            Register8::F => self.f,
            Register8::B => self.b,
            Register8::C => self.c,
            Register8::D => self.d,
            Register8::E => self.e,
            Register8::H => self.h,
            Register8::L => self.l,
        }
    }

    pub fn read_reg16(&mut self, reg16: &Register16) -> u16 {
        match reg16 {
            Register16::AF => self.af(),
            Register16::BC => self.bc(),
            Register16::DE => self.de(),
            Register16::HL => self.hl(),
            Register16::HLI => {
                let value = self.hl();
                self.hl_w(value.wrapping_add(1));
                value
            }
            Register16::HLD => {
                let value = self.hl();
                self.hl_w(value.wrapping_sub(1));
                value
            }
            Register16::SP => self.sp,
        }
    }

    pub fn write_reg8(&mut self, reg8: &Register8, value: u8) {
        match reg8 {
            Register8::A => self.a = value,
            Register8::F => self.f = value,
            Register8::B => self.b = value,
            Register8::C => self.c = value,
            Register8::D => self.d = value,
            Register8::E => self.e = value,
            Register8::H => self.h = value,
            Register8::L => self.l = value,
        }
    }

    pub fn write_reg16(&mut self, reg16: &Register16, value: u16) {
        match reg16 {
            Register16::AF => { self.af_w(value); }
            Register16::BC => { self.bc_w(value); }
            Register16::DE => { self.de_w(value); }
            Register16::HL => { self.hl_w(value); }
            Register16::HLI => { self.hl_w(value.wrapping_add(1)); }
            Register16::HLD => { self.hl_w(value.wrapping_sub(1)); }
            Register16::SP => self.sp = value,
        }
    }
}

fn combine_u8(a: u8, b: u8) -> u16 {
    (a as u16) << 8 | (b as u16)
}

fn low_byte(value: u16) -> u8 {
    value as u8
}

fn high_byte(value: u16) -> u8 {
    (value >> 8) as u8
}
