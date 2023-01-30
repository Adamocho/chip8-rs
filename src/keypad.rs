use std::io::stdout;
use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Keypad {
    keys: [char; 16],
}

impl Keypad {
    pub fn new() -> Self {
        Keypad {
            keys : KEY_SET,
        }
    }

    pub fn get_keys(&self) -> [char; 16] {
        self.keys
    }

    pub fn await_key_press(&self) -> u8 {
        let mut _stdout = stdout();
        enable_raw_mode().unwrap();
        loop {
            match read().unwrap() {
                Event::Key(event) => match event.code {
                    KeyCode::Char(c) => { disable_raw_mode().unwrap(); return c as u8; }
                    _ => (),
                },
                _ => ()
            }
        }
    }

    pub fn get_key_pressed(&self) -> Option<u8> {
        let mut _stdout = stdout();
        enable_raw_mode().unwrap();
        match read().unwrap() {
            Event::Key(event) => match event.code {
                KeyCode::Char(c) => { disable_raw_mode().unwrap(); return Some(c as u8); }
                _ => { disable_raw_mode().unwrap(); return None; }
            },
            _ => { disable_raw_mode().unwrap(); return None; }
        }
    }
}

pub const  KEY_SET: [char; 16] = [
    '1', '2', '3', '4',
    'q', 'w', 'e', 'r',
    'a', 's', 'd', 'f',
    'z', 'x', 'c', 'v'
];