# CHIP-8 emulator in Rust
![chip8-rust logo](logo.svg)

## Table of Content
- [Origins and reasoning](#origins-and-reasoning)
- [Requirements](#requirements)
- [Installation](#installation)
- [Uninstalling](#uninstalling)
- [Bonus features](#bonus-features)
    - [Debug](#debug)
    - [Fast](#fast)

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
```

## Uninstalling

Simply delete the project's directory
```bash
rm -r chip8-rust
```

## Bonus features 

> Both `--feature` and `-F` are the exact same flag.

Activate a feature/s using

```bash
cargo run --feature [NAME1] [NAME2] [...]
# OR
cargo run -F [NAME1] [NAME2] [...]
```

### Debug

You can guess it's usage :wink: 

Enabling the feature: 
- shows loaded ROM in *HEX*;
- executes it in steps (press `Enter` to progress);
- additionally, the whole struct is printed to console.

### Fast

Ignores the 60Hz *(60 per sec)* opcode processing limitation.