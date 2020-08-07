#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]
extern crate sdl2;
mod chip8;
use crate::chip8::*;
use std::env;
use std::fs::read;
use std::path::Path;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;
use std::time::Duration;
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2, "invalid number of arguments\n");
    let path = Path::new(&args[1]);
    let buf: Vec<u8> = read(path).unwrap();
    let mut chip8: Chip8 = Chip8::new();
    chip8.chip8_init();
    chip8.chip8_load(buf);
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window(config::WINDOW_TITLE, (config::DISPLAY_WIDTH*config::DISPLAY_SCALE) as u32, (config::DISPLAY_HEIGHT*config::DISPLAY_SCALE) as u32)
        .position_centered()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();
    'running: loop{
        let mut event_pump = sdl_context.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'running,
                Event::KeyDown {keycode: Some(k), ..} => {
                    let key = chip8.keyboard.get_key_map(k);
                    if key != -1{
                        chip8.keyboard.key_down(key as usize);
                    }
                },
                Event::KeyUp {keycode: Some(k), ..} => {
                    let key = chip8.keyboard.get_key_map(k);
                    if key != -1{
                        chip8.keyboard.key_up(key as usize);
                    }
                },
                _ => {}
            }
        };
        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for x in 0..config::DISPLAY_WIDTH{
            for y in 0..config::DISPLAY_HEIGHT{
                if chip8.display.get_pixel(x, y){
                    let r:Rect = Rect::new((x*config::DISPLAY_SCALE) as i32, (y*config::DISPLAY_SCALE) as i32, config::DISPLAY_SCALE as u32, config::DISPLAY_SCALE as u32) ;
                    canvas.fill_rect(r);
                }
            }
        };
        canvas.present();
        while chip8.registers.delay_timer > 0{
            thread::sleep(Duration::from_millis(1));
            chip8.registers.delay_timer -= 1;
        };
        if chip8.registers.sound_timer > 0{
            //play a sound
            thread::sleep(Duration::from_millis(10*chip8.registers.sound_timer as u64));
            chip8.registers.sound_timer = 0;
        };

        let opcode: u16 = chip8.memory.get_mem_16(chip8.registers.PC as usize);
        chip8.registers.PC += 2;
        chip8.chip8_exec(opcode, event_pump);
        thread::sleep(Duration::from_millis(1));
    };
}
