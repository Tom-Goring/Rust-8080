#![allow(dead_code)]
#![allow(unused_variables)]

extern crate sdl2;

mod i8080;
mod disassembler;
mod machine;
mod screen;

use screen::Screen;
use std::io::{stdin, stdout, Read, Write};

fn main() -> Result<(), String> {
    let cpu = i8080::cpu::CPU::new();

    let sdl_context = sdl2::init()?;

    let mut _screen = Screen::new(&sdl_context)?;

    pause();

    Ok(())
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}