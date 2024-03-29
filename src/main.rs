#![feature(bigint_helper_methods)]
extern crate core;

mod cpu;
mod gpu;
mod registers;
mod memory;
mod instructions;
mod opcode_parser;
mod cycles;
mod flags;
mod rom;
mod frontend;

use std::{env};
use std::path::{Path};
use cpu::Cpu;
use rom::Rom;
use crate::frontend::Frontend;
use crate::gpu::Gpu;
use crate::instructions::Opcode;
use crate::memory::Memory;

struct Emulator {
    frontend: Frontend,
    cpu: Cpu,
    gpu: Gpu
}

impl Emulator {
    fn new(bootrom: Rom) -> Emulator {
        let mem = Memory::new(bootrom);
        let cpu = Cpu::new(mem);
        Emulator {
            frontend : Frontend::new(),
            cpu,
            gpu : Gpu::new()
        }
    }

    fn run(&mut self) {
        loop {

            if !self.frontend.is_open() { return; }

            self.cpu.step();
            self.gpu.step(&self.cpu.mem);
            self.frontend.step(&self.gpu);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let boot_rom_arg: &String = &args.get(1).expect("First argument must contain boot rom path");
    let boot_rom = Rom::new(Path::new(boot_rom_arg)).expect("Failed to read boot rom");

    let mut emulator = Emulator::new(boot_rom);
    emulator.run();
    println!("{:?}", emulator.cpu);
}
