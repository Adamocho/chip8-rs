mod components;
use crate::components::cpu::Cpu;
use std::{time::Duration, io};
use crossterm::style::{style, Stylize, StyledContent};
use std::fs;
use inquire::{Select, ui::{RenderConfig, Color, StyleSheet, Styled, Attributes}};


fn main() {
    let dir_path = "roms/";
    let files: Vec<String> = fs::read_dir(dir_path)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_string_lossy().into_owned())
        .collect();

    let menu_render_config = RenderConfig::empty()
        .with_prompt_prefix(
            Styled::new("?")
            .with_fg(Color::rgb(231, 206, 140)))
        .with_highlighted_option_prefix(
            Styled::new(">")
            .with_fg(Color::LightBlue))
        .with_canceled_prompt_indicator(
            Styled::new("ERROR - canceled")
            .with_fg(Color::LightRed)
            .with_attr(Attributes::BOLD))
        .with_answer(
            StyleSheet::new()
            .with_fg(Color::LightGreen));

    let menu_dialog = format!("Choose ROM to execute (\"{}\" directory):", dir_path);

    let menu = Select::new(&menu_dialog, files.iter().map(|s| &s[dir_path.len()..]).collect())
        .with_render_config(menu_render_config)
        .without_help_message()
        .with_vim_mode(false);

    let file_path = menu.prompt().unwrap();

    let rom = fs::read(String::from(dir_path) + file_path)
        .expect("Cannot read the file: \"{path}\"");

    if cfg!(feature = "debug") {
        let debug_message = style("\nROM debug:\n").with(crossterm::style::Color::Yellow);
        let mut counter_message: StyledContent<String>;

        print!("{}", debug_message);

        for (counter, value) in rom.iter().enumerate() {
            if counter % 10 == 0 {
                counter_message = style(format!("\n{:#03}\t", counter)).with(crossterm::style::Color::Red);
                print!("{}", counter_message);
            }
            print!("{:#04x} ", value);
        }
        println!("\nConfirm to continue...");
        let _ = io::stdin().read_line(&mut String::new());
    }

    let mut cpu = Cpu::new();
    cpu.reset();
    cpu.load_program(rom);

    loop {
        cpu.execute_cycle();

        if !cfg!(feature = "fast") {
            // simulate 60hz
            std::thread::sleep(Duration::new(0, 16_000_000));
        }
    }
}
