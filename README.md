# CHIP-8 emulator in Rust
![chip8-rust logo](logo.svg)

## Origins and reasoning
A friend of mine told me into writing an emulator like CHIP-8, which proved to be a challenge alongside an epic adventure! I chose Rust because it is fast, reliable and guarantees memory safety, to name a few.

Resources used:
+ [CHIP-8 Wikipedia](https://en.wikipedia.org/wiki/CHIP-8)
+ [Amazing guide by Tobias V. Langhoff](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)

*Hope this will encourage you to write one entirely on your own!*

## Requirements
- [`crossterm`](https://crates.io/crates/inquire) - cross-platform terminal manipulation library
- [`inquire`](https://crates.io/crates/crossterm) - interactive terminal prompts library

> **NOTE:** Because those libraries are cross-platform, the whole package should work on any system you please.

## Installation

Clone the repository
```bash
git clone https://github.com/Adamocho/chip8-rust.git
# OR a shallow clone
git clone --depth 1 https://github.com/Adamocho/chip8-rust.git
```

*cd* inside; build and execute
```bash
cd chip8-rust

cargo run

# OR run with debugging use either
cargo run --features debug
# OR
cargo run -F debug
```

## Uninstalling

Simply delete the project's directory
```bash
rm -r chip8-rust
```