#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs::File;
use std::io::Read;

mod i8080;
mod disassembler;

fn main() {
    let mut file = File::open("./ROMS/invaders").expect("Couldn't open file for some reason.");
    let mut data = Vec::new();

    file.read_to_end(&mut data).expect("Couldn't read file for some reason.");

    let mut pc = 0;

    for byte in &data {
        pc += disassembler::disassemble_8080_op(&data, pc) as u16;
        if pc > 30 {
            break;
        }
    }
}