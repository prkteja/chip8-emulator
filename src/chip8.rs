pub mod config;
mod display;
mod stack;
mod mem;
mod reg;
mod keyboard;
extern crate sdl2;
extern crate rand;
use crate::chip8::config::*;
use crate::chip8::display::*;
use crate::chip8::stack::*;
use crate::chip8::mem::*;
use crate::chip8::reg::*;
use crate::chip8::keyboard::*;
use crate::chip8::sdl2::keyboard::Keycode;
use rand::random;
use std::process;
use sdl2::event::Event;

const DEFAULT_CHAR_SET: [u8; 80] = [
    0xf0, 0x90, 0x90, 0x90, 0xf0,
    0x20, 0x60, 0x20, 0x20, 0x70,
    0xf0, 0x10, 0xf0, 0x80, 0xf0,
    0xf0, 0x10, 0xf0, 0x10, 0xf0,
    0x90, 0x90, 0xf0, 0x10, 0x10,
    0xf0, 0x80, 0xf0, 0x10, 0xf0,
    0xf0, 0x80, 0xf0, 0x90, 0xf0,
    0xf0, 0x10, 0x20, 0x40, 0x40,
    0xf0, 0x90, 0xf0, 0x90, 0xf0,
    0xf0, 0x90, 0xf0, 0x10, 0xf0,
    0xf0, 0x90, 0xf0, 0x90, 0x90,
    0xe0, 0x90, 0xe0, 0x90, 0xe0,
    0xf0, 0x80, 0x80, 0x80, 0xf0,
    0xe0, 0x90, 0x90, 0x90, 0xe0,
    0xf0, 0x80, 0xf0, 0x80, 0xf0, 
    0xf0, 0x80, 0xf0, 0x80, 0x80
];

const KEYBOARD_MAP: [Keycode; TOTAL_KEYS] = [
    Keycode::Num0, Keycode::Num1, Keycode::Num2, Keycode::Num3, Keycode::Num4, Keycode::Num5,
    Keycode::Num6, Keycode::Num7, Keycode::Num8, Keycode::Num9, Keycode::A, Keycode::B,
    Keycode::C, Keycode::D, Keycode::E, Keycode::F];

pub struct Chip8{
    pub memory: Memory,
    stack: Stack,
    pub registers: Registers,
    pub keyboard: Keyboard,
    pub display: Display
}

impl Chip8{
    pub fn new()->Chip8{
        Chip8{
            memory: Memory::new(),
            stack: Stack::new(),
            registers: Registers::new(),
            keyboard: Keyboard::new(KEYBOARD_MAP),
            display: Display::new()
        }
    }
    pub fn chip8_init(&mut self){
        for i in 0..DEFAULT_CHAR_SET.len(){
            self.memory.bytes[i] = DEFAULT_CHAR_SET[i];
        }
    }
    pub fn chip8_load(&mut self, buf: Vec<u8>){
        assert!(buf.len()+PROGRAM_LOAD_ADDR < MEMORY_SIZE, "Program too large\n");
        for i in 0..buf.len(){
            self.memory.bytes[PROGRAM_LOAD_ADDR+i] = buf[i];
        };
        self.registers.PC = PROGRAM_LOAD_ADDR as u16;
    }
    pub fn stack_push(&mut self, val: u16){
        self.registers.SP += 1;
        self.stack.stack_in_bounds(self.registers.SP);
        self.stack.stack[self.registers.SP as usize] = val;
    }
    pub fn stack_pop(&mut self) -> u16{
        self.stack.stack_in_bounds(self.registers.SP);
        let top = self.stack.stack[self.registers.SP as usize];
        self.registers.SP -= 1;
        return top;
    }
    pub fn wait_for_input(&self, mut event_pump: sdl2::EventPump)->u8{
        // let mut event_pump = sdl_context.event_pump().unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => break 'running,
                    Event::KeyDown {keycode: Some(k), ..} => {
                        let key = self.keyboard.get_key_map(k);
                        if key != -1{
                            return key as u8;
                        }
                    },
                    _ => {}
                }
            }
        };
        process::exit(0);
    }
    pub fn chip8_exec(&mut self, opcode: u16, mut event_pump: sdl2::EventPump){
        let nnn: u16 = opcode & 0x0fff;
        let mut x = ((opcode >> 8) & 0x000f) as usize;
        let mut y = ((opcode >> 4) & 0x000f) as usize;
        let kk: u8 = (opcode & 0x00ff) as u8;
        let n: u8 = (opcode & 0x000f) as u8;
        /////////////////////////////////////////////////////////////
        match opcode{

            // CLS: Clear The Display
            0x00E0=>self.display.clear_display(),

            // Ret: Return from subroutine
            0x00EE=>self.registers.PC = self.stack_pop(),

            // Special 
            _ => match opcode & 0xf000{

                // JP addr, 1nnn Jump to location nnn's
                0x1000=>self.registers.PC = nnn,

                // CALL addr, 2nnn Call subroutine at location nnn
                0x2000=>{
                    self.stack_push(self.registers.PC);
                    self.registers.PC = nnn;
                },

                // SE Vx, byte - 3xkk Skip next instruction if Vx=kk
                0x3000=>{
                    if self.registers.V[x] == kk{
                        self.registers.PC += 2;
                    }
                },

                // SNE Vx, byte - 3xkk Skip next instruction if Vx!=kk
                0x4000=>{
                    if self.registers.V[x] != kk{
                        self.registers.PC += 2;
                    }
                },

                // 5xy0 - SE, Vx, Vy, skip the next instruction if Vx = Vy
                0x5000=>{
                    if self.registers.V[x] == self.registers.V[y]{
                        self.registers.PC += 2;
                    }
                },

                // 6xkk - LD Vx, byte, Vx = kk
                0x6000=> self.registers.V[x] = kk,

                // 7xkk - ADD Vx, byte. Set Vx = Vx + kk
                0x7000=> self.registers.V[x] = self.registers.V[x] + kk,

                // Special 
                0x8000=> {
                    x = ((opcode >> 8) & 0x000f) as usize;
                    y = ((opcode >> 4) & 0x000f) as usize;
                    let last_bits = opcode as u8 & 0x000f;
                    let tmp: u16;
                    match last_bits{
                        // 8xy0 - LD Vx, Vy. Vx = Vy
                        0x00=>self.registers.V[x] = self.registers.V[y],

                        // 8xy1 - OR Vx, Vy. Performs a bitwise OR on Vx and Vy stores the result in Vx
                        0x01=>self.registers.V[x] = self.registers.V[x] | self.registers.V[y],

                        // 8xy2 - AND Vx, Vy. Performs a bitwise AND on Vx and Vy stores the result in Vx
                         0x02=>self.registers.V[x] = self.registers.V[x] & self.registers.V[y],

                        // 8xy3 - XOR Vx, Vy. Performs a bitwise XOR on Vx and Vy stores the result in Vx
                         0x03=>self.registers.V[x] = self.registers.V[x] ^ self.registers.V[y],

                        // 8xy4 - ADD Vx, Vy. Set Vx = Vx + Vy, set VF = carry
                         0x04=>{
                            tmp = self.registers.V[x] as u16 + self.registers.V[y] as u16;
                            self.registers.V[0x0f] = 0;
                            if tmp > 0xff{
                                self.registers.V[0x0f] = 1;
                            }
                            self.registers.V[x] = tmp as u8;
                        },

                        // 8xy5 - SUB Vx, Vy. Set vx = Vx - Vy, set VF = Not borrow
                        0x05=>{
                            self.registers.V[0x0f] = 0;
                            if self.registers.V[x] > self.registers.V[y]{
                                self.registers.V[0x0f] = 1;
                            }
                            self.registers.V[x] = self.registers.V[x] - self.registers.V[y];
                        },

                        // 8xy6 - SHR Vx {, Vy}
                        0x06=>{
                            self.registers.V[0x0f] = self.registers.V[x] & 0x01;
                            self.registers.V[x] = self.registers.V[x] / 2;
                        },

                        // 8xy7 - SUBN Vx, Vy
                        0x07=>{
                            self.registers.V[0x0f] = 0;
                            if self.registers.V[y] > self.registers.V[x]{
                                self.registers.V[0x0f] = 1;
                            };
                            self.registers.V[x] = self.registers.V[y] - self.registers.V[x];
                        },

                        // 8xyE - SHL Vx {, Vy}
                        0x0E=>{
                            self.registers.V[0x0f] = self.registers.V[x] & 0b10000000;
                            self.registers.V[x] = self.registers.V[x] * 2;
                        },

                        _=>{}
                    }
                },

                // 9xy0 - SNE Vx, Vy. Skip next instruction if Vx != Vy
                0x9000=>if self.registers.V[x] != self.registers.V[y]{
                        self.registers.PC += 2;
                    },

                // Annn - LD I, addr. Sets the I register to nnn
                0xA000=>self.registers.I = nnn,

                // bnnn - Jump to location nnn + V0
                0xB000=>self.registers.PC = nnn + self.registers.V[0x00] as u16,

                // Cxkk - RND Vx, byte
                0xC000=>self.registers.V[x] = random::<u8>() & kk,

                // Dxyn - DRW Vx, Vy, nibble. Draws sprite to the screen
                0xD000=>{
                    // const char* sprite = (const char*) &self.memory.memory[self.registers.I];
                    // self.registers.V[0x0f] = chip8_screen_draw_sprite(&self.screen, self.registers.V[x], self.registers.V[y], sprite, n);
                    let mut pixel_collision: u8 = 0;
                    for i in 0..n{
                        let c:u8 = self.memory.bytes[self.registers.I as usize + i as usize];
                        for j in 0..8 as u8{
                            if (c & (0b10000000 >> j)) == 0{
                                continue;
                            };
                            if self.display.pixels[(i+self.registers.V[y]) as usize % DISPLAY_HEIGHT][(j+self.registers.V[x]) as usize %DISPLAY_WIDTH] {
                                pixel_collision = 1;
                            };
                            self.display.pixels[(i+self.registers.V[y]) as usize%DISPLAY_HEIGHT][(j+self.registers.V[x]) as usize%DISPLAY_WIDTH] ^= true;
                            self.registers.V[0x0f] = pixel_collision;

                        }
                    }
                },

                // Keyboard operations
                0xE000=>{
                    match opcode & 0x00ff{
                        // Ex9e - SKP Vx, Skip the next instruction if the key with the value of Vx is pressed
                        0x9e=>if self.keyboard.is_pressed(self.registers.V[x] as usize){
                            self.registers.PC += 2;
                        },

                        // Exa1 - SKNP Vx - Skip the next instruction if the key with the value of Vx is not pressed
                        0xa1=>if !self.keyboard.is_pressed(self.registers.V[x] as usize){
                            self.registers.PC += 2;
                        },

                        _=>{}
                    }
                },

                0xF000=>{
                    x = ((opcode >> 8) & 0x000f) as usize;
                    match opcode & 0x00ff {
                        // fx07 - LD Vx, DT. Set Vx to the delay timer value
                        0x07=>self.registers.V[x] = self.registers.delay_timer,

                        // fx0a - LD Vx, K
                        0x0A=>{
                            let pressed_key = self.wait_for_input(event_pump);
                            self.registers.V[x] = pressed_key;
                        },

                        // fx15 - LD DT, Vx, set the delay timer to Vx
                        0x15=>self.registers.delay_timer = self.registers.V[x],

                        // fx18 - LD ST, Vx, set the sound timer to Vx
                        0x18=>self.registers.sound_timer = self.registers.V[x],

                        // fx1e - Add I, Vx
                        0x1e=>self.registers.I +=  self.registers.V[x] as u16,

                        // fx29 - LD F, Vx
                        0x29=>self.registers.I = (self.registers.V[x] * DEFAULT_SPRITE_HEIGHT) as u16,

                        // fx33 - LD B, Vx
                        0x33=>{
                            let h: u8 = (self.registers.V[x] / 100) as u8;
                            let t: u8 = ((self.registers.V[x] / 10) % 10) as u8;
                            let u: u8 = (self.registers.V[x] % 10) as u8;
                            self.memory.set_mem(self.registers.I as usize, h);
                            self.memory.set_mem((self.registers.I+1) as usize, t);
                            self.memory.set_mem((self.registers.I+2) as usize, u);
                        },

                        // fx55 - LD [I], Vx
                        0x55=>{
                            for i in 0..x+1{
                                self.memory.set_mem(self.registers.I as usize + i, self.registers.V[i]);
                            }
                        },

                        // fx65 - LD Vx, [I]
                        0x65=>{
                            for i in 0..x+1{
                                self.registers.V[i] = self.memory.get_mem(self.registers.I as usize + i);
                            }
                        },

                        _=> {}
                    }
                },

                _=> {}
                
            }
        }
        /////////////////////////////////////////////////////////////
    }
}