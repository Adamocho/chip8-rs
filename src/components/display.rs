use std::{env, io::Write};

use crossterm::{style::Print, cursor::MoveTo };
use minifb::{Window, WindowOptions};

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
pub const ON: u32 = 0xFF_FF_FF_FF;
pub const OFF: u32 = 0x0;

pub struct Display {
    pub memory: [bool; WIDTH * HEIGHT],
}

impl Default for Display {
    fn default() -> Self {
        Self::new()
    }
}

impl Display {
    pub fn new() -> Self {
        Display {
            memory : [false; WIDTH * HEIGHT],
        }
    }

    pub fn cls(&mut self) {
        self.memory = [false; WIDTH * HEIGHT]
    } 

    pub fn draw(&mut self, x: u8, y:u8) -> bool {
        self.memory[x as usize + (y as usize * WIDTH)] ^= true;

        // whether the pixel was turned off (v[0xF] register)
        !self.memory[x as usize + (y as usize * WIDTH)]
    }
    
    pub fn print_to_console(&mut self) {
        let on: char = '█';
        let off: char = ' ';

        let mut stdout = std::io::stdout();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let pixel: char = if self.memory[y * WIDTH + x] {on} else {off};
                crossterm::queue!(stdout,
                    MoveTo(x as u16, y as u16),
                    Print(pixel),
                ).unwrap();
            }
        }

        // Make changes visible on the screen
        stdout.flush().unwrap();
    }

    /// # Examples
    /// 
    /// ```sh
    /// # Set env variables
    /// $   export CHIP8_ON='170000040'
    ///
    /// $   export CHIP8_ON='16711400'
    /// 
    /// # And then..
    /// $   cargo run
    /// ```
    pub fn get_screen_buffer(&self) -> [u32; WIDTH * HEIGHT] {
        let on: u32 = if let Ok(val) = env::var("CHIP8_ON") {
            val.parse().unwrap_or(ON)
        } else { ON };

        let off: u32 = if let Ok(val) = env::var("CHIP8_OFF") {
            val.parse().unwrap_or(OFF)
        } else { OFF };

        self.memory.map(|pixel| if pixel { on } else { off })
    }

    pub fn create_window() -> Window {
        let mut window = Window::new(
            "Chip8 - Rust",
            self::WIDTH,
            self::HEIGHT,
            WindowOptions {
                scale: minifb::Scale::X8,
                borderless: false,
                title: true,
                resize: true,
                scale_mode: minifb::ScaleMode::Stretch,
                topmost: true,
                transparency: false,
                none: false,
            },
        ).unwrap();

        window.set_position(20, 20);
        // do not set a framerate limit for the display
        // but rather do it for the CPU
        window.limit_update_rate(None);
        window
    }
}

pub const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];
