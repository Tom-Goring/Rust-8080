pub type Address = u16;
pub type Word = u16;
pub type Byte = u8;

use super::register::Register;
use super::register::Flag;
use super::memory::Memory;

pub struct CPU {
    pub reg: Register,
    pub memory: Memory,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            reg: Register::new(),
            memory: Memory::new(),
        }
    }

    pub fn tick(&mut self) {
        let opcode = self.fetch();
        self.execute_opcode(opcode);
    }

    pub fn fetch(&self) -> Byte {
        self.memory[self.reg.pc]
    }

    fn add(&mut self, reg: Byte) {
        let sum: u16 = self.reg.a as u16 + reg as u16;
        self.reg.set_flag(Flag::C, sum > 0xFF);

        self.reg.a = sum as u8;

        self.reg.set_flag(Flag::Z, self.reg.a == 0);
        self.reg.set_flag(Flag::S, (self.reg.a & 0x80) != 0);
        self.reg.set_flag(Flag::P, self.reg.a.count_ones() % 2 == 0);
        self.reg.set_flag(Flag::AC, self.reg.a > 0xF);
    }
}

impl CPU {
    pub fn execute_opcode(&mut self, opcode: Byte) {
        match opcode {
            // 00
            0x00 => {},
            0x01 => {},
            0x02 => {},
            0x03 => {},
            0x04 => {},
            0x05 => {},
            0x06 => {},
            0x07 => {},

            // 08
            0x08 => {},
            0x09 => {},
            0x0a => {},
            0x0b => {},
            0x0c => {},
            0x0d => {},
            0x0e => {},
            0x0f => {},

            // 10
            0x10 => {},
            0x11 => {},
            0x12 => {},
            0x13 => {},
            0x14 => {},
            0x15 => {},
            0x16 => {},
            0x17 => {},

            // 18
            0x18 => {},
            0x19 => {},
            0x1a => {},
            0x1b => {},
            0x1c => {},
            0x1d => {},
            0x1e => {},
            0x1f => {},

            // 20
            0x20 => {},
            0x21 => {},
            0x22 => {},
            0x23 => {},
            0x24 => {},
            0x25 => {},
            0x26 => {},
            0x27 => {},

            // 28
            0x28 => {},
            0x29 => {},
            0x2a => {},
            0x2b => {},
            0x2c => {},
            0x2d => {},
            0x2e => {},
            0x2f => {},

            // 30
            0x30 => {},
            0x31 => {},
            0x32 => {},
            0x33 => {},
            0x34 => {},
            0x35 => {},
            0x36 => {},
            0x37 => {},

            // 38
            0x38 => {},
            0x39 => {},
            0x3a => {},
            0x3b => {},
            0x3c => {},
            0x3d => {},
            0x3e => {},
            0x3f => {},

            // 40
            0x40 => {},
            0x41 => {},
            0x42 => {},
            0x43 => {},
            0x44 => {},
            0x45 => {},
            0x46 => {},
            0x47 => {},

            // 48
            0x48 => {},
            0x49 => {},
            0x4a => {},
            0x4b => {},
            0x4c => {},
            0x4d => {},
            0x4e => {},
            0x4f => {},

            // 50
            0x50 => {},
            0x51 => {},
            0x52 => {},
            0x53 => {},
            0x54 => {},
            0x55 => {},
            0x56 => {},
            0x57 => {},

            // 58
            0x58 => {},
            0x59 => {},
            0x5a => {},
            0x5b => {},
            0x5c => {},
            0x5d => {},
            0x5e => {},
            0x5f => {},

            // 60
            0x60 => {},
            0x61 => {},
            0x62 => {},
            0x63 => {},
            0x64 => {},
            0x65 => {},
            0x66 => {},
            0x67 => {},

            // 68
            0x68 => {},
            0x69 => {},
            0x6a => {},
            0x6b => {},
            0x6c => {},
            0x6d => {},
            0x6e => {},
            0x6f => {},

            // 70
            0x70 => {},
            0x71 => {},
            0x72 => {},
            0x73 => {},
            0x74 => {},
            0x75 => {},
            0x76 => {},
            0x77 => {},

            // 78
            0x78 => {},
            0x79 => {},
            0x7a => {},
            0x7b => {},
            0x7c => {},
            0x7d => {},
            0x7e => {},
            0x7f => {},

            // 80
            0x80 => { self.add(self.reg.b) },
            0x81 => { self.add(self.reg.c) },
            0x82 => { self.add(self.reg.d) },
            0x83 => { self.add(self.reg.e) },
            0x84 => { self.add(self.reg.h) },
            0x85 => { self.add(self.reg.l) },
            0x86 => { self.add(self.reg.a) },
            0x87 => { self.add(self.memory[self.reg.get_hl()])},

            // 88
            0x88 => {},
            0x89 => {},
            0x8a => {},
            0x8b => {},
            0x8c => {},
            0x8d => {},
            0x8e => {},
            0x8f => {},

            // 90
            0x90 => {},
            0x91 => {},
            0x92 => {},
            0x93 => {},
            0x94 => {},
            0x95 => {},
            0x96 => {},
            0x97 => {},

            // 98
            0x98 => {},
            0x99 => {},
            0x9a => {},
            0x9b => {},
            0x9c => {},
            0x9d => {},
            0x9e => {},
            0x9f => {},

            // a0
            0xa0 => {},
            0xa1 => {},
            0xa2 => {},
            0xa3 => {},
            0xa4 => {},
            0xa5 => {},
            0xa6 => {},
            0xa7 => {},

            // a8
            0xa8 => {},
            0xa9 => {},
            0xaa => {},
            0xab => {},
            0xac => {},
            0xad => {},
            0xae => {},
            0xaf => {},

            // b0
            0xb0 => {},
            0xb1 => {},
            0xb2 => {},
            0xb3 => {},
            0xb4 => {},
            0xb5 => {},
            0xb6 => {},
            0xb7 => {},

            // b8
            0xb8 => {},
            0xb9 => {},
            0xba => {},
            0xbb => {},
            0xbc => {},
            0xbd => {},
            0xbe => {},
            0xbf => {},

            // c0
            0xc0 => {},
            0xc1 => {},
            0xc2 => {},
            0xc3 => {},
            0xc4 => {},
            0xc5 => {},
            0xc6 => {},
            0xc7 => {},

            // c8
            0xc8 => {},
            0xc9 => {},
            0xca => {},
            0xcb => {},
            0xcc => {},
            0xcd => {},
            0xce => {},
            0xcf => {},

            // d0
            0xd0 => {},
            0xd1 => {},
            0xd2 => {},
            0xd3 => {},
            0xd4 => {},
            0xd5 => {},
            0xd6 => {},
            0xd7 => {},

            // d8
            0xd8 => {},
            0xd9 => {},
            0xda => {},
            0xdb => {},
            0xdc => {},
            0xdd => {},
            0xde => {},
            0xdf => {},

            // e0
            0xe0 => {},
            0xe1 => {},
            0xe2 => {},
            0xe3 => {},
            0xe4 => {},
            0xe5 => {},
            0xe6 => {},
            0xe7 => {},

            // e8
            0xe8 => {},
            0xe9 => {},
            0xea => {},
            0xeb => {},
            0xec => {},
            0xed => {},
            0xee => {},
            0xef => {},

            // f0
            0xf0 => {},
            0xf1 => {},
            0xf2 => {},
            0xf3 => {},
            0xf4 => {},
            0xf5 => {},
            0xf6 => {},
            0xf7 => {},

            // f8
            0xf8 => {},
            0xf9 => {},
            0xfa => {},
            0xfb => {},
            0xfc => {},
            0xfd => {},
            0xff => {},
            0xfe => {},
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add() {
        let mut cpu = CPU::new();

        cpu.reg.a = 0x1;
        cpu.reg.b = 0x1;

        assert_eq!(cpu.reg.a, 1);
        assert_eq!(cpu.reg.b, 1);

        cpu.memory[0] = 0x80;

        assert_eq!(cpu.memory[0], 0x80);

        cpu.execute_opcode(cpu.fetch());

        assert_eq!(cpu.reg.a, 2);

        assert_eq!(cpu.reg.get_flag(Flag::Z), false);
        assert_eq!(cpu.reg.get_flag(Flag::S), false);
        assert_eq!(cpu.reg.get_flag(Flag::P), false);
        assert_eq!(cpu.reg.get_flag(Flag::C), false);
        assert_eq!(cpu.reg.get_flag(Flag::AC), false);

        cpu.reg.a = 0xFF;
        cpu.reg.b = 0xFF;

        assert_eq!(cpu.reg.a, 0xFF);
        assert_eq!(cpu.reg.b, 0xFF);

        cpu.execute_opcode(cpu.fetch());
        
        assert_eq!(cpu.reg.get_flag(Flag::Z), false);
        assert_eq!(cpu.reg.get_flag(Flag::S), true);
        assert_eq!(cpu.reg.get_flag(Flag::P), false);
        assert_eq!(cpu.reg.get_flag(Flag::C), true);
        assert_eq!(cpu.reg.get_flag(Flag::AC), true);
    }
}