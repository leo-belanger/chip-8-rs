mod cpu;
mod display;
mod keypad;
mod ram;

use cpu::CPU;
use std::{env, process};

fn main() {
    println!("Chip 8 emulator");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut cpu = CPU::new();

    if let Err(e) = cpu.run(file_path) {
        eprintln!("Chip-8 error: {e}");
        process::exit(1)
    }
}
