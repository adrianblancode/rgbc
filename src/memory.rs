use std::fmt::{Debug, Formatter};
use crate::BootRom;

pub struct Memory {
    pub data: [u8;0xffff]
}

impl Debug for Memory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        // let rows = self.data.len() / 16;
        let rows = 16;
        for row in 0 .. rows {
            res.push_str(format!("{:#06x}: ", row * 16).as_str());

            for byte in 0 .. 16 {
                res.push_str(format!("{:02x} ", self.data[row * 16 + byte]).as_str())
            }

            res.push_str("\n");
        }

        write!(f, "{}", res)
    }
}

impl Memory {

    pub fn new(bootrom: BootRom) -> Memory {
        let mut data: [u8;0xffff] = [0;0xffff];
        for i in 0..bootrom.data.len() {
            data[i] = bootrom.data[i]
        }
        Memory { data }
    }

    pub fn read_addr8(&self, addr: u16) -> u8 { self.data[addr as usize] }
    pub fn read_addr16(&self, addr: u16) -> u16 {
        self.data[addr as usize] as u16 | (self.data[(addr + 1) as usize] as u16) << 8
    }

    pub fn write_addr8(&mut self, addr: u16, value: u8) {
        self.data[addr as usize] = value
    }
    pub fn write_addr16(&mut self, addr: u16, value: u16){
        self.data[addr as usize] = value as u8;
        self.data[(addr + 1) as usize] = (value >> 8) as u8;
    }

}