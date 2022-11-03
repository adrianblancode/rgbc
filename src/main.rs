#![feature(bigint_helper_methods)]
extern crate core;

mod cpu;
mod registers;
mod memory;
mod instructions;
mod opcode_parser;
mod cycles;
mod flags;
mod bootrom;
mod frontend;

use std::{env};
use std::path::{Path};
use cpu::Cpu;
use bootrom::BootRom;
use crate::frontend::Frontend;
use crate::instructions::Opcode;
use crate::memory::Memory;

struct Emulator {
    frontend: Frontend,
    cpu: Cpu,
}

impl Emulator {
    fn new(bootrom: BootRom) -> Emulator {
        let mem = Memory::new(bootrom);
        let cpu = Cpu::new(mem);
        Emulator {
            frontend : Frontend::new(),
            cpu
        }
    }

    fn run(&mut self) {
        for x in 0 .. 99990000 {
            self.cpu.step();
            if x % 100 == 0 {
                self.frontend.step(&self.cpu.mem);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let boot_rom_arg: &String = &args.get(1).expect("First argument must contain boot rom path");
    let boot_rom = BootRom::new(Path::new(boot_rom_arg)).expect("Failed to read boot rom");

    let mut emulator = Emulator::new(boot_rom);
    emulator.run();
    println!("{:?}", emulator.cpu);
}
