use std::path::Path;
use std::time::Duration;
use chip_8::cpu::Cpu;
use std::fs;

fn main() {
    // let path: &Path = Path::new("roms/ibm_logo.ch8").as_ref();
    let path: &Path = Path::new("roms/test_opcode.ch8").as_ref();
    let rom = fs::read(path)
        .expect("Cannot read the file: \"{path}\"");

    // rom debug
    // for value in &rom {
    //     println!("{:#10x}", value);
    // }
    // println!("{}", rom.len());

    let mut cpu = Cpu::new();
    cpu.reset();
    cpu.load_program(rom);

    loop {
        cpu.execute_cycle();

        // simulate 60hz
        std::thread::sleep(Duration::new(0, 16_000_000));
    }

}
