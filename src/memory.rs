use std::fmt::{Debug, Formatter};
use crate::Rom;

pub struct Memory {
    pub data: [u8; 0xffff + 1],
}

impl Debug for Memory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        let addr = 0x8000;
        let row_length = 32;
        for i in 0..8 {
            res.push_str(format!("{:#06x}: ", addr + i * row_length).as_str());

            for byte in 0..row_length {
                res.push_str(format!("{:02x} ", &self.data[addr + i * row_length + byte]).as_str())
            }

            res.push_str("\n");
        }

        write!(f, "{}", res)
    }
}

impl Memory {
    pub fn new(boot_rom: Rom, rom: Option<Rom>) -> Memory {
        let mut data: [u8; 0xffff + 1] = [0; 0xffff + 1];
        if let Some(r) = rom {
            println!("Loading rom ${:}", r.data.len());
            data[0..r.data.len()].copy_from_slice(r.data.as_slice());
        }
        data[0..0xFF].copy_from_slice(&boot_rom.data[0..0xFF]);
        Memory { data }
    }

    pub fn read_addr8(&self, addr: u16) -> u8 { self.data[addr as usize] }
    pub fn read_addr16(&self, addr: u16) -> u16 {
        self.data[addr as usize] as u16 | (self.data[(addr + 1) as usize] as u16) << 8
    }

    pub fn write_addr8(&mut self, addr: u16, value: u8) {
        self.data[addr as usize] = value
    }
    pub fn write_addr16(&mut self, addr: u16, value: u16) {
        self.data[addr as usize] = value as u8;
        self.data[(addr + 1) as usize] = (value >> 8) as u8;
    }
}