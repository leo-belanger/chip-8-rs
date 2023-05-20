use std::{
    error::Error,
    thread,
    time::{Duration, Instant},
};

use sdl2::{event::Event, keyboard::Keycode, EventPump, Sdl};

use super::{cpu::CPU, devices};

const DEFAULT_INSTRUCTIONS_PER_FRAME: usize = 50; // This can vary a lot by programs, usually programs that are well designed should not care, but that's not always the case unfortunately
const FRAME_TIME_IN_MILLIS: u64 = 17; // 1000 (1 sec in millis) / 60 (fps) = 16.666

pub struct Emulator<'a> {
    cpu: CPU,
    instructions_per_frame: usize,
    keypad: devices::Keypad,
    sdl_context: &'a Sdl,
}

impl<'a> Emulator<'a> {
    pub fn new(instructions_per_frame: Option<usize>, sdl_context: &'a Sdl) -> Emulator<'a> {
        let instructions_per_frame =
            instructions_per_frame.unwrap_or(DEFAULT_INSTRUCTIONS_PER_FRAME);

        let keypad = devices::Keypad::new();

        Emulator {
            cpu: CPU::new(sdl_context).unwrap(),
            instructions_per_frame,
            keypad,
            sdl_context,
        }
    }

    pub fn run(&mut self, program_path: &str) -> Result<(), Box<dyn Error>> {
        self.cpu.load_font_in_ram()?;
        self.cpu.load_program_in_ram(program_path)?;

        let mut event_pump = self.sdl_context.event_pump().unwrap();

        'running: loop {
            let start_time = Instant::now();

            if self.process_events(&mut event_pump) {
                break 'running;
            }

            self.cpu.tick(&self.keypad, self.instructions_per_frame)?;

            let elapsed_time_in_millis = start_time.elapsed().as_millis();

            if FRAME_TIME_IN_MILLIS as u128 > elapsed_time_in_millis {
                thread::sleep(Duration::from_millis(
                    FRAME_TIME_IN_MILLIS - elapsed_time_in_millis as u64,
                ));
            }
        }

        Ok(())
    }

    fn process_events(&mut self, event_pump: &mut EventPump) -> bool {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return true,
                Event::KeyDown { keycode, .. } => {
                    if let Some(key) = keycode {
                        self.keypad.press_key(key);

                        match key {
                            Keycode::PageUp => self.instructions_per_frame += 1,
                            Keycode::PageDown if self.instructions_per_frame > 1 => {
                                self.instructions_per_frame -= 1
                            }
                            _ => (),
                        }
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(key) = keycode {
                        self.keypad.release_key(key);
                    }
                }
                _ => (),
            }
        }

        false
    }
}
