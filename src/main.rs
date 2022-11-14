mod cpu;
mod display;
mod keypad;
mod ram;
mod speaker;

use clap::{self, Parser};
use cpu::CPU;
use std::{env, process};

#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(default_value_t = String::from("programs/test_opcode.ch8"))]
    program_path: String,

    #[arg(default_value_t = 50)]
    max_instructions_per_frames: usize,
}

fn main() {
    println!("Chip 8 emulator");

    let args = Args::parse();

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut cpu = match CPU::new() {
        Ok(new_cpu) => new_cpu,
        Err(e) => {
            eprintln!("Error while trying to create the CPU: {e}");
            process::exit(1)
        }
    };

    if let Err(e) = cpu.run(file_path) {
        eprintln!("Chip-8 error: {e}");
        process::exit(1)
    }
}
