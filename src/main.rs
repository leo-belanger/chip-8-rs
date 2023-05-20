mod chip8;
mod cli;

use std::process;

use crate::chip8::Emulator;

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

    let mut emulator = Emulator::new(args.instructions_per_frame, &sdl_context);

    if let Err(e) = emulator.run(&args.program_path) {
        eprintln!("Chip-8 error: {e}");
        process::exit(1)
    }
}
