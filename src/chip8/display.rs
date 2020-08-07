// mod config;
use crate::chip8::config::*;

pub struct Display{
    pub pixels : [[bool;DISPLAY_WIDTH];DISPLAY_HEIGHT]
}

impl Display{
    pub fn new()->Display{
        Display{
            pixels: [[false;DISPLAY_WIDTH];DISPLAY_HEIGHT]
        }
    }
    pub fn check_screen_bounds(x: usize, y: usize){
        assert!(x < DISPLAY_WIDTH && y < DISPLAY_HEIGHT);
    }
    pub fn clear_display(&mut self){
        self.pixels = [[false;DISPLAY_WIDTH];DISPLAY_HEIGHT];
    }
    pub fn get_pixel(&self, x: usize, y: usize)->bool{
        Display::check_screen_bounds(x,y);
        return self.pixels[y][x];
    }
}
