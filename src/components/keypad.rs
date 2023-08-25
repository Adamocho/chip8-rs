use std::io::stdout;
use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

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
        enable_raw_mode().unwrap();
        loop {
            if let Event::Key(event) = read().unwrap() { 
                if let KeyCode::Char(c) = event.code {
                    disable_raw_mode().unwrap();
                    return c as u8
                }
            }
        }
    }

    pub fn get_key_pressed(&self) -> Option<u8> {
        let mut _stdout = stdout();
        enable_raw_mode().unwrap();
        match read().unwrap() {
            Event::Key(event) => match event.code {
                KeyCode::Char(c) => {
                    disable_raw_mode().unwrap();
                    if self.keys.contains(&c) {
                        return Some(self.keys.iter().position(|ch| *ch == c).unwrap() as u8 + 1)
                    }
                    Some(0)
                },
                KeyCode::Enter => { disable_raw_mode().unwrap(); println!("Enter"); Some(0) },
                KeyCode::Esc => { disable_raw_mode().unwrap(); println!("Exiting.."); None },
                _ => { disable_raw_mode().unwrap(); Some(0) }
            },
            _ => { disable_raw_mode().unwrap(); None }
        }
    }
}