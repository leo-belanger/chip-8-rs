use crate::{
    display::{self, Position},
    keypad, ram, speaker,
};

use rand::prelude::*;

use sdl2::{event::Event, keyboard::Keycode, EventPump, Sdl};
use std::{
    error::Error,
    fs, thread,
    time::{Duration, Instant},
};

const FONT_STARTING_ADDRESS: usize = 0x000;
const PROGRAM_STARTING_ADDRESS: usize = 0x200;
const DEFAULT_INSTRUCTIONS_PER_FRAME: usize = 50; // This can vary a lot by programs, usually programs that are well designed should not care, but that's not always the case unfortunately
const FRAME_TIME_IN_MILLIS: u64 = 17; // 1000 (1 sec in millis) / 60 (fps) = 16.666

#[derive(Debug)]
struct Instruction {
    kk: u8,
    nnn: u16,
    x: usize,
    y: usize,
    nibbles: (u8, u8, u8, u8),
}

pub struct CPU<'a> {
    delay_timer: u8,
    display: display::Display,
    i: u16,
    keypad: keypad::Keypad,
    pc: u16,
    ram: ram::RAM,
    rng: ThreadRng,
    sdl_context: &'a Sdl,
    sound_timer: u8,
    sp: u8,
    speaker: speaker::Speaker,
    stack: [u16; 16],
    v: [u8; 16],
    instructions_per_frame: usize,
}

impl<'a> CPU<'a> {
    pub fn new(sdl_context: &'a Sdl) -> Result<CPU, Box<dyn Error>> {
        let display = display::Display::new(sdl_context)?;
        let keypad = keypad::Keypad::new();
        let ram = ram::RAM::new();
        let speaker = speaker::Speaker::new(sdl_context)?;

        let rng = rand::thread_rng();

        Ok(CPU {
            delay_timer: 0,
            display,
            i: 0,
            keypad,
            pc: PROGRAM_STARTING_ADDRESS as u16,
            ram,
            rng,
            sdl_context,
            sound_timer: 0,
            sp: 0,
            speaker,
            stack: [0; 16],
            v: [0; 16],
            instructions_per_frame: DEFAULT_INSTRUCTIONS_PER_FRAME,
        })
    }

    pub fn run(
        &mut self,
        program_path: &str,
        instructions_per_frame: Option<usize>,
    ) -> Result<(), Box<dyn Error>> {
        self.load_font_in_ram()?;
        self.load_program_in_ram(program_path)?;

        if let Some(i) = instructions_per_frame {
            self.instructions_per_frame = i;
        }

        self.main_loop()?;

        Ok(())
    }

    fn load_font_in_ram(&mut self) -> Result<(), Box<dyn Error>> {
        self.load_in_ram(FONT_STARTING_ADDRESS, &display::FONT_DATA)?;

        Ok(())
    }

    fn load_program_in_ram(&mut self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let bytes = fs::read(file_path)?;

        println!("Read {} bytes from {}.", bytes.len(), file_path);

        self.load_in_ram(PROGRAM_STARTING_ADDRESS, &bytes)?;

        Ok(())
    }

    fn load_in_ram(&mut self, address: usize, data: &[u8]) -> Result<(), Box<dyn Error>> {
        let bytes_loaded = self.ram.write(address, data)?;

        println!(
            "Loaded {} bytes in RAM at address {:#04X?}.",
            bytes_loaded, address
        );

        Ok(())
    }

    fn read_instruction(&mut self) -> Result<Instruction, String> {
        let bytes = self.ram.read(self.pc.into(), 2)?;

        if bytes.len() != 2 {
            return Err(format!(
                "Tried to read 2 bytes at address {:#04X?} but got {} byte(s). {:#04X?}",
                self.pc,
                bytes.len(),
                bytes
            ));
        }

        self.pc += 2;

        Ok(CPU::parse_instruction(bytes[0], bytes[1]))
    }

    fn parse_instruction(high: u8, low: u8) -> Instruction {
        let nnn: u16 = (low as u16) | (((high as u16) << 8) & 0x0FFF);

        let nibbles = (
            (high & 0xF0) >> 4,
            high & 0x0F,
            (low & 0xF0) >> 4,
            low & 0x0F,
        );

        Instruction {
            kk: low,
            nnn,
            x: nibbles.1 as usize,
            y: nibbles.2 as usize,
            nibbles,
        }
    }

    fn main_loop(&mut self) -> Result<(), Box<dyn Error>> {
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        'running: loop {
            let start_time = Instant::now();

            if self.process_events(&mut event_pump) {
                break 'running;
            }

            for _ in 0..self.instructions_per_frame {
                let instruction = self.read_instruction()?;
                self.execute_instruction(&instruction)?;
            }

            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }

            if self.sound_timer > 0 {
                self.sound_timer -= 1;
            }

            if self.sound_timer > 0 {
                self.speaker.start_beep();
            } else {
                self.speaker.stop_beep();
            }

            self.display.refresh()?;

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
        let mut should_quit = false;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => should_quit = true,
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

        return should_quit;
    }

    fn execute_instruction(&mut self, instruction: &Instruction) -> Result<(), Box<dyn Error>> {
        match instruction.nibbles {
            (0x0, 0x0, 0xE, 0x0) => self.inst_00e0(),
            (0x0, 0x0, 0xE, 0xE) => self.inst_00ee(),
            (0x1, _, _, _) => self.inst_1nnn(instruction),
            (0x2, _, _, _) => self.inst_2nnn(instruction),
            (0x3, _, _, _) => self.inst_3xkk(instruction),
            (0x4, _, _, _) => self.inst_4xkk(instruction),
            (0x5, _, _, 0x0) => self.inst_5xy0(instruction),
            (0x6, _, _, _) => self.inst_6xkk(instruction),
            (0x7, _, _, _) => self.inst_7xkk(instruction),
            (0x8, _, _, 0x0) => self.inst_8xy0(instruction),
            (0x8, _, _, 0x1) => self.inst_8xy1(instruction),
            (0x8, _, _, 0x2) => self.inst_8xy2(instruction),
            (0x8, _, _, 0x3) => self.inst_8xy3(instruction),
            (0x8, _, _, 0x4) => self.inst_8xy4(instruction),
            (0x8, _, _, 0x5) => self.inst_8xy5(instruction),
            (0x8, _, _, 0x6) => self.inst_8xy6(instruction),
            (0x8, _, _, 0x7) => self.inst_8xy7(instruction),
            (0x8, _, _, 0xE) => self.inst_8xye(instruction),
            (0x9, _, _, 0x0) => self.inst_9xy0(instruction),
            (0xA, _, _, _) => self.inst_annn(instruction),
            (0xB, _, _, _) => self.inst_bnnn(instruction),
            (0xC, _, _, _) => self.inst_cxkk(instruction),
            (0xD, _, _, _) => self.inst_dxyn(instruction)?,
            (0xE, _, 0x9, 0xE) => self.inst_ex9e(instruction)?,
            (0xE, _, 0xA, 0x1) => self.inst_exa1(instruction)?,
            (0xF, _, 0x0, 0x7) => self.inst_fx07(instruction),
            (0xF, _, 0x0, 0xA) => self.inst_fx0a(instruction)?,
            (0xF, _, 0x1, 0x5) => self.inst_fx15(instruction),
            (0xF, _, 0x1, 0x8) => self.inst_fx18(instruction),
            (0xF, _, 0x1, 0xE) => self.inst_fx1e(instruction),
            (0xF, _, 0x2, 0xE) => self.inst_fx2e(instruction),
            (0xF, _, 0x3, 0x3) => self.inst_fx33(instruction)?,
            (0xF, _, 0x5, 0x5) => self.inst_fx55(instruction)?,
            (0xF, _, 0x6, 0x5) => self.inst_fx65(instruction)?,
            _ => (),
        };

        Ok(())
    }

    fn inst_00e0(&mut self) {
        self.display.clear();
    }

    fn inst_00ee(&mut self) {
        self.pc = self.stack[self.sp as usize];
        self.sp -= 1;
    }

    fn inst_1nnn(&mut self, instruction: &Instruction) {
        self.pc = instruction.nnn;
    }

    fn inst_2nnn(&mut self, instruction: &Instruction) {
        self.sp += 1;
        self.stack[self.sp as usize] = self.pc;

        self.pc = instruction.nnn;
    }

    fn inst_3xkk(&mut self, instruction: &Instruction) {
        if self.v[instruction.x] == instruction.kk {
            self.pc += 2;
        }
    }

    fn inst_4xkk(&mut self, instruction: &Instruction) {
        if self.v[instruction.x] != instruction.kk {
            self.pc += 2;
        }
    }

    fn inst_5xy0(&mut self, instruction: &Instruction) {
        if self.v[instruction.x] == self.v[instruction.y] {
            self.pc += 2;
        }
    }

    fn inst_6xkk(&mut self, instruction: &Instruction) {
        self.v[instruction.x] = instruction.kk;
    }

    fn inst_7xkk(&mut self, instruction: &Instruction) {
        self.v[instruction.x] = self.v[instruction.x].wrapping_add(instruction.kk);
    }

    fn inst_8xy0(&mut self, instruction: &Instruction) {
        self.v[instruction.x] = self.v[instruction.y]
    }

    fn inst_8xy1(&mut self, instruction: &Instruction) {
        self.v[instruction.x] |= self.v[instruction.y]
    }

    fn inst_8xy2(&mut self, instruction: &Instruction) {
        self.v[instruction.x] &= self.v[instruction.y]
    }

    fn inst_8xy3(&mut self, instruction: &Instruction) {
        self.v[instruction.x] ^= self.v[instruction.y]
    }

    fn inst_8xy4(&mut self, instruction: &Instruction) {
        let result: u16 = self.v[instruction.x] as u16 + self.v[instruction.y] as u16;

        self.v[0xF] = if result > 255 { 1 } else { 0 };

        self.v[instruction.x] = result as u8;
    }

    fn inst_8xy5(&mut self, instruction: &Instruction) {
        self.v[0xF] = if self.v[instruction.x] > self.v[instruction.y] {
            1
        } else {
            0
        };

        self.v[instruction.x] = self.v[instruction.x].wrapping_sub(self.v[instruction.y]);
    }

    fn inst_8xy6(&mut self, instruction: &Instruction) {
        self.v[0xF] = if self.v[instruction.x] & 0x01 != 0 {
            1
        } else {
            0
        };

        self.v[instruction.x] = self.v[instruction.x].wrapping_div(2);
    }

    fn inst_8xy7(&mut self, instruction: &Instruction) {
        self.v[0xF] = if self.v[instruction.y] > self.v[instruction.x] {
            1
        } else {
            0
        };

        self.v[instruction.x] = self.v[instruction.y].wrapping_sub(self.v[instruction.x]);
    }

    fn inst_8xye(&mut self, instruction: &Instruction) {
        self.v[0xF] = if self.v[instruction.x] & 0x80 != 0 {
            1
        } else {
            0
        };

        self.v[instruction.x] = self.v[instruction.x].wrapping_mul(2);
    }

    fn inst_9xy0(&mut self, instruction: &Instruction) {
        if self.v[instruction.x] != self.v[instruction.y] {
            self.pc += 2;
        }
    }

    fn inst_annn(&mut self, instruction: &Instruction) {
        self.i = instruction.nnn;
    }

    fn inst_bnnn(&mut self, instruction: &Instruction) {
        self.pc = instruction.nnn + (self.v[0] as u16);
    }

    fn inst_cxkk(&mut self, instruction: &Instruction) {
        let random_byte: u8 = self.rng.gen_range(0..=255);

        self.v[instruction.x] = random_byte & instruction.kk;
    }

    fn inst_dxyn(&mut self, instruction: &Instruction) -> Result<(), String> {
        let sprite = self
            .ram
            .read(self.i as usize, instruction.nibbles.3 as usize)?;

        let position = Position::new(
            self.v[instruction.x] as usize,
            self.v[instruction.y] as usize,
        );

        let collided = self.display.draw_sprite(sprite, position)?;

        self.v[0xF] = if collided { 1 } else { 0 };

        Ok(())
    }

    fn inst_ex9e(&mut self, instruction: &Instruction) -> Result<(), String> {
        let key = self.v[instruction.x];

        if self.keypad.is_key_pressed(key)? {
            self.pc += 2;
        }

        Ok(())
    }

    fn inst_exa1(&mut self, instruction: &Instruction) -> Result<(), String> {
        let key = self.v[instruction.x];

        if !self.keypad.is_key_pressed(key)? {
            self.pc += 2;
        }

        Ok(())
    }

    fn inst_fx07(&mut self, instruction: &Instruction) {
        self.v[instruction.x] = self.delay_timer;
    }

    fn inst_fx0a(&mut self, instruction: &Instruction) -> Result<(), String> {
        for key in 0x0u8..=0xF {
            if self.keypad.is_key_pressed(key)? {
                self.v[instruction.x] = key;
                break; // Break out of loop if key is pressed so we move on to next instruction
            }
        }

        // Keep re-reading instruction until a key is press
        self.pc -= 2;

        Ok(())
    }

    fn inst_fx15(&mut self, instruction: &Instruction) {
        self.delay_timer = self.v[instruction.x]
    }

    fn inst_fx18(&mut self, instruction: &Instruction) {
        self.sound_timer = self.v[instruction.x]
    }

    fn inst_fx1e(&mut self, instruction: &Instruction) {
        self.i += self.v[instruction.x] as u16
    }

    fn inst_fx2e(&mut self, instruction: &Instruction) {
        self.i = self.v[instruction.x] as u16 * 5
    }

    fn inst_fx33(&mut self, instruction: &Instruction) -> Result<(), String> {
        // Decimal to BCD
        let vx = self.v[instruction.x];

        let hundreds = vx / 100;
        let tens = (vx - (hundreds * 100)) / 10;
        let ones = vx - (hundreds * 100) - (tens * 10);

        self.ram.write(self.i as usize, &[hundreds])?;
        self.ram.write((self.i as usize) + 1, &[tens])?;
        self.ram.write((self.i as usize) + 2, &[ones])?;

        Ok(())
    }

    fn inst_fx55(&mut self, instruction: &Instruction) -> Result<(), String> {
        let mut data: Vec<u8> = vec![];
        for register_index in 0x0..=instruction.x {
            data.push(self.v[register_index]);
        }

        self.ram.write(self.i as usize, &data)?;

        Ok(())
    }

    fn inst_fx65(&mut self, instruction: &Instruction) -> Result<(), String> {
        let bytes_to_read = instruction.x + 1;

        let bytes = self.ram.read(self.i as usize, bytes_to_read)?;

        if bytes.len() != bytes_to_read {
            return Err(format!(
                "Tried to read {} bytes at address {:#04X?} but got {} byte(s). {:#04X?}",
                bytes_to_read,
                self.i,
                bytes.len(),
                bytes
            ));
        }

        for (byte_index, byte) in bytes.iter().enumerate() {
            self.v[byte_index] = byte.clone();
        }

        Ok(())
    }
}
