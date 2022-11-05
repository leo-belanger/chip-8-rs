use crate::{
    display::{self, Position},
    ram,
};

use sdl2::{event::Event, keyboard::Keycode, Sdl};
use std::{error::Error, fs};

const FONT_STARTING_ADDRESS: usize = 0x000;
const PROGRAM_STARTING_ADDRESS: usize = 0x200;

struct Instruction {
    kk: u8,
    nnn: u16,
    x: usize,
    y: usize,
    nibbles: (u8, u8, u8, u8),
}

pub struct CPU {
    delay_timer: u8,
    sound_timer: u8,
    i: u16,
    pc: u16,
    sp: u8,
    stack: [u16; 16],
    v: [u8; 16],
    display: display::Display,
    ram: ram::RAM,
    sdl_context: Sdl,
}

impl CPU {
    pub fn new() -> CPU {
        let sdl_context = sdl2::init().unwrap();

        let display = display::Display::new(&sdl_context);

        let ram = ram::RAM::new();

        CPU {
            delay_timer: 0,
            sound_timer: 0,
            i: 0,
            pc: PROGRAM_STARTING_ADDRESS as u16,
            sp: 0,
            stack: [0; 16],
            v: [0; 16],
            display,
            ram,
            sdl_context,
        }
    }

    pub fn run(&mut self, file_path: &str) -> Result<(), Box<dyn Error>> {
        self.load_font_in_ram()?;
        self.load_program_in_ram(file_path)?;

        let bytes_from_ram = self.ram.read(0x000, 0xFFF)?;

        println!("Read {} bytes from RAM.", bytes_from_ram.len());
        println!("RAM: {:#04X?}", bytes_from_ram);

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
                "Tried to read 2 bytes at address {:#04X?} but got {} byte(s).",
                self.pc,
                bytes.len()
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
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            let instruction = self.read_instruction()?;

            self.execute_instruction(&instruction)?;
        }

        Ok(())
    }

    fn execute_instruction(&mut self, instruction: &Instruction) -> Result<(), Box<dyn Error>> {
        match instruction.nibbles.0 {
            0x0 => self.execute_00xx_instruction(instruction),
            0x1 => self.execute_1nnn_instruction(instruction),
            0x2 => self.execute_2nnn_instruction(instruction),
            0x3 => self.execute_3xkk_instruction(instruction),
            0x4 => self.execute_4xkk_instruction(instruction),
            0x5 => self.execute_5xy0_instruction(instruction),
            0x6 => self.execute_6xkk_instruction(instruction),
            0x7 => self.execute_7xkk_instruction(instruction),
            0x8 => self.execute_8xyz_instruction(instruction),
            0x9 => self.execute_9xy0_instruction(instruction),
            0xA => self.execute_annn_instruction(instruction),
            0xB => self.execute_bnnn_instruction(instruction),
            0xC => self.execute_cxkk_instruction(instruction),
            0xD => self.execute_dxyn_instruction(instruction)?,
            0xE => self.execute_exxx_instruction(instruction),
            0xF => self.execute_fxxx_instruction(instruction),
            _ => (),
        };

        Ok(())
    }

    fn execute_00xx_instruction(&mut self, instruction: &Instruction) {
        match instruction.kk {
            0xE0 => self.display.clear(),
            0xEE => {
                self.pc = self.stack[self.sp as usize];
                self.sp -= 1;
            }
            _ => (),
        };
    }

    fn execute_1nnn_instruction(&mut self, instruction: &Instruction) {
        self.pc = instruction.nnn;
    }

    fn execute_2nnn_instruction(&mut self, instruction: &Instruction) {
        self.sp += 1;
        self.stack[self.sp as usize] = self.pc;

        self.pc = instruction.nnn;
    }

    fn execute_3xkk_instruction(&mut self, instruction: &Instruction) {
        if self.v[instruction.x] == instruction.kk {
            self.pc += 2;
        }
    }

    fn execute_4xkk_instruction(&mut self, instruction: &Instruction) {
        if self.v[instruction.x] != instruction.kk {
            self.pc += 2;
        }
    }

    fn execute_5xy0_instruction(&mut self, instruction: &Instruction) {
        if self.v[instruction.x] == self.v[instruction.y] {
            self.pc += 2;
        }
    }

    fn execute_6xkk_instruction(&mut self, instruction: &Instruction) {
        self.v[instruction.x] = instruction.kk;
    }

    fn execute_7xkk_instruction(&mut self, instruction: &Instruction) {
        self.v[instruction.x] += instruction.kk;
    }

    fn execute_8xyz_instruction(&mut self, instruction: &Instruction) {
        match instruction.nibbles.3 {
            0x0 => self.v[instruction.x] = self.v[instruction.y],
            0x1 => self.v[instruction.x] |= self.v[instruction.y],
            0x2 => self.v[instruction.x] &= self.v[instruction.y],
            0x3 => self.v[instruction.x] ^= self.v[instruction.y],
            0x4 => (), /* TODO */
            0x5 => (), /* TODO */
            0x6 => (), /* TODO */
            0x7 => (), /* TODO */
            0xE => (), /* TODO */
            _ => (),
        }
    }

    fn execute_9xy0_instruction(&mut self, instruction: &Instruction) {
        if self.v[instruction.x] != self.v[instruction.y] {
            self.pc += 2;
        }
    }

    fn execute_annn_instruction(&mut self, instruction: &Instruction) {
        self.i = instruction.nnn;
    }

    fn execute_bnnn_instruction(&mut self, instruction: &Instruction) {
        self.pc = instruction.nnn + (self.v[0] as u16);
    }

    fn execute_cxkk_instruction(&mut self, instruction: &Instruction) {}

    fn execute_dxyn_instruction(&mut self, instruction: &Instruction) -> Result<(), String> {
        let sprite = self
            .ram
            .read(self.i as usize, instruction.nibbles.3 as usize)?;

        let position = Position::new(
            self.v[instruction.x] as usize,
            self.v[instruction.y] as usize,
        );

        self.display.draw_sprite(sprite, position)?;

        // TODO: collision check, draw sprite should return bool for collision

        Ok(())
    }

    fn execute_exxx_instruction(&mut self, instruction: &Instruction) {}

    fn execute_fxxx_instruction(&mut self, instruction: &Instruction) {}
}
