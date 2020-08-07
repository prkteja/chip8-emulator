// mod config;
extern crate sdl2;
use crate::chip8::config::*;
use sdl2::keyboard::Keycode;

#[derive(Copy, Clone)]
pub struct Keyboard{
    keys: [bool; TOTAL_KEYS],
    key_map: [Keycode; TOTAL_KEYS]
}

impl Keyboard{

    pub fn new(km: [Keycode; TOTAL_KEYS])->Keyboard{
        Keyboard{
            keys: [false; TOTAL_KEYS],
            key_map: km
        }
    }

    pub fn get_key_map(self, key: Keycode) -> i64{
        let keymap = self.key_map.iter().position(|&s| s == key);
        match keymap{
            None=> return -1,
            _=> {
                let ret = keymap.unwrap() as i64;
                return ret;
            }
        }
    }

    pub fn key_down(&mut self, keynum: usize){
        if keynum < TOTAL_KEYS {
            self.keys[keynum] = true;
        }
    }

    pub fn key_up(&mut self, keynum: usize){
        if keynum < TOTAL_KEYS {
            self.keys[keynum] = false;
        }
    }

    pub fn is_pressed(&self, keynum: usize)->bool{
        if keynum < TOTAL_KEYS {
            return self.keys[keynum];
        };
        return false;
    }

}