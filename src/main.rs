#![allow(dead_code)]
#![allow(unused_variables)]

mod i8080;
mod disassembler;

fn main() {
    let mut cpu = i8080::cpu::CPU::new();

        for x in 0..0xFE {
            cpu.memory[x] = x as u8;
        }

        for x in 0..0xFE {
            disassembler::disassemble_8080_op(&cpu.memory, x);
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disassembler() {
        let mut cpu = i8080::cpu::CPU::new();

        for x in 0..0xFE {
            cpu.memory[x] = x as u8;
        }

        for x in 0..0xFE {
            disassembler::disassemble_8080_op(&cpu.memory, x);
        }
    }
}