// mod config;
use crate::chip8::config::*;

pub struct Registers{
    pub V : [u8; TOTAL_DATA_REGISTERS],
    pub I : u16,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub PC: u16,
    pub SP: u8,
}

impl Registers{
    pub fn new() -> Registers{
        Registers{
            V : [0; TOTAL_DATA_REGISTERS],
            I : 0,
            delay_timer: 0,
            sound_timer: 0,
            PC: 0,
            SP: 0,
        }
    }
}