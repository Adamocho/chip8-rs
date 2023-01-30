# CHIP-8 emulator in rust

## Origins and reasoning
A friend of mine told me into writing an emulator like CHIP-8, which proved to be a challenge alongside an epic adventure! I chose Rust because it is fast, reliable and guarantees memory safety, to name a few.

Resources I used:
+ [CHIP-8 Wikipedia](https://en.wikipedia.org/wiki/CHIP-8)
+ [CHIP-8 guide by Tobias V. Langhoff](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)

*Hope it will encourage you to write one entirely on your own*

## Requirements
- [`crossterm`](https://crates.io/crates/inquire) - cross-platform terminal manipulation library
- [`inquire`](https://crates.io/crates/crossterm) - interactive terminal prompts library

## Installation

Clone the repository
```bash
git clone https://github.com/Adamocho/chip8-rust.git
```

*cd* inside, build and execute
```bash
cd chip8-rust

cargo run
# OR run with debugging
cargo run --features debug
```

## Uninstalling

To uninstall simply delete the project's directory
```bash
rm -r chip8-rust
```