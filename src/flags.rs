#[derive(PartialEq, Debug)]
pub enum Flag {
    Z, NZ, C, NC
}

#[derive(Default, Debug)]
pub struct Flags {
    // Zero
    pub z: bool,
    // Negative
    pub n: bool,
    // Half carry
    pub h: bool,
    // Carry
    pub c: bool
}

impl Flags {

    pub fn new() -> Self {
        Default::default()
    }

    // Flags useful for branching
    pub fn flag(&self, flag: &Flag) -> bool {
        match flag {
            Flag::Z => self.z,
            Flag::NZ => !self.z,
            Flag::C => self.c,
            Flag::NC => !self.c
        }
    }
}