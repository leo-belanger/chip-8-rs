use clap::{self, Parser};

#[derive(clap::Parser, Debug)]
pub struct Args {
    #[arg(short = 'p', long = "program_path", default_value_t = String::from("programs/test_opcode.ch8"))]
    pub program_path: String,

    #[arg(short = 'i', long = "instructions_per_frame")]
    pub instructions_per_frame: Option<usize>,
}

pub fn parse_args() -> Args {
    Args::parse()
}
