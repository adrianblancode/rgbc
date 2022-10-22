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

use std::{env, path};
use std::path::{Path, PathBuf};
use cpu::Cpu;
use bootrom::BootRom;
use crate::instructions::Opcode;
use crate::memory::Memory;

struct Emulator {
    cpu: Cpu,
}

impl Emulator {
    fn new(bootrom: BootRom) -> Emulator {
        let mem = Memory::new(bootrom);
        Emulator { cpu: Cpu::new(mem) }
    }

    fn run(&mut self) {
        loop {
            self.cpu.step()
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let boot_rom_arg: &String = &args.get(1).expect("First argument must contain boot rom path");
    let boot_rom = BootRom::new(Path::new(boot_rom_arg)).expect("Failed to read boot rom");

    let mut emulator = Emulator::new(boot_rom);

    emulator.run();
    // println!("Cpu: {0:?}", emulator.cpu);
}
