#![allow(dead_code)]
#![allow(unused_variables)]

extern crate sdl2;

mod i8080;
mod disassembler;
mod machine;
mod screen;

use std::fs::File;
use std::io::prelude::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use machine::{Keys, SpaceInvadersMachine};
use screen::Screen;
use std::io::{stdin, stdout, Read, Write};

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut screen = Screen::new(&sdl_context)?;

    let mut cpu = i8080::cpu::CPU::new();
    let mut machine = SpaceInvadersMachine::new();
    machine.press_key(Keys::Coin);

    let rom = File::open("./ROMS/invaders").unwrap();
    let rom_bytes: Vec<u8> = rom.bytes().map(|x| x.unwrap()).collect();


    pause();

    Ok(())
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}