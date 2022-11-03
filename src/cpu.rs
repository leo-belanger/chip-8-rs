use crate::ram;

use std::{error::Error, fs};

#[derive(Default)]
pub struct CPU {
    delay_timer: u8,
    sound_timer: u8,
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

    pub fn run(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let bytes = fs::read(file_path)?;

        println!("Read {} bytes from {}", bytes.len(), file_path);
        println!("Hexadecimal representation: {:#04X?}", bytes);

        Ok(())
    }
}
