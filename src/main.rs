mod cpu;
mod display;
mod keypad;
mod ram;
mod speaker;

use cpu::CPU;
use std::{env, process};

fn main() {
    println!("Chip 8 emulator");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // TODO: better error handling than just panicking
    let mut cpu = CPU::new().unwrap();

    if let Err(e) = cpu.run(file_path) {
        eprintln!("Chip-8 error: {e}");
        process::exit(1)
    }
}
