# CHIP-8 emulator in Rust
![chip8-rust logo](logo.svg)

## Table of Content
- [Origins and reasoning](#origins-and-reasoning)
- [Requirements](#requirements)
- [Installation](#installation)
- [Uninstalling](#uninstalling)
- [Getting ROMs](#getting-roms)
- [Features](#features)
    - [Window](#window)
    - [Alternate-screen](#alternate-screen)
    - [Debug](#debug)
    - [Fast](#fast)

## Origins and reasoning
A friend of mine told me into writing an emulator like CHIP-8, which proved to be a challenge alongside an epic adventure! I chose Rust because it is fast, reliable and guarantees memory safety, to name a few.

Resources used:
+ [CHIP-8 Wikipedia](https://en.wikipedia.org/wiki/CHIP-8)
+ [Amazing guide by Tobias V. Langhoff](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)

*Hope this will encourage you to write one entirely on your own!*

## Requirements
- [**crossterm**](https://crates.io/crates/inquire) - cross-platform terminal manipulation library
- [**inquire**](https://crates.io/crates/crossterm) - interactive terminal prompts library
- [**minifb**](https://crates.io/crates/minifb) - barebones window setup with bitmap rendering

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

## Getting ROMs

This repo provides just a few test ROMs purely for testing purposes, among which are `ibm_logo.ch8` and `test_opcode.ch8` most iconic. This project does not provide neither games nor demos.

Should you wish to run other **.ch8** ROMs, here are a few links:

- *https://github.com/kripod/chip8-roms* - kripod's repo of ROMs
- *https://archive.org/details/chip-8-games* - you can guess
- *https://github.com/tobiasvl/awesome-chip-8* - another repo (by tobiasvl)

The Internet is full of ROMs worth trying out.

## Work showcase

The `img/` directory contains a few showcases of working test **.ch8** files.
They are 1920x1080, so you can have a cool wallpaper out of them.

## Features

Activate a feature/s using:

```bash
cargo run --feature [NAME1] [NAME2] [...]
# OR
cargo run -F [NAME1] [NAME2] [...]
```

Available features are listed in `Cargo.toml`.

### Window

This feature is enabled by default. It creates a window, where all the magic takes place.

Quit by pressing the `Esc` key.

### Alternate-screen

Another way of rendering. It uses crossterm to create a virtual window, just like vim. The old screen is restored upon quitting (usually `Ctrl-C`).

### Debug

You can guess it's usage :wink: 

Enabling the feature: 
- shows loaded ROM in *HEX*;
- executes it in steps (press `Enter` to progress);
- additionally, the whole struct is printed to console.

> NOTE:
> Works best with `--no-default-features`, due to bugs when both `debug` and `default`|`window`|`alternate-screen` are on.

### Fast

Ignores the 60Hz *(60 per sec)* opcode processing limitation.

> NOTE: 
> Works only for a terminal-based output.