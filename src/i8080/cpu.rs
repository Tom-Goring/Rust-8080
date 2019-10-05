pub type Address = u16;
pub type Word = u16;
pub type Byte = u8;

use super::register::Register;
use super::register::Flag;
use super::memory::Memory;

use crate::disassembler::disassemble_8080_op;

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
        disassemble_8080_op(&self.memory, self.reg.pc);
        self.reg.pc += self.execute_opcode(opcode);
    }

    pub fn fetch(&self) -> Byte {
        self.memory[self.reg.pc]
    }
}

impl CPU {
    fn read_byte_at_address(&self, address: Address) -> Byte {
        self.memory[address]
    }

    fn read_word_at_address(&self, address: Address) -> Word {
         ((self.memory[address + 1] as Word) << 8) | self.memory[address] as Word
    }

    fn read_byte_immediate(&self) -> Byte {
        self.read_byte_at_address(self.reg.pc + 1)
    }

    fn read_word_immediate(&self) -> Word {
        self.read_word_at_address(self.reg.pc + 1)
    }

    fn write_word_to_memory(&mut self, address: Address, word: Word) {
        self.memory[address + 1] = (word >> 8) as Byte;
        self.memory[address] = word as u8;
    }

    fn set_zspac_flags_on_byte(&mut self, byte: Byte) {
        self.reg.set_flag(Flag::Z, byte == 0);
        self.reg.set_flag(Flag::S, (byte & 0x80) != 0);
        self.reg.set_flag(Flag::P, byte.count_ones() % 2 == 0);
        self.reg.set_flag(Flag::AC, byte > 0xF);
    }

    fn set_all_flags_on_word(&mut self, word: Word) {
        self.reg.set_flag(Flag::C, word > 0xFF);
        self.reg.set_flag(Flag::Z, word == 0);
        self.reg.set_flag(Flag::S, (word & 0x80) != 0);
        self.reg.set_flag(Flag::P, word.count_ones() % 2 == 0);
        self.reg.set_flag(Flag::AC, word > 0xF);
    }

    fn add(&mut self, reg: Byte) -> Word {
        let sum: u16 = self.reg.a as u16 + reg as u16;
        self.set_all_flags_on_word(sum);
        self.reg.a = sum as Byte;
        1
    }

    fn rlc(&mut self) -> Word {
        self.reg.set_flag(Flag::C, self.reg.a >> 7 != 0);
        self.reg.a = self.reg.a << 1 | self.reg.a >> 7;
        1
    }

    fn rrc(&mut self) -> Word {
        self.reg.set_flag(Flag::C, self.reg.a << 7 != 0);
        self.reg.a = self.reg.a << 7 | self.reg.a >> 1;
        1
    }

    fn ral(&mut self) -> Word {
        let set_flag = self.reg.a >> 7 != 0;
        self.reg.a = self.reg.a << 1;
        if self.reg.get_flag(Flag::C) { self.reg.a |= 0b00000001; }
        self.reg.set_flag(Flag::C, set_flag);
        1
    }

    fn rar(&mut self) -> Word {
        let set_flag = self.reg.a << 7 != 0;
        self.reg.a = self.reg.a >> 1;
        if self.reg.get_flag(Flag::C) { self.reg.a |= 0b10000000; }
        self.reg.set_flag(Flag::C, set_flag);
        1
    }
}

impl CPU {
    pub fn execute_opcode(&mut self, opcode: Byte) -> Word {
        match opcode {
            // 00
            0x00 => { println!("NOP"); 1 },
            0x01 => { self.reg.set_bc(self.read_word_immediate()); 3 },
            0x02 => { self.memory[self.reg.get_bc()] = self.reg.a; 1 },
            0x03 => { self.reg.set_bc(self.reg.get_bc() + 1);      1 },
            0x04 => { self.reg.b += 1; self.set_zspac_flags_on_byte(self.reg.b); 1 },
            0x05 => { self.reg.b -= 1; self.set_zspac_flags_on_byte(self.reg.b); 1 },
            0x06 => { self.reg.b = self.read_byte_immediate(); 2 },
            0x07 => { self.rlc() },

            // 08
            0x08 => { println!("NOP"); 1 },
            0x09 => { self.reg.set_hl(self.reg.get_hl() + self.reg.get_bc()); 1 },
            0x0a => { self.reg.a = self.memory[self.reg.get_bc()]; 1 },
            0x0b => { self.reg.set_bc(self.reg.get_bc() - 1); 1 },
            0x0c => { self.reg.c += 1; self.set_zspac_flags_on_byte(self.reg.c); 1 },
            0x0d => { self.reg.c -= 1; self.set_zspac_flags_on_byte(self.reg.c); 1 },
            0x0e => { self.reg.c = self.read_byte_immediate(); 2 },
            0x0f => { self.rrc() },

            // 10
            0x10 => { println!("NOP"); 1 },
            0x11 => { self.reg.set_de(self.read_word_immediate()); 3 },
            0x12 => { self.memory[self.reg.get_de()] = self.reg.a; 1 },
            0x13 => { self.reg.set_de(self.reg.get_de() + 1);      1 },
            0x14 => { self.reg.d += 1; self.set_zspac_flags_on_byte(self.reg.d); 1 },
            0x15 => { self.reg.d -= 1; self.set_zspac_flags_on_byte(self.reg.d); 1 },
            0x16 => { self.reg.d = self.read_byte_immediate(); 2 },
            0x17 => { self.ral() },

            // 18
            0x18 => { println!("NOP"); 1 },
            0x19 => { self.reg.set_hl(self.reg.get_hl() + self.reg.get_de()); 1 },
            0x1a => { self.reg.a = self.memory[self.reg.get_de()]; 1 },
            0x1b => { self.reg.set_de(self.reg.get_de() - 1); 1 },
            0x1c => { self.reg.e += 1; self.set_zspac_flags_on_byte(self.reg.e); 1 },
            0x1d => { self.reg.e -= 1; self.set_zspac_flags_on_byte(self.reg.e); 1 },
            0x1e => { self.reg.e = self.read_byte_immediate(); 2 },
            0x1f => { self.rar() },

            // 20
            0x20 => { println!("NOP"); 1 },
            0x21 => { self.reg.set_hl(self.read_word_immediate()); 3 },
            0x22 => {
                let address = self.read_word_immediate() & 0x00FF;
                self.memory[address] = self.reg.l;
                let address = self.read_word_immediate() >> 8;
                self.memory[address] = self.reg.h;
                3
            },
            0x23 => { self.reg.set_hl(self.reg.get_hl() + 1);      1 },
            0x24 => { self.reg.h += 1; self.set_zspac_flags_on_byte(self.reg.h); 1 },
            0x25 => { self.reg.h -= 1; self.set_zspac_flags_on_byte(self.reg.h); 1 },
            0x26 => { self.reg.h = self.read_byte_immediate(); 2 },
            0x27 => {0}, // TODO: After BCD -> DAA

            // 28
            0x28 => { println!("NOP"); 1 },
            0x29 => { self.reg.set_hl(self.reg.get_hl() + self.reg.get_hl()); 1 },
            0x2a => {
                let address = self.read_word_immediate() & 0x00FF;
                self.reg.l = self.memory[address];
                let address = self.read_word_immediate() >> 8;
                self.reg.h = self.memory[address];
                3
            },
            0x2b => { self.reg.set_hl(self.reg.get_hl() - 1); 1 },
            0x2c => { self.reg.l += 1; self.set_zspac_flags_on_byte(self.reg.l); 1 },
            0x2d => { self.reg.l -= 1; self.set_zspac_flags_on_byte(self.reg.l); 1 },
            0x2e => { self.reg.l = self.read_byte_immediate(); 2 },
            0x2f => { self.reg.a = !self.reg.a; 1 },

            // 30
            0x30 => { println!("NOP"); 1 },
            0x31 => { self.reg.sp = self.read_word_immediate(); 3 },
            0x32 => { 
                let address = self.read_word_immediate();
                self.memory[address] = self.reg.a; 
                3
            },
            0x33 => { self.reg.sp += 1; 1 },
            0x34 => {
                self.memory[self.reg.get_hl()] += 1;
                self.set_zspac_flags_on_byte(self.memory[self.reg.get_hl()]); 
                1
            },
            0x35 => {
                self.memory[self.reg.get_hl()] -= 1;
                self.set_zspac_flags_on_byte(self.memory[self.reg.get_hl()]); 
                1
            },
            0x36 => { self.memory[self.reg.get_hl()] = self.read_byte_immediate(); 2 },
            0x37 => { self.reg.set_flag(Flag::C, true); 1 },

            // 38
            0x38 => { println!("NOP"); 1 },
            0x39 => { self.reg.set_hl(self.reg.get_hl() + self.reg.sp); 1 },
            0x3a => {
                 let address = self.read_word_immediate();
                 self.reg.a = self.memory[address];
                 3
            },
            0x3b => { self.reg.sp -= 1; 1 },
            0x3c => { self.reg.a += 1; self.set_zspac_flags_on_byte(self.reg.a); 1 },
            0x3d => { self.reg.a -= 1; self.set_zspac_flags_on_byte(self.reg.a); 1},
            0x3e => { self.reg.a = self.read_byte_immediate(); 2 },
            0x3f => { self.reg.set_flag(Flag::C, !self.reg.get_flag(Flag::C)); 1 },

            // 40
            0x40 => { self.reg.b = self.reg.b; 1},
            0x41 => { self.reg.b = self.reg.c; 1},
            0x42 => { self.reg.b = self.reg.d; 1},
            0x43 => { self.reg.b = self.reg.e; 1},
            0x44 => { self.reg.b = self.reg.h; 1},
            0x45 => { self.reg.b = self.reg.l; 1},
            0x46 => { self.reg.b = self.memory[self.reg.get_hl()]; 1},
            0x47 => { self.reg.b = self.reg.a; 1},

            // 48
            0x48 => { self.reg.c = self.reg.b; 1},
            0x49 => { self.reg.c = self.reg.c; 1},
            0x4a => { self.reg.c = self.reg.d; 1},
            0x4b => { self.reg.c = self.reg.e; 1},
            0x4c => { self.reg.c = self.reg.h; 1},
            0x4d => { self.reg.c = self.reg.l; 1},
            0x4e => { self.reg.c = self.memory[self.reg.get_hl()]; 1},
            0x4f => { self.reg.c = self.reg.a; 1},

            // 50
            0x50 => { self.reg.d = self.reg.b; 1},
            0x51 => { self.reg.d = self.reg.c; 1},
            0x52 => { self.reg.d = self.reg.d; 1},
            0x53 => { self.reg.d = self.reg.e; 1},
            0x54 => { self.reg.d = self.reg.h; 1},
            0x55 => { self.reg.d = self.reg.l; 1},
            0x56 => { self.reg.d = self.memory[self.reg.get_hl()]; 1},
            0x57 => { self.reg.d = self.reg.a; 1},

            // 58
            0x58 => { self.reg.e = self.reg.b; 1},
            0x59 => { self.reg.e = self.reg.c; 1},
            0x5a => { self.reg.e = self.reg.d; 1},
            0x5b => { self.reg.e = self.reg.e; 1},
            0x5c => { self.reg.e = self.reg.h; 1},
            0x5d => { self.reg.e = self.reg.l; 1},
            0x5e => { self.reg.e = self.memory[self.reg.get_hl()]; 1},
            0x5f => { self.reg.e = self.reg.a; 1},

            // 60
            0x60 => { self.reg.h = self.reg.b; 1},
            0x61 => { self.reg.h = self.reg.c; 1},
            0x62 => { self.reg.h = self.reg.d; 1},
            0x63 => { self.reg.h = self.reg.e; 1},
            0x64 => { self.reg.h = self.reg.h; 1},
            0x65 => { self.reg.h = self.reg.l; 1},
            0x66 => { self.reg.h = self.memory[self.reg.get_hl()]; 1},
            0x67 => { self.reg.h = self.reg.a; 1},

            // 68
            0x68 => { self.reg.l = self.reg.b; 1},
            0x69 => { self.reg.l = self.reg.c; 1},
            0x6a => { self.reg.l = self.reg.d; 1},
            0x6b => { self.reg.l = self.reg.e; 1},
            0x6c => { self.reg.l = self.reg.h; 1},
            0x6d => { self.reg.l = self.reg.l; 1},
            0x6e => { self.reg.l = self.memory[self.reg.get_hl()]; 1},
            0x6f => { self.reg.l = self.reg.a; 1},

            // 70
            0x70 => { self.memory[self.reg.get_hl()] = self.reg.b; 1},
            0x71 => { self.memory[self.reg.get_hl()] = self.reg.c; 1},
            0x72 => { self.memory[self.reg.get_hl()] = self.reg.d; 1},
            0x73 => { self.memory[self.reg.get_hl()] = self.reg.e; 1},
            0x74 => { self.memory[self.reg.get_hl()] = self.reg.h; 1},
            0x75 => { self.memory[self.reg.get_hl()] = self.reg.l; 1},
            0x76 => {0}, // TODO: HLT
            0x77 => { self.memory[self.reg.get_hl()] = self.reg.a; 1},

            // 78
            0x78 => { self.reg.a = self.reg.b; 1},
            0x79 => { self.reg.a = self.reg.c; 1},
            0x7a => { self.reg.a = self.reg.d; 1},
            0x7b => { self.reg.a = self.reg.e; 1},
            0x7c => { self.reg.a = self.reg.h; 1},
            0x7d => { self.reg.a = self.reg.l; 1},
            0x7e => { self.reg.a = self.memory[self.reg.get_hl()]; 1},
            0x7f => { self.reg.a = self.reg.a; 1},

            // 80
            0x80 => { self.add(self.reg.b) },
            0x81 => { self.add(self.reg.c) },
            0x82 => { self.add(self.reg.d) },
            0x83 => { self.add(self.reg.e) },
            0x84 => { self.add(self.reg.h) },
            0x85 => { self.add(self.reg.l) },
            0x86 => { self.add(self.reg.a) },
            0x87 => { self.add(self.read_byte_at_address(self.reg.get_hl())) },

            // 88
            0x88 => {0},
            0x89 => {0},
            0x8a => {0},
            0x8b => {0},
            0x8c => {0},
            0x8d => {0},
            0x8e => {0},
            0x8f => {0},

            // 90
            0x90 => {0},
            0x91 => {0},
            0x92 => {0},
            0x93 => {0},
            0x94 => {0},
            0x95 => {0},
            0x96 => {0},
            0x97 => {0},

            // 98
            0x98 => {0},
            0x99 => {0},
            0x9a => {0},
            0x9b => {0},
            0x9c => {0},
            0x9d => {0},
            0x9e => {0},
            0x9f => {0},

            // a0
            0xa0 => {0},
            0xa1 => {0},
            0xa2 => {0},
            0xa3 => {0},
            0xa4 => {0},
            0xa5 => {0},
            0xa6 => {0},
            0xa7 => {0},

            // a8
            0xa8 => {0},
            0xa9 => {0},
            0xaa => {0},
            0xab => {0},
            0xac => {0},
            0xad => {0},
            0xae => {0},
            0xaf => {0},

            // b0
            0xb0 => {0},
            0xb1 => {0},
            0xb2 => {0},
            0xb3 => {0},
            0xb4 => {0},
            0xb5 => {0},
            0xb6 => {0},
            0xb7 => {0},

            // b8
            0xb8 => {0},
            0xb9 => {0},
            0xba => {0},
            0xbb => {0},
            0xbc => {0},
            0xbd => {0},
            0xbe => {0},
            0xbf => {0},

            // c0
            0xc0 => {0},
            0xc1 => {0},
            0xc2 => {0},
            0xc3 => {0},
            0xc4 => {0},
            0xc5 => {0},
            0xc6 => {0},
            0xc7 => {0},

            // c8
            0xc8 => {0},
            0xc9 => {0},
            0xca => {0},
            0xcb => {0},
            0xcc => {0},
            0xcd => {0},
            0xce => {0},
            0xcf => {0},

            // d0
            0xd0 => {0},
            0xd1 => {0},
            0xd2 => {0},
            0xd3 => {0},
            0xd4 => {0},
            0xd5 => {0},
            0xd6 => {0},
            0xd7 => {0},

            // d8
            0xd8 => {0},
            0xd9 => {0},
            0xda => {0},
            0xdb => {0},
            0xdc => {0},
            0xdd => {0},
            0xde => {0},
            0xdf => {0},

            // e0
            0xe0 => {0},
            0xe1 => {0},
            0xe2 => {0},
            0xe3 => {0},
            0xe4 => {0},
            0xe5 => {0},
            0xe6 => {0},
            0xe7 => {0},

            // e8
            0xe8 => {0},
            0xe9 => {0},
            0xea => {0},
            0xeb => {0},
            0xec => {0},
            0xed => {0},
            0xee => {0},
            0xef => {0},

            // f0
            0xf0 => {0},
            0xf1 => {0},
            0xf2 => {0},
            0xf3 => {0},
            0xf4 => {0},
            0xf5 => {0},
            0xf6 => {0},
            0xf7 => {0},

            // f8
            0xf8 => {0},
            0xf9 => {0},
            0xfa => {0},
            0xfb => {0},
            0xfc => {0},
            0xfd => {0},
            0xff => {0},
            0xfe => {0},
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_accessors() {
        let mut cpu = CPU::new();

        cpu.memory[0x0] = 0x34;
        cpu.memory[0x1] = 0x11;
        cpu.memory[0x2] = 0x24;
        cpu.memory[0x3] = 0x31;
        cpu.memory[0x4] = 0x47;

        assert_eq!(cpu.read_byte_at_address(0x0), 0x34);
        assert_eq!(cpu.read_byte_at_address(0x1), 0x11);
        assert_eq!(cpu.read_byte_at_address(0x2), 0x24);
        assert_eq!(cpu.read_byte_at_address(0x3), 0x31);
        assert_eq!(cpu.read_byte_at_address(0x4), 0x47);

        assert_eq!(cpu.read_word_at_address(0x0), 0x1134);
        assert_eq!(cpu.read_word_at_address(0x1), 0x2411);

        cpu.reg.pc = 0x0;
        assert_eq!(cpu.read_byte_immediate(), 0x11);
        cpu.reg.pc += 0x1;
        assert_eq!(cpu.read_byte_immediate(), 0x24);
        assert_eq!(cpu.read_word_immediate(), 0x3124);
    }

    #[test]
    fn test_write_word() {
        let mut cpu = CPU::new();

        cpu.write_word_to_memory(0, 0xAABB);
        assert_eq!(cpu.memory[0], 0xBB);
        assert_eq!(cpu.memory[1], 0xAA);
    }

    #[test]
    fn test_add() {
        let mut cpu = CPU::new();

        cpu.memory[0] = 0x80;
        cpu.memory[1] = 0x81;
        cpu.memory[2] = 0x82;
        cpu.memory[3] = 0x83;
        cpu.memory[4] = 0x84;
        cpu.memory[5] = 0x85;
        cpu.memory[6] = 0x86;
        cpu.memory[7] = 0x87;

        cpu.reg.a = 0b1;
        cpu.reg.b = 0b1;
        cpu.tick();
        assert_eq!(cpu.reg.a, 0b10);
        assert_eq!(cpu.reg.get_flag(Flag::Z), false);
        assert_eq!(cpu.reg.get_flag(Flag::S), false);
        assert_eq!(cpu.reg.get_flag(Flag::P), false);
        assert_eq!(cpu.reg.get_flag(Flag::C), false);
        assert_eq!(cpu.reg.get_flag(Flag::AC), false);

        cpu.reg.c = 0x1;
        cpu.tick();
        assert_eq!(cpu.reg.a, 0b11);
        assert_eq!(cpu.reg.get_flag(Flag::Z), false);
        assert_eq!(cpu.reg.get_flag(Flag::S), false);
        assert_eq!(cpu.reg.get_flag(Flag::P), true);
        assert_eq!(cpu.reg.get_flag(Flag::C), false);
        assert_eq!(cpu.reg.get_flag(Flag::AC), false);

        cpu.reg.d = 0x1;
        cpu.tick();
        assert_eq!(cpu.reg.a, 0b100);
        assert_eq!(cpu.reg.get_flag(Flag::Z), false);
        assert_eq!(cpu.reg.get_flag(Flag::S), false);
        assert_eq!(cpu.reg.get_flag(Flag::P), false);
        assert_eq!(cpu.reg.get_flag(Flag::C), false);
        assert_eq!(cpu.reg.get_flag(Flag::AC), false);

        cpu.reg.e = 0x1;
        cpu.tick();
        assert_eq!(cpu.reg.a, 0b101);
        assert_eq!(cpu.reg.get_flag(Flag::Z), false);
        assert_eq!(cpu.reg.get_flag(Flag::S), false);
        assert_eq!(cpu.reg.get_flag(Flag::P), true);
        assert_eq!(cpu.reg.get_flag(Flag::C), false);
        assert_eq!(cpu.reg.get_flag(Flag::AC), false);

        cpu.reg.h = 0x1;
        cpu.tick();
        assert_eq!(cpu.reg.a, 0b110);
        assert_eq!(cpu.reg.get_flag(Flag::Z), false);
        assert_eq!(cpu.reg.get_flag(Flag::S), false);
        assert_eq!(cpu.reg.get_flag(Flag::P), true);
        assert_eq!(cpu.reg.get_flag(Flag::C), false);
        assert_eq!(cpu.reg.get_flag(Flag::AC), false);

        cpu.reg.l = 0x1;
        cpu.tick();
        assert_eq!(cpu.reg.a, 0b111);
        assert_eq!(cpu.reg.get_flag(Flag::Z), false);
        assert_eq!(cpu.reg.get_flag(Flag::S), false);
        assert_eq!(cpu.reg.get_flag(Flag::P), false);
        assert_eq!(cpu.reg.get_flag(Flag::C), false);
        assert_eq!(cpu.reg.get_flag(Flag::AC), false);

        cpu.tick();
        assert_eq!(cpu.reg.a, 0b1110);
        assert_eq!(cpu.reg.get_flag(Flag::Z), false);
        assert_eq!(cpu.reg.get_flag(Flag::S), false);
        assert_eq!(cpu.reg.get_flag(Flag::P), false);
        assert_eq!(cpu.reg.get_flag(Flag::C), false);
        assert_eq!(cpu.reg.get_flag(Flag::AC), false);

        cpu.reg.a = 0x0;
        cpu.memory[0x1001] = 0xFF;
        cpu.reg.set_hl(0x1001);

        cpu.tick();

        assert_eq!(cpu.reg.a, 0xFF);

        assert_eq!(cpu.reg.get_flag(Flag::Z), false);
        assert_eq!(cpu.reg.get_flag(Flag::S), true);
        assert_eq!(cpu.reg.get_flag(Flag::P), true);
        assert_eq!(cpu.reg.get_flag(Flag::C), false);
        assert_eq!(cpu.reg.get_flag(Flag::AC), true);
    }

    #[test]
    fn test_pc_increment() { 
        let mut cpu = CPU::new();

        cpu.reg.a = 0x1;
        cpu.reg.b = 0x1;

        cpu.memory[0] = 0x80;

        cpu.tick();

        assert_eq!(cpu.reg.pc, 0x1);
    }

    #[test]
    fn test_lxi() {
        let mut cpu = CPU::new();

        cpu.memory[0] = 0x1;
        cpu.memory[1] = 0x35;
        cpu.memory[2] = 0x76;

        cpu.memory[3] = 0x11;
        cpu.memory[4] = 0x35;
        cpu.memory[5] = 0x76;

        cpu.memory[6] = 0x21;
        cpu.memory[7] = 0x35;
        cpu.memory[8] = 0x76;

        cpu.memory[9] = 0x31;
        cpu.memory[10] = 0x35;
        cpu.memory[11] = 0x76;

        cpu.tick();
        assert_eq!(cpu.reg.get_bc(), 0x7635);

        cpu.tick();
        assert_eq!(cpu.reg.get_de(), 0x7635);

        cpu.tick();
        assert_eq!(cpu.reg.get_hl(), 0x7635);

        cpu.tick();
        assert_eq!(cpu.reg.sp, 0x7635);
    }

    #[test]
    fn test_stax() {
        let mut cpu = CPU::new();

        cpu.memory[cpu.reg.pc] = 0x2;
        cpu.reg.a = 0x42;
        cpu.reg.set_bc(0xF00F);

        cpu.tick();

        assert_eq!(cpu.memory[0xF00F], 0x42);

        cpu.memory[cpu.reg.pc] = 0x12;
        cpu.reg.a = 0x82;
        cpu.reg.set_de(0xEA3);

        cpu.tick();

        assert_eq!(cpu.memory[0xEA3], 0x82);
    }

    #[test]
    fn test_inx() {
        let mut cpu = CPU::new();

        cpu.memory[0] = 0x3;
        cpu.memory[1] = 0x13;
        cpu.memory[2] = 0x23;

        cpu.reg.set_bc(0);
        cpu.reg.set_de(0);
        cpu.reg.set_hl(0);

        cpu.tick();
        cpu.tick();
        cpu.tick();

        assert_eq!(cpu.reg.get_bc(), 0x1);
        assert_eq!(cpu.reg.get_de(), 0x1);
        assert_eq!(cpu.reg.get_hl(), 0x1);
    }

    #[test]
    fn test_inr() {
        let mut cpu = CPU::new();

        cpu.memory[0] = 0x4;
        cpu.memory[1] = 0xC;
        cpu.memory[2] = 0x14;
        cpu.memory[3] = 0x1C;
        cpu.memory[4] = 0x24;
        cpu.memory[5] = 0x2C;
        cpu.memory[6] = 0x3C;
        cpu.memory[7] = 0x34; // we execute M last because it changes the l register
        
        for x in 0..7 {
            cpu.tick();
        }

        assert_eq!(cpu.reg.a, 1);
        assert_eq!(cpu.reg.b, 1);
        assert_eq!(cpu.reg.c, 1);
        assert_eq!(cpu.reg.d, 1);
        assert_eq!(cpu.reg.e, 1);
        assert_eq!(cpu.reg.h, 1);
        assert_eq!(cpu.reg.l, 1);

        cpu.reg.set_hl(8);
        cpu.tick();
        assert_eq!(cpu.memory[8], 1);
    }

    #[test]
    fn test_dcr() {
        let mut cpu = CPU::new();

        cpu.reg.a = 2;
        cpu.reg.b = 2;
        cpu.reg.c = 2;
        cpu.reg.d = 2;
        cpu.reg.e = 2;
        cpu.reg.h = 2;
        cpu.reg.l = 2;
        
        cpu.memory[0] = 0x5;
        cpu.memory[1] = 0xD;
        cpu.memory[2] = 0x15;
        cpu.memory[3] = 0x1D;
        cpu.memory[4] = 0x25;
        cpu.memory[5] = 0x2D;
        cpu.memory[6] = 0x3D;
        cpu.memory[7] = 0x35; // we execute M last because it changes the l register
        
        for x in 0..7 {
            cpu.tick();
        }

        assert_eq!(cpu.reg.a, 1);
        assert_eq!(cpu.reg.b, 1);
        assert_eq!(cpu.reg.c, 1);
        assert_eq!(cpu.reg.d, 1);
        assert_eq!(cpu.reg.e, 1);
        assert_eq!(cpu.reg.h, 1);
        assert_eq!(cpu.reg.l, 1);

        cpu.memory[8] = 2;
        cpu.reg.set_hl(8);
        cpu.tick();
        assert_eq!(cpu.memory[8], 1);
    }

    #[test]
    fn test_mvi() {
        let mut cpu = CPU::new();
        cpu.memory[0] = 0x06;
        cpu.memory[1] = 0x10;
        cpu.memory[2] = 0x0E;
        cpu.memory[3] = 0x10;
        cpu.memory[4] = 0x16;
        cpu.memory[5] = 0x10;
        cpu.memory[6] = 0x1E;
        cpu.memory[7] = 0x10;
        cpu.memory[8] = 0x26;
        cpu.memory[9] = 0x10;
        cpu.memory[10] = 0x2E;
        cpu.memory[11] = 0x10;
        cpu.memory[12] = 0x3E;
        cpu.memory[13] = 0x10;
        cpu.memory[14] = 0x36;
        cpu.memory[15] = 0x10;

        for x in 0..7 {
            cpu.tick();
        }

        assert_eq!(cpu.reg.a, 0x10);
        assert_eq!(cpu.reg.b, 0x10);
        assert_eq!(cpu.reg.c, 0x10);
        assert_eq!(cpu.reg.d, 0x10);
        assert_eq!(cpu.reg.e, 0x10);
        assert_eq!(cpu.reg.h, 0x10);
        assert_eq!(cpu.reg.l, 0x10);

        cpu.reg.set_hl(0xFF);
        cpu.tick();
        assert_eq!(cpu.memory[cpu.reg.get_hl()], 0x10);
    }

    #[test]
    fn test_dad() {
        let mut cpu = CPU::new();
        cpu.memory[0] = 0x9;
        cpu.memory[1] = 0x19;
        cpu.memory[2] = 0x29;
        cpu.reg.set_bc(0x1);
        cpu.reg.set_de(0x1);
        cpu.reg.set_hl(0x1);

        cpu.tick();
        cpu.tick();
        cpu.tick();

        assert_eq!(cpu.reg.get_hl(), 0x6);

        cpu.memory[3] = 0x39;
        cpu.reg.sp = 1;
        cpu.tick();
        assert_eq!(cpu.reg.get_hl(), 0x7);
    }

    #[test]
    fn test_ldax() {
        let mut cpu = CPU::new();
        cpu.memory[0] = 0x0A;
        cpu.memory[1] = 0x1A;

        cpu.memory[0xF1] = 0x10;
        cpu.memory[0xF2] = 0x20;

        cpu.reg.set_bc(0xF1);
        cpu.reg.set_de(0xF2);

        cpu.tick();
        assert_eq!(cpu.reg.a, 0x10);
        cpu.tick();
        assert_eq!(cpu.reg.a, 0x20);
    }

    #[test]
    fn test_dcx() {
        let mut cpu = CPU::new();
        cpu.memory[0] = 0x0B;
        cpu.memory[1] = 0x1B;
        cpu.memory[2] = 0x2B;
        cpu.memory[3] = 0x3B;

        cpu.reg.set_bc(1);
        cpu.reg.set_de(1);
        cpu.reg.set_hl(1);
        cpu.reg.sp = 1;

        for x in 0..4 {
            cpu.tick();
        }

        assert_eq!(cpu.reg.get_bc(), 0);
        assert_eq!(cpu.reg.get_de(), 0);
        assert_eq!(cpu.reg.get_hl(), 0);
        assert_eq!(cpu.reg.sp, 0);
    }

    #[test]
    fn test_rlc() {
        let mut cpu = CPU::new();
        cpu.memory[0] = 0x07;
        cpu.reg.a = 0b10101010;

        cpu.tick();
        assert_eq!(cpu.reg.a, 0b01010101);
        assert_eq!(cpu.reg.get_flag(Flag::C), true);

        cpu.reg.pc = 0;
        cpu.tick();

        assert_eq!(cpu.reg.a, 0b10101010);
        assert_eq!(cpu.reg.get_flag(Flag::C), false);
    }

    #[test]
    fn test_rrc() {
        let mut cpu = CPU::new();
        cpu.memory[0] = 0x0F;
        cpu.reg.a = 0b10000001;

        cpu.tick();
        assert_eq!(cpu.reg.a, 0b11000000);
        assert_eq!(cpu.reg.get_flag(Flag::C), true);

        cpu.reg.pc = 0;
        cpu.tick();
        cpu.reg.a = 0b01100000;
         assert_eq!(cpu.reg.get_flag(Flag::C), false);
    }

    #[test]
    fn test_ral() {
        let mut cpu = CPU::new();
        cpu.memory[0] = 0x17;
        cpu.reg.a = 0b10101010;

        cpu.tick();
        assert_eq!(cpu.reg.a, 0b01010100);
        assert_eq!(cpu.reg.get_flag(Flag::C), true);

        cpu.reg.pc = 0;
        cpu.tick();

        assert_eq!(cpu.reg.a, 0b10101001);
        assert_eq!(cpu.reg.get_flag(Flag::C), false);
    }

    #[test]
    fn test_rar() {
        let mut cpu = CPU::new();
        cpu.memory[0] = 0x1F;
        cpu.reg.a = 0b10000001;

        cpu.tick();
        assert_eq!(cpu.reg.a, 0b01000000);
        assert_eq!(cpu.reg.get_flag(Flag::C), true);

        cpu.reg.pc = 0;
        cpu.tick();

        assert_eq!(cpu.reg.a, 0b10100000);
    }

    #[test]
    fn test_shld() {
        let mut cpu = CPU::new();
        cpu.memory[0] = 0x22;
        cpu.memory[1] = 0xAA;
        cpu.memory[2] = 0xBB;
        cpu.reg.l = 0xCC;
        cpu.reg.h = 0xDD;

        cpu.tick();

        assert_eq!(cpu.memory[0xAA], 0xCC);
        assert_eq!(cpu.memory[0xBB], 0xDD);
    }

    #[test]
    fn test_lhld() {
        let mut cpu = CPU::new();
        cpu.memory[0] = 0x2a;
        cpu.memory[1] = 0xAA;
        cpu.memory[2] = 0xBB;
        cpu.memory[0xAA] = 0xEE;
        cpu.memory[0xBB] = 0xFF;
        cpu.reg.l = 0xCC;
        cpu.reg.h = 0xDD;

        cpu.tick();

        assert_eq!(cpu.reg.l, 0xEE);
        assert_eq!(cpu.reg.h, 0xFF);
    }

    #[test]
    fn test_cma() {
        let mut cpu = CPU::new();
        cpu.memory[0] = 0x2f;
        cpu.reg.a = 0b00000001;
        cpu.tick();
        assert_eq!(cpu.reg.a, 0b11111110);
    }

    #[test]
    fn test_sta() {
        let mut cpu = CPU::new();
        cpu.memory[0] = 0x32;
        cpu.memory[1] = 0xBB;
        cpu.memory[2] = 0xAA;
        cpu.reg.a = 0xFF;
        cpu.tick();
        assert_eq!(cpu.memory[0xAABB], 0xFF);
    }

    #[test]
    fn test_inxsp() {
        let mut cpu = CPU::new();
        cpu.memory[0] = 0x33;
        cpu.tick();
        assert_eq!(cpu.reg.sp, 1);
    }

    #[test]
    fn test_stc() {
        let mut cpu = CPU::new();
        cpu.memory[0] = 0x37;
        cpu.tick();
        assert_eq!(cpu.reg.get_flag(Flag::C), true);
    }

    #[test]
    fn test_lda() {
        let mut cpu = CPU::new();
        cpu.memory[0] = 0x3a;
        cpu.memory[1] = 0xFF;
        cpu.memory[2] = 0x00;
        cpu.memory[0x00FF] = 0xAA;
        cpu.tick();
        assert_eq!(cpu.reg.a, 0xAA);
    }

    #[test]
    fn test_cmc() {
        let mut cpu = CPU::new();
        cpu.memory[0] = 0x3F;
        cpu.memory[1] = 0x3F;
        cpu.tick();
        assert_eq!(cpu.reg.get_flag(Flag::C), true);
        cpu.tick();
        assert_eq!(cpu.reg.get_flag(Flag::C), false);
    }

    #[test]
    fn test_mov() {
        let mut cpu = CPU::new();
        let mut index = 0;

        for x in 0x40..0x80 {
            cpu.memory[index] = x;
            index += 1;
        }

        let values = [2, 3, 4, 5, 6, 7, 8, 9];

        cpu.reg.b = values[0];
        cpu.reg.c = values[1];
        cpu.reg.d = values[2];
        cpu.reg.e = values[3];
        cpu.reg.h = values[4];
        cpu.reg.l = values[5];
        cpu.memory[cpu.reg.get_hl()] = values[6];
        cpu.reg.a = values[7];

        for x in 0..8 {
            cpu.tick();
            assert_eq!(cpu.reg.b, values[x]);
        }

        cpu.reg.b = values[0];
        cpu.reg.c = values[1];
        cpu.reg.d = values[2];
        cpu.reg.e = values[3];
        cpu.reg.h = values[4];
        cpu.reg.l = values[5];
        cpu.memory[cpu.reg.get_hl()] = values[6];
        cpu.reg.a = values[7];

        for x in 0..8 {
            cpu.tick();
            if x == 1 {cpu.reg.c = values[x]}
            assert_eq!(cpu.reg.c, values[x]);
        }

        cpu.reg.b = values[0];
        cpu.reg.c = values[1];
        cpu.reg.d = values[2];
        cpu.reg.e = values[3];
        cpu.reg.h = values[4];
        cpu.reg.l = values[5];
        cpu.memory[cpu.reg.get_hl()] = values[6];
        cpu.reg.a = values[7];

        for x in 0..8 {
            cpu.tick();
            if x == 2 {cpu.reg.d = values[x]}
            assert_eq!(cpu.reg.d, values[x]);
        }

        cpu.reg.b = values[0];
        cpu.reg.c = values[1];
        cpu.reg.d = values[2];
        cpu.reg.e = values[3];
        cpu.reg.h = values[4];
        cpu.reg.l = values[5];
        cpu.memory[cpu.reg.get_hl()] = values[6];
        cpu.reg.a = values[7];

        for x in 0..8 {
            cpu.tick();
            if x == 3 {cpu.reg.e = values[x]}
            assert_eq!(cpu.reg.e, values[x]);
        }

        cpu.reg.b = values[0];
        cpu.reg.c = values[1];
        cpu.reg.d = values[2];
        cpu.reg.e = values[3];
        cpu.reg.h = values[4];
        cpu.reg.l = values[5];
        cpu.memory[cpu.reg.get_hl()] = values[6];
        cpu.reg.a = values[7];

        for x in 0..8 {
            
            if x == 6 {
                cpu.reg.h = values[4];
                cpu.reg.l = values[5];
                cpu.memory[cpu.reg.get_hl()] = values[6];
            }
            cpu.tick();
            if x == 4 {cpu.reg.h = values[x]};
            assert_eq!(cpu.reg.h, values[x]);
        }

        cpu.reg.b = values[0];
        cpu.reg.c = values[1];
        cpu.reg.d = values[2];
        cpu.reg.e = values[3];
        cpu.reg.h = values[4];
        cpu.reg.l = values[5];
        cpu.memory[cpu.reg.get_hl()] = values[6];
        cpu.reg.a = values[7];

        for x in 0..8 {
            cpu.tick();
            if x == 5 {cpu.reg.l = values[x]}
            assert_eq!(cpu.reg.l, values[x]);
        }

        cpu.reg.b = values[0];
        cpu.reg.c = values[1];
        cpu.reg.d = values[2];
        cpu.reg.e = values[3];
        cpu.reg.h = values[4];
        cpu.reg.l = values[5];
        cpu.memory[cpu.reg.get_hl()] = values[6];
        cpu.reg.a = values[7];

        for x in 0..8 {
            if cpu.memory[cpu.reg.pc] != 0x76 {
                cpu.tick();
                assert_eq!(cpu.memory[cpu.reg.get_hl()], values[x]);
            } else {
                cpu.reg.pc += 1;
            }
        }

        cpu.reg.b = values[0];
        cpu.reg.c = values[1];
        cpu.reg.d = values[2];
        cpu.reg.e = values[3];
        cpu.reg.h = values[4];
        cpu.reg.l = values[5];
        cpu.memory[cpu.reg.get_hl()] = values[6];
        cpu.reg.a = values[7];  

        for x in 0..8 {
            cpu.tick();
            if x == 7 {cpu.reg.a = values[x]}
            assert_eq!(cpu.reg.a, values[x]);
        }
    } 
}