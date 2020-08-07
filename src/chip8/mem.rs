// mod config;
use crate::chip8::config::*;
// use crate::chip8::mem::*;
pub struct Memory{
    pub bytes: [u8; MEMORY_SIZE]
}

impl Memory{
    pub fn new() -> Memory{
        Memory{
            bytes: [0; MEMORY_SIZE]
        }
    }
    pub fn mem_in_bounds(index: usize){
        assert!(index < MEMORY_SIZE, "memory out of bounds\n");
    }
    pub fn set_mem(&mut self, index: usize, val: u8){
        Memory::mem_in_bounds(index);
        self.bytes[index] = val;
    }
    pub fn get_mem(&self, index: usize) -> u8 {
        Memory::mem_in_bounds(index);
        return self.bytes[index];
    }
    pub fn get_mem_16(&self, index: usize) -> u16 {
        let byte1 = self.get_mem(index) as u16;
        let byte2 = self.get_mem(index+1) as u16;
        return byte1 << 8 | byte2;
    }
}