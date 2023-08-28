use std::io::Write;

use crossterm::{style::Print, cursor::MoveTo };

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

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
            memory : [false; WIDTH * HEIGHT]
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