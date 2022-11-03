use crate::ram;

use std::{error::Error, fs};

const PROGRAM_STARTING_ADDRESS: usize = 0x200;

#[derive(Default)]
pub struct CPU {
    delay_timer: u8,
    sound_timer: u8,
    i: u16,
    pc: u16,
    sp: u8,
    stack: [u16; 16],
    v: [u8; 16],
    ram: ram::RAM,
}

impl CPU {
    pub fn build() -> CPU {
        CPU {
            ..Default::default()
        }
    }

    pub fn run(&mut self, file_path: &str) -> Result<(), Box<dyn Error>> {
        self.load_in_ram(file_path)?;

        let bytes_from_ram = self.ram.read(0x000, 0xFFF)?;

        println!("Read {} bytes from RAM.", bytes_from_ram.len());
        println!("RAM: {:#04X?}", bytes_from_ram);

        Ok(())
    }

    fn load_in_ram(&mut self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let bytes = fs::read(file_path)?;

        println!("Read {} bytes from {}.", bytes.len(), file_path);

        let bytes_loaded = self.ram.write(PROGRAM_STARTING_ADDRESS, &bytes)?;

        println!(
            "Loaded {} bytes in RAM at address {}.",
            bytes_loaded, PROGRAM_STARTING_ADDRESS
        );

        Ok(())
    }
}