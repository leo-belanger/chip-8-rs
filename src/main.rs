mod cpu;
mod display;
mod keypad;
mod ram;
mod speaker;

use clap::Parser;
use cpu::CPU;
use std::process;

#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(short = 'p', long = "program_path", default_value_t = String::from("programs/test_opcode.ch8"))]
    program_path: String,

    #[arg(short = 'i', long = "instructions_per_frame")]
    instructions_per_frame: Option<usize>,
}

fn main() {
    println!("Chip 8 emulator");

    let args = Args::parse();

    let mut cpu = match CPU::new() {
        Ok(new_cpu) => new_cpu,
        Err(e) => {
            eprintln!("Error while trying to create the CPU: {e}");
            process::exit(1)
        }
    };

    if let Err(e) = cpu.run(&args.program_path, args.instructions_per_frame) {
        eprintln!("Chip-8 error: {e}");
        process::exit(1)
    }
}
