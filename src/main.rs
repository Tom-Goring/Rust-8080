#![allow(dead_code)]
#![allow(unused_variables)]

mod i8080;
mod disassembler;

use i8080::cpu::CPU;
use disassembler::disassemble_8080_op;

fn main() {
    // let mut file = File::open("./ROMS/invaders.h").expect("Couldn't open file for some reason.");
    // let mut data = Vec::new();

    // file.read_to_end(&mut data).expect("Couldn't read file for some reason.");

    // let mut pc = 0;

    // for byte in &data {
    //     pc += disassembler::disassemble_8080_op(&data, pc) as u16;
    //     if pc > 0x3f {
    //         break;
    //     }
    // }

    let mut cpu = CPU::new();

    cpu.reg.a = 0xFF;
    cpu.reg.b = 0xFF;

    cpu.memory[0] = 0x80;

    disassemble_8080_op(&cpu.memory, 0x0);
    cpu.execute_opcode(cpu.fetch());
    
    println!("{}", cpu.reg.a);

    let a: u8 = 0xFF;
    let b: u8 = 0xFF;

    let c: u16 = a as u16 + b as u16;

    let d = c as u8;
    println!("{:04X}", d);
}