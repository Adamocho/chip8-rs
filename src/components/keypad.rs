use std::io::stdout;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{enable_raw_mode, self};

pub struct Keypad {
    keys: Vec<char>,
}

impl Default for Keypad {
    fn default() -> Self {
        Self::new()
    }
}

impl Keypad {
    pub fn new() -> Self {
        Keypad {
            keys : vec![
                        '1', '2', '3', '4',
                        'q', 'w', 'e', 'r',
                        'a', 's', 'd', 'f',
                        'z', 'x', 'c', 'v'
                    ]
        }
    }

    pub fn await_key_press(&self) -> u8 {
        let mut _stdout = stdout();
        if !terminal::is_raw_mode_enabled().unwrap() {
            enable_raw_mode().unwrap();
        }

        loop {
            if let Event::Key(KeyEvent { code: KeyCode::Char(c), modifiers: KeyModifiers::NONE, .. }) = read().unwrap() {
                return c as u8;
            }
        }
    }

    pub fn get_key_pressed(&self) -> Option<u8> {
        let mut _stdout = stdout();
        if !terminal::is_raw_mode_enabled().unwrap() {
            enable_raw_mode().unwrap();
        }

        if let Event::Key(KeyEvent { code: KeyCode::Char(c), modifiers: KeyModifiers::NONE, .. }) = read().unwrap() {
            if self.keys.contains(&c) {
                return Some(self.keys.iter().position(|ch| *ch == c).unwrap() as u8 + 1)
            }
            Some(0)
        } else {
            None
        }
    }
}