use crate::display::{Display, FONT_SET, WIDTH, HEIGHT};
use crate::keypad::Keypad;
use crate::rand::DoomRNG;

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
    pub stack: u16,
    // delay timer
    pub dt: u8,
    // sound timer
    pub st: u8,
    // display
    pub display: Display,
    // keypad
    pub keypad: Keypad,
    // random number generator - doom implementation (0-255) 
    pub random: DoomRNG, 
}

fn read_opcode(memory: [u8; 4096], index: u16) -> u16 {
    (memory[index as usize] as u16) << 8 |
        memory[(index + 1) as usize] as u16
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            i : 0,
            pc : 0,
            memory : [0; 4096],
            v : [0; 16],
            stack : 0,
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
        self.stack = 0;
        self.dt = 0;
        self.st = 0;
        self.display.cls();
        self.keypad.reset();

        // load the font
        for i in 0..80 {
            self.memory[i] = FONT_SET[i];
        }
    } 

    pub fn load_program(&mut self, program: Vec<u8>) {
        for i in 0..program.len(){
            // start at memory address 512 - specification
            self.memory[i + 0x200] = program[i];
        }
    }

    pub fn execute_cycle(&mut self) {
        // read next opcode
        let opcode = read_opcode(self.memory, self.pc);

        // increment the program counter
        self.pc += 2;

        self.process_opcode(opcode);
    }

    fn process_opcode(&mut self, opcode: u16)  {

        // break into various parameters
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let vx = self.v[x];
        let vy = self.v[y];
        let n = opcode & 0x000F;
        let kk = (opcode & 0x00FF) as u8;
        let nnn = (opcode & 0x0FFF) as u16;

        // break up into nibbles
        let op_1 = (opcode & 0xF000) >> 12; 
        let op_2 = (opcode & 0x0F00) >> 8; 
        let op_3 = (opcode & 0x00F0) >> 4; 
        let op_4 = opcode & 0x000F; 

        // println!("{:#06x} -> {} {} {} {}", opcode, op_1, op_2, op_3, op_4);

        match (op_1, op_2, op_3, op_4) {
            (0, 0, 0xE, 0) => {
                self.display.cls();
            }
            (0x1, _, _, _) => {
                self.pc = nnn;
            }
            (0x6, _, _, _) => {
                self.v[x] = kk;
            }
            (0x7, _, _, _) => {
                self.v[x] += ((vx as u16 + kk as u16) & 0xFF) as u8;
                // self.v[x] &= 0xFF;
            }
            (0xA, _, _, _) => {
                self.i = nnn;
            }
            (0xD, _, _, _) => {
                // reset the carry flag beforehand
                self.v[0xF] = 0;

                // wrap around if exceeds screen borders
                let mut xcoord = vx % WIDTH as u8;
                let mut ycoord = vy % HEIGHT as u8;

                // draw sprites on display
                for voffset in 0..n {
                    let sprite: u8 = self.memory[(self.i + voffset) as usize];
                    for hoffset in 0..8 {
                        if (sprite & (0b10000000 >> hoffset)) != 0 {
                            self.v[0xF] |= self.display.draw(xcoord, ycoord) as u8;

                            // debug
                            // println!("Draw sprite: {:#0x} at ({},{})", sprite, xcoord, ycoord);
                        }
                        xcoord += 1;
                        if xcoord >= WIDTH as u8 { break; }
                    }
                    // reset xcoord for next iteration
                    xcoord = vx % WIDTH as u8;
                    
                    ycoord += 1;
                    if ycoord >= HEIGHT as u8 { break; }
                }
                // debug
                self.display.print_to_console();
            }
            (_, _, _, _) => ()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Cpu;
    
    #[test]
    fn it_works() {
        let mut cpu = Cpu::new();
        cpu.pc = 1001;

        cpu.process_opcode(0x2ABC);
        assert_eq!(1, 1, "Hello it works");
        todo!("Make real tests");
    }
}