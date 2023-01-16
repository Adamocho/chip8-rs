pub struct Keypad {
    pub keys: [bool; 16],
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
            keys : [false; 16]
        }
    }

    pub fn reset(&mut self) {
        self.keys = [false; 16];
    }
}