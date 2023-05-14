mod chip8;
mod cli;

use chip8::CPU;
use std::process;

fn main() {
    println!("Chip 8 emulator");

    let args = cli::parse_args();

    let sdl_context = match sdl2::init() {
        Ok(sdl_context) => sdl_context,
        Err(e) => {
            eprintln!("Error while initializing SDL: {e}");
            process::exit(1);
        }
    };

    let mut cpu = match CPU::new(&sdl_context) {
        Ok(cpu) => cpu,
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
