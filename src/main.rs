#![allow(dead_code)]
#![allow(unused_variables)]

extern crate sdl2;

mod invaders;
mod i8080;

use std::io::prelude::*;
use std::fs::File;

static CYCLES_PER_FRAME: usize = 2_000_000 / 120;

fn main() {

    let rom = File::open("./ROMS/invaders").unwrap();
    let rom_bytes: Vec<u8> = rom.bytes().map(|x| x.unwrap()).collect();

    let mut invaders = invaders::SpaceInvaders::new(&rom_bytes);
    invaders.cpu.memory.load(0x000, &rom_bytes);
    
    loop {
        invaders.step();
    }
}

fn pause() {
    use std::io::{stdin, stdout};
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}