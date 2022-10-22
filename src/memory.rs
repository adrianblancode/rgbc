use crate::BootRom;

#[derive(Debug)]
pub struct Memory {
    data: [u8;0xffff]
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