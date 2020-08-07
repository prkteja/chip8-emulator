// mod config;
use crate::chip8::config::*;

pub struct Stack{
    pub stack: [u16; STACK_DEPTH]
}

impl Stack{
    pub fn new() -> Stack{
        Stack{
            stack: [0; STACK_DEPTH]
        }
    }
    pub fn stack_in_bounds(&self, sp: u8){
        assert!((sp as usize) < self.stack.len(), "Stack overflow");
    }
}