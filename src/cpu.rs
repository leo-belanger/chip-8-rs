use crate::display;
use crate::ram;

use std::{error::Error, fs};

const FONT_STARTING_ADDRESS: usize = 0x000;
const PROGRAM_STARTING_ADDRESS: usize = 0x200;

struct Instruction {
    high: u8,
    low: u8,
}

#[derive(Default)]
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
}

impl CPU {
    pub fn build() -> CPU {
        CPU {
            pc: PROGRAM_STARTING_ADDRESS as u16,
            ..Default::default()
        }
    }

    pub fn run(&mut self, file_path: &str) -> Result<(), Box<dyn Error>> {
        self.load_font_in_ram()?;
        self.load_program_in_ram(file_path)?;

        let bytes_from_ram = self.ram.read(0x000, 0xFFF)?;

        println!("Read {} bytes from RAM.", bytes_from_ram.len());
        println!("RAM: {:#04X?}", bytes_from_ram);

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

    fn read_instruction(&mut self) -> Result<Instruction, Box<dyn Error>> {
        let bytes = self.ram.read(self.pc.into(), 2)?;

        self.pc += 2;

        Ok(Instruction {
            high: bytes[0].clone(),
            low: bytes[1].clone(),
        })
    }

    fn main_loop(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            // TODO: fetch inputs

            let instruction = self.read_instruction()?;

            self.execute_instruction(&instruction)?;

            self.display.refresh();
        }

        Ok(())
    }

    fn execute_instruction(&mut self, instruction: &Instruction) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
