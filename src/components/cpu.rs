use crossterm::style::{Stylize, StyledContent};

use super::display::{Display, FONT_SET, WIDTH, HEIGHT};
use super::keypad::Keypad;
use super::rand::DoomRNG;

pub struct Cpu {
    // index register
    pub i: u16,
    // program counter
    pub pc: u16,
    // memory
    pub memory: [u8; 4096],
    // registers
    pub v: [u8; 16],
    // stack
    pub stack: Vec<u16>,
    // delay timer
    pub dt: u8,
    // sound timer
    pub st: u8,
    // display
    pub display: Display,
    // keypad
    pub keypad: Keypad,
    // random number generator - DOOM implementation
    pub random: DoomRNG,
}

fn read_opcode(memory: [u8; 4096], index: u16) -> u16 {
    (memory[index as usize] as u16) << 8 |
        memory[(index + 1) as usize] as u16
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu::new()
    }
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            i : 0,
            pc : 0,
            memory : [0; 4096],
            v : [0; 16],
            stack : vec![],
            dt: 0,
            st: 0,
            display : Display::new(),
            keypad : Keypad::new(),
            random : DoomRNG::new()
        }
    }
    
    pub fn reset(&mut self) {
        self.i = 0;
        self.pc = 512;
        self.memory = [0; 4096];
        self.v = [0; 16];
        self.stack = vec![];
        self.dt = 0;
        self.st = 0;
        self.display.cls();

        // load the font
        for (i, item) in FONT_SET.iter().enumerate() {
            self.memory[i] = *item;
        }
    } 

    pub fn load_program(&mut self, program: Vec<u8>) {
        self.memory[512..(program.len() + 512)].copy_from_slice(&program[..]);
    }

    pub fn execute_cycle(&mut self) {
        let opcode = read_opcode(self.memory, self.pc);
        self.pc += 2;
        self.process_opcode(opcode);

        // decrease both timers
        if self.st > 0 { self.st >>= 1 };
        if self.dt > 0 { self.dt >>= 1 };
    }

    fn process_opcode(&mut self, opcode: u16)  {

        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let vx = self.v[x];
        let vy = self.v[y];
        let n = opcode & 0x000F;
        let kk = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        // break up into nibbles
        let op_1 = (opcode & 0xF000) >> 12;
        let op_2 = (opcode & 0x0F00) >> 8;
        let op_3 = (opcode & 0x00F0) >> 4;
        let op_4 = opcode & 0x000F;

        let mut operation_type = "Unknown";

        match (op_1, op_2, op_3, op_4) {
            (0, 0, 0xE, 0) => {
                self.display.cls();
                operation_type = "Display";
            }
            (0, 0, 0xE, 0xE) => {
                self.pc = self.stack.pop().unwrap_or(512);
                operation_type = "Flow";
            }
            (0x1, _, _, _) => {
                self.pc = nnn;
                operation_type = "Flow";
            }
            (0x2, _, _, _) => {
                self.stack.push(self.pc);
                self.pc = nnn;
                operation_type = "Flow";
            }
            (0x3, _, _, _) => {
                if vx == kk {
                    self.pc += 2;
                }
                operation_type = "Cond";
            }
            (0x4, _, _, _) => {
                if vx != kk {
                    self.pc += 2;
                }
                operation_type = "Cond";
            }
            (0x5, _, _, 0) => {
                if vx == vy {
                    self.pc += 2;
                }
                operation_type = "Cond";
            }
            (0x6, _, _, _) => {
                self.v[x] = kk;
                operation_type = "Const";
            }
            (0x7, _, _, _) => {
                self.v[x] = ((vx as u16 + kk as u16) & 0xFF) as u8;
                operation_type = "Const";
            }
            (0x8, _, _, 0) => {
                self.v[x] = vy;
                operation_type = "Assig";
            }
            (0x8, _, _, 1) => {
                self.v[x] |= vy;
                operation_type = "BitOp";
            }
            (0x8, _, _, 2) => {
                self.v[x] &= vy;
                operation_type = "BitOp";
            }
            (0x8, _, _, 3) => {
                self.v[x] ^= vy;
                operation_type = "BitOp";
            }
            (0x8, _, _, 4) => {
                if vx as u16 + vy as u16 > 0xFF {
                    self.v[x] = (vx as u16 + vy as u16 - 0x100) as u8;
                    self.v[0xF] = 1;
                } else {
                    self.v[x] += vy;
                    self.v[0xF] = 0;
                }
                operation_type = "Math";
            }
            (0x8, _, _, 5) => {
                if vx < vy {
                    self.v[x] = (256 - (vy - vx) as u16) as u8;
                    self.v[0xF] = 0;
                } else {
                    self.v[x] -= vy;
                    self.v[0xF] = 1;
                }
                operation_type = "Math";
            }
            (0x8, _, _, 6) => {
                self.v[0xF] = vx & 0x1;
                self.v[x] >>= 1;
                operation_type = "BitOp";
            }
            (0x8, _, _, 7) => {
                if vy < vx {
                    self.v[0xF] = 0;
                    self.v[x] = (vx - vy) ^ 0xFF;
                } else {
                    self.v[x] = vy - vx;
                    self.v[0xF] = 1;
                }
                operation_type = "Math";
            }
            (0x8, _, _, 0xE) => {
                self.v[0xF] = vx & 0x80;
                self.v[x] <<= 1;
                operation_type = "BitOp";
            }
            (0x9, _, _, 0) => {
                if vx != vy {
                    self.pc += 2;
                }
                operation_type = "Cond";
            }
            (0xA, _, _, _) => {
                self.i = nnn;
                operation_type = "MEM";
            }
            (0xB, _, _, _) => {
                self.pc = nnn + self.v[0] as u16;
                operation_type = "Flow";
            }
            (0xC, _, _, _) => {
                self.v[x] = self.random.change() & kk;
                operation_type = "Rand";
            }
            (0xD, _, _, _) => {
                self.v[0xF] = 0;

                let mut xcoord = vx % WIDTH as u8;
                let mut ycoord = vy % HEIGHT as u8;

                for voffset in 0..n {
                    let sprite: u8 = self.memory[(self.i + voffset) as usize];
                    for hoffset in 0..8 {
                        if (sprite & (0b10000000 >> hoffset)) != 0 {
                            self.v[0xF] |= self.display.draw(xcoord, ycoord) as u8;
                        }
                        xcoord += 1;
                        if xcoord >= WIDTH as u8 { break; }
                    }
                    xcoord = vx % WIDTH as u8;
                    
                    ycoord += 1;
                    if ycoord >= HEIGHT as u8 { break; }
                }
                self.display.print_to_console();
                operation_type = "Display";
            }
            (0xE, _, 0x9, 0xE) => {
                if self.keypad.get_key_pressed().unwrap() == vx {
                    self.pc += 2;
                }
                operation_type = "KeyOp";
            }
            (0xE, _, 0xA, 0x1) => {
                if self.keypad.get_key_pressed().unwrap() != vx {
                    self.pc += 2;
                }
                operation_type = "KeyOp";
            }
            (0xF, _, 0, 0x7) => {
                self.v[x] = self.dt;
                operation_type = "Timer";
            }
            (0xF, _, 0, 0xA) => {
                self.v[x] = self.keypad.await_key_press();
                operation_type = "KeyOp";
            }
            (0xF, _, 0x1, 0x5) => {
                self.dt = vx;
                operation_type = "Timer";
            }
            (0xF, _, 0x1, 0x8) => {
                self.st = vx;
                operation_type = "Sound";
            }
            (0xF, _, 0x1, 0xE) => {
                self.i += vx as u16;
                operation_type = "MEM";
            }
            (0xF, _, 0x2, 0x9) => {
                self.i = (vx as u16 & 0xF) * 5;
                operation_type = "MEM";
            }
            (0xF, _, 0x3, 0x3) => {
                self.memory[self.i as usize] = vx / 100;
                self.memory[(self.i + 1) as usize] = (vx / 10) % 10;
                self.memory[(self.i + 2) as usize] = vx % 10;
                operation_type = "BCD";
            }
            (0xF, _, 0x5, 0x5) => {
                for offset in 0..=x {
                    self.memory[self.i as usize + offset] = self.v[offset];
                }
                operation_type = "MEM";
            }
            (0xF, _, 0x6, 0x5) => {
                for offset in 0..=x {
                    self.v[offset] = self.memory[self.i as usize + offset];
                }
                operation_type = "MEM";
            }
            (_, _, _, _) => ()
        }

        if cfg!(feature = "debug") {
            let opcode_styled: StyledContent<String>;
            let optype_styled: StyledContent<String>;

            let cpu_styled: StyledContent<String> = crossterm::style::style(format!("\ti={:?},\n\tpc={:?},\n\tv={:?},\n\tstack={:?},\n\tdt={:?},\n\tst={:?}",
            self.i, self.pc, self.v, self.stack, self.dt, self.st)).with(crossterm::style::Color::DarkYellow);

            if operation_type == "Unknown" {
                opcode_styled = crossterm::style::style(format!("{:04X}", opcode)).with(crossterm::style::Color::Red);
                optype_styled = crossterm::style::style(operation_type.to_string()).with(crossterm::style::Color::Red);
            } else {
                opcode_styled = crossterm::style::style(format!("{:04X}", opcode)).with(crossterm::style::Color::Green);
                optype_styled = crossterm::style::style(operation_type.to_string()).with(crossterm::style::Color::Green);
            }

            println!("Opcode={}, Type={} ",
            opcode_styled,
            optype_styled);
            println!("Cpu:\n{}", cpu_styled);

            let _ = std::io::stdin().read_line(&mut String::new());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Cpu, read_opcode};
    
    #[test]
    fn it_resets() {
        let mut cpu = Cpu::new();
        cpu.i = 2001;
        cpu.pc = 2002;
        cpu.memory = [4; 4096];
        cpu.v = [1; 16];
        cpu.stack = vec![1, 2, 3];
        cpu.dt = 120;
        cpu.st = 120;
        cpu.reset();

        assert_eq!(cpu.i, 0, "i does not reset");
        assert_eq!(cpu.pc, 512, "pc does not reset");
        assert_eq!(cpu.memory[80..], [0; 4096][80..], "memory does not reset");
        assert_eq!(cpu.v, [0; 16], "register does not reset");
        assert_eq!(cpu.stack, vec![], "stack does not reset");
        assert_eq!(cpu.dt, 0, "delay timer does not reset");
        assert_eq!(cpu.st, 0, "sound timer does not reset");
    }

    #[test]
    fn it_increments() {
        let mut cpu = Cpu::new();
        cpu.reset();
        cpu.execute_cycle();
        cpu.execute_cycle();
        cpu.execute_cycle();
        cpu.execute_cycle();

        assert_eq!(cpu.pc, 512 + 8, "pc does not increment");
    }

    #[test]
    fn it_reads_opcode() {
        let mut cpu = Cpu::new();
        cpu.reset();

        let mut opcode = read_opcode(cpu.memory, 0);
        assert_eq!(opcode, 0xF090, "wrong opcode read");

        opcode = read_opcode(cpu.memory, 81);
        assert_eq!(opcode, 0x0000, "wrong opcode read");

        cpu.memory[202] = 0xFA;
        cpu.memory[203] = 0x02;
        opcode = read_opcode(cpu.memory, 202);
        assert_eq!(opcode, 0xFA02, "wrong opcode read");
    }

    #[test]
    fn it_loads_program() {
        let mut cpu = Cpu::new();
        cpu.reset();

        cpu.load_program(vec![0x12, 0x24, 0xFD, 0x0A]);

        assert_eq!(cpu.memory[0x200], 0x12, "program not loaded");
        assert_eq!(cpu.memory[0x200 + 1], 0x24, "program not loaded");
        assert_eq!(cpu.memory[0x200 + 2], 0xFD, "program not loaded");
        assert_eq!(cpu.memory[0x200 + 3], 0x0A, "program not loaded");
    }
}