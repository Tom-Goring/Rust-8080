pub type Address = u16;
pub type Word = u16;
pub type Byte = u8;

use super::register::Register;
use super::memory::Memory;

use crate::disassembler::disassemble_8080_op;

use super::register::Reg8;
use super::register::Reg16;
use super::register::Reg8::{A, B, C, D, E, H, L, M};
use super::register::Reg16::{BC, DE, HL, SP};
use super::register::Flag::{Carry, Parity, Sign, AuxCarry, Zero};

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

    fn read_bytes_immediate(&self) -> (Byte, Byte) {
        (self.read_byte_at_address(self.reg.pc + 1), self.read_byte_at_address(self.reg.pc + 2))
    }

    fn read_word_immediate(&self) -> Word {
        self.read_word_at_address(self.reg.pc + 1)
    }

    fn write_word_to_memory(&mut self, address: Address, word: Word) {
        self.memory[address + 1] = (word >> 8) as Byte;
        self.memory[address] = word as u8;
    }

    fn set_flags_on_result(&mut self, result: Byte, overflow: bool) {
        self.set_zspac_flags_on_byte(result);
        self.reg.set_flag(Carry, overflow);
    }

    fn set_zspac_flags_on_byte(&mut self, byte: Byte) {
        self.reg.set_flag(Zero, byte == 0);
        self.reg.set_flag(Sign, (byte & 0x80) != 0);
        self.reg.set_flag(Parity, byte.count_ones() % 2 == 0);
        self.reg.set_flag(AuxCarry, byte > 0xF);
    }
}

impl CPU {
    fn lxi(&mut self, x: Reg16) -> Word {
        self.reg[x] = self.read_word_immediate();
        3
    }

    fn stax(&mut self, x: Reg16) -> Word {
        self.memory[self.reg[x]] = self.reg[A];
        1
    }

    fn inx(&mut self, x: Reg16) -> Word {
        self.reg[x] += 1;
        1
    }

    fn inr(&mut self, x: Reg8) -> Word {
        self.reg[x] += 1;
        1
    }

    fn inr_m(&mut self) -> Word {
        self.memory[self.reg[HL]] += 1;
        1
    }

    fn dcr(&mut self, x: Reg8) -> Word {
        self.reg[x] -= 1;
        1
    }

    fn dcr_m(&mut self) -> Word {
        self.memory[self.reg[HL]] -= 1;
        1
    }

    fn mvi(&mut self, x: Reg8) -> Word {
        self.reg[x] = self.read_byte_immediate();
        2
    }

    fn mvi_m(&mut self) -> Word {
        self.memory[self.reg[HL]] = self.read_byte_immediate();
        2
    }

    fn dad(&mut self, x: Reg16) -> Word {
        self.reg[HL] += self.reg[x];
        1
    }

    fn ldax(&mut self, x: Reg16) -> Word {
        self.reg[A] = self.memory[self.reg[x]];
        1
    }

    fn dcx(&mut self, x: Reg16) -> Word {
        self.reg[x] -= 1;
        1
    }

    fn shld(&mut self) -> Word {
        let (address, address2) = self.read_bytes_immediate();
        self.memory[address as Address] = self.reg[L];
        self.memory[address2 as Address] = self.reg[H];
        3
    }

    fn lhld(&mut self) -> Word {
        let (address, address2) = self.read_bytes_immediate();
        self.reg[L] = self.memory[address as Address];
        self.reg[H] = self.memory[address2 as Address];
        3
    }

    fn cma(&mut self) -> Word {
        self.reg[A] = !(self.reg[A]);
        1
    }

    fn sta(&mut self) -> Word {
        let address = self.read_word_immediate();
        self.memory[address] = self.reg[A];
        3
    }

    fn stc(&mut self) -> Word {
        self.reg.set_flag(Carry, true);
        1
    }

    fn lda(&mut self) -> Word {
        self.reg[A] = self.memory[self.read_word_immediate()];
        3
    }

    fn cmc(&mut self) -> Word {
        self.reg.set_flag(Carry, !self.reg.get_flag(Carry));
        1
    }

    fn mov(&mut self, dest: Reg8, src: Reg8) -> Word {
        if dest == M {
            self.memory[self.reg[HL]] = self.reg[src];
        } else if src == M {
            self.reg[dest] = self.memory[self.reg[HL]];
        } else {
            self.reg[dest] = self.reg[src];
            println!("Moving reg[{:?}] of value {} to reg[{:?}]", src, self.reg[src], dest);
        }
        1
    }

    fn add(&mut self, byte: Byte) -> Word {
        let (result, overflow) = self.reg[A].overflowing_add(byte);
        self.set_flags_on_result(result, overflow);
        self.reg[A] = result;
        1
    }

    fn adc(&mut self, byte: Byte) -> Word {
        let (mut result, overflow) = self.reg[A].overflowing_add(byte);
        if self.reg.get_flag(Carry) {result += 1};
        self.set_flags_on_result(result, overflow);
        self.reg[A] = result;
        1
    }

    fn sub(&mut self, byte: Byte) -> Word {
        let (result, overflow) = self.reg[A].overflowing_sub(byte);
        self.set_flags_on_result(result, overflow);
        self.reg[A] = result;
        1
    }

    fn sbb(&mut self, byte: Byte) -> Word {
        let (mut result, overflow) = self.reg[A].overflowing_sub(byte);
        if self.reg.get_flag(Carry) {result -= 1};
        self.set_flags_on_result(result, overflow);
        self.reg[A] = result;
        1
    }

    fn ana(&mut self, byte: Byte) -> Word {
        self.reg[A] &= byte;
        self.set_zspac_flags_on_byte(self.reg[A]);
        1
    }

    fn xra(&mut self, byte: Byte) -> Word {
        self.reg[A] ^= byte;
        self.set_zspac_flags_on_byte(self.reg[A]);
        1
    }

    fn ora(&mut self, byte: Byte) -> Word {
        self.reg[A] |= byte;
        self.set_zspac_flags_on_byte(self.reg[A]);
        1
    }

    fn cmp(&mut self, byte: Byte) -> Word {
        self.reg.set_flag(Carry, self.reg[A] < byte);
        self.reg.set_flag(Zero, self.reg[A] >= byte);
        1
    }

    fn rlc(&mut self) -> Word {
        self.reg.set_flag(Carry, self.reg[A] >> 7 != 0);
        self.reg[A] = self.reg[A] << 1 | self.reg[A] >> 7;
        1
    }

    fn rrc(&mut self) -> Word {
        self.reg.set_flag(Carry, self.reg.a << 7 != 0);
        self.reg.a = self.reg[A] << 7 | self.reg[A] >> 1;
        1
    }

    fn ral(&mut self) -> Word {
        let set_flag = self.reg[A] >> 7 != 0;
        self.reg.a = self.reg.a << 1;
        if self.reg.get_flag(Carry) { self.reg.a |= 0b00000001; }
        self.reg.set_flag(Carry, set_flag);
        1
    }

    fn rar(&mut self) -> Word {
        let set_flag = self.reg[A] << 7 != 0;
        self.reg[A] = self.reg[A] >> 1;
        if self.reg.get_flag(Carry) { self.reg[A] |= 0b10000000; }
        self.reg.set_flag(Carry, set_flag);
        1
    }
}

impl CPU {
    pub fn execute_opcode(&mut self, opcode: Byte) -> Word {
        match opcode {
            // 00
            0x00 => { println!("NOP"); 1 },
            0x01 => { self.lxi(BC)  },
            0x02 => { self.stax(BC) },
            0x03 => { self.inx(BC)  },
            0x04 => { self.inr(B)   },
            0x05 => { self.dcr(B)   },
            0x06 => { self.mvi(B)   },
            0x07 => { self.rlc()    },

            // 08
            0x08 => { println!("NOP"); 1 },
            0x09 => { self.dad(BC)  },
            0x0a => { self.ldax(BC) },
            0x0b => { self.dcx(BC)  },
            0x0c => { self.inr(C)   },
            0x0d => { self.dcr(C)   },
            0x0e => { self.mvi(C)   },
            0x0f => { self.rrc()    },

            // 10
            0x10 => { println!("NOP"); 1 },
            0x11 => { self.lxi(DE)  },
            0x12 => { self.stax(DE) },
            0x13 => { self.inx(DE)  },
            0x14 => { self.inr(D)   },
            0x15 => { self.dcr(D)   },
            0x16 => { self.mvi(D)   },
            0x17 => { self.ral()    },

            // 18
            0x18 => { println!("NOP"); 1 },
            0x19 => { self.dad(DE) },
            0x1a => { self.ldax(DE) },
            0x1b => { self.dcx(DE) },
            0x1c => { self.inr(E) },
            0x1d => { self.dcr(E) },
            0x1e => { self.mvi(E) },
            0x1f => { self.rar() },

            // 20
            0x20 => { println!("NOP"); 1 },
            0x21 => { self.lxi(HL) },
            0x22 => { self.shld() },
            0x23 => { self.inx(HL) },
            0x24 => { self.inr(H) },
            0x25 => { self.dcr(H) },
            0x26 => { self.mvi(H) },
            0x27 => {0}, // TODO: After BCD -> DAA

            // 28
            0x28 => { println!("NOP"); 1 },
            0x29 => { self.dad(HL) },
            0x2a => { self.lhld() },
            0x2b => { self.dcx(HL) },
            0x2c => { self.inr(L) },
            0x2d => { self.dcr(L) },
            0x2e => { self.mvi(L) },
            0x2f => { self.cma() },

            // 30
            0x30 => { println!("NOP"); 1 },
            0x31 => { self.lxi(SP) },
            0x32 => { self.sta() },
            0x33 => { self.inx(SP) },
            0x34 => { self.inr_m() },
            0x35 => { self.dcr_m() },
            0x36 => { self.mvi_m() },
            0x37 => { self.stc() },

            // 38
            0x38 => { println!("NOP"); 1 },
            0x39 => { self.dad(SP) },
            0x3a => { self.lda() },
            0x3b => { self.dcx(SP) },
            0x3c => { self.inr(A) },
            0x3d => { self.dcr(A) },
            0x3e => { self.mvi(A) },
            0x3f => { self.cmc() },

            // 40
            0x40 => { self.mov(B, B) },
            0x41 => { self.mov(B, C) },
            0x42 => { self.mov(B, D) },
            0x43 => { self.mov(B, E) },
            0x44 => { self.mov(B, H) },
            0x45 => { self.mov(B, L) },
            0x46 => { self.mov(B, M) },
            0x47 => { self.mov(B, A) },

            // 48
            0x48 => { self.mov(C, B) },
            0x49 => { self.mov(C, C) },
            0x4a => { self.mov(C, D) },
            0x4b => { self.mov(C, E) },
            0x4c => { self.mov(C, H) },
            0x4d => { self.mov(C, L) },
            0x4e => { self.mov(C, M) },
            0x4f => { self.mov(C, A) },

            // 50
            0x50 => { self.mov(D, B) },
            0x51 => { self.mov(D, C) },
            0x52 => { self.mov(D, D) },
            0x53 => { self.mov(D, E) },
            0x54 => { self.mov(D, H) },
            0x55 => { self.mov(D, L) },
            0x56 => { self.mov(D, M) },
            0x57 => { self.mov(D, A) },

            // 58
            0x58 => { self.mov(E, B) },
            0x59 => { self.mov(E, C) },
            0x5a => { self.mov(E, D) },
            0x5b => { self.mov(E, E) },
            0x5c => { self.mov(E, H) },
            0x5d => { self.mov(E, L) },
            0x5e => { self.mov(E, M) },
            0x5f => { self.mov(E, A) },

            // 60
            0x60 => { self.mov(H, B) },
            0x61 => { self.mov(H, C) },
            0x62 => { self.mov(H, D) },
            0x63 => { self.mov(H, E) },
            0x64 => { self.mov(H, H) },
            0x65 => { self.mov(H, L) },
            0x66 => { self.mov(H, M) },
            0x67 => { self.mov(H, A) },

            // 68
            0x68 => { self.mov(L, B) },
            0x69 => { self.mov(L, C) },
            0x6a => { self.mov(L, D) },
            0x6b => { self.mov(L, E) },
            0x6c => { self.mov(L, H) },
            0x6d => { self.mov(L, L) },
            0x6e => { self.mov(L, M) },
            0x6f => { self.mov(L, A) },

            // 70
            0x70 => { self.mov(M, B) },
            0x71 => { self.mov(M, C) },
            0x72 => { self.mov(M, D) },
            0x73 => { self.mov(M, E) },
            0x74 => { self.mov(M, H) },
            0x75 => { self.mov(M, L) },
            0x76 => {1}, // TODO: Add halt 
            0x77 => { self.mov(M, A) },

            // 78
            0x78 => { self.mov(A, B) },
            0x79 => { self.mov(A, C) },
            0x7a => { self.mov(A, D) },
            0x7b => { self.mov(A, E) },
            0x7c => { self.mov(A, H) },
            0x7d => { self.mov(A, L) },
            0x7e => { self.mov(A, M) },
            0x7f => { self.mov(A, A) },

            // 80
            0x80 => { self.add(self.reg[B]) },
            0x81 => { self.add(self.reg[C]) },
            0x82 => { self.add(self.reg[D]) },
            0x83 => { self.add(self.reg[E]) },
            0x84 => { self.add(self.reg[H]) },
            0x85 => { self.add(self.reg[L]) },
            0x86 => { self.add(self.read_byte_at_address(self.reg[HL])) },
            0x87 => { self.add(self.reg.a) },

            // 88
            0x88 => { self.adc(self.reg[B]) },
            0x89 => { self.adc(self.reg[C]) },
            0x8a => { self.adc(self.reg[D]) },
            0x8b => { self.adc(self.reg[E]) },
            0x8c => { self.adc(self.reg[H]) },
            0x8d => { self.adc(self.reg[L]) },
            0x8e => { self.adc(self.read_byte_at_address(self.reg[HL])) },
            0x8f => { self.adc(self.reg.a) },

            // 90
            0x90 => { self.sub(self.reg[B]) },
            0x91 => { self.sub(self.reg[C]) },
            0x92 => { self.sub(self.reg[D]) },
            0x93 => { self.sub(self.reg[E]) },
            0x94 => { self.sub(self.reg[H]) },
            0x95 => { self.sub(self.reg[L]) },
            0x96 => { self.sub(self.read_byte_at_address(self.reg[HL])) },
            0x97 => { self.sub(self.reg.a) },

            // 98
            0x98 => { self.sbb(self.reg[B]) },
            0x99 => { self.sbb(self.reg[C]) },
            0x9a => { self.sbb(self.reg[D]) },
            0x9b => { self.sbb(self.reg[E]) },
            0x9c => { self.sbb(self.reg[H]) },
            0x9d => { self.sbb(self.reg[L]) },
            0x9e => { self.sbb(self.read_byte_at_address(self.reg[HL])) },
            0x9f => { self.sbb(self.reg.a) },

            // a0
            0xa0 => { self.ana(self.reg[B]) },
            0xa1 => { self.ana(self.reg[C]) },
            0xa2 => { self.ana(self.reg[D]) },
            0xa3 => { self.ana(self.reg[E]) },
            0xa4 => { self.ana(self.reg[H]) },
            0xa5 => { self.ana(self.reg[L]) },
            0xa6 => { self.ana(self.read_byte_at_address(self.reg[HL])) },
            0xa7 => { self.ana(self.reg.a) },

            // a8
            0xa8 => { self.xra(self.reg[B]) },
            0xa9 => { self.xra(self.reg[C]) },
            0xaa => { self.xra(self.reg[D]) },
            0xab => { self.xra(self.reg[E]) },
            0xac => { self.xra(self.reg[H]) },
            0xad => { self.xra(self.reg[L]) },
            0xae => { self.xra(self.read_byte_at_address(self.reg[HL])) },
            0xaf => { self.xra(self.reg.a) },

            // b0
            0xb0 => { self.ora(self.reg[B]) },
            0xb1 => { self.ora(self.reg[C]) },
            0xb2 => { self.ora(self.reg[D]) },
            0xb3 => { self.ora(self.reg[E]) },
            0xb4 => { self.ora(self.reg[H]) },
            0xb5 => { self.ora(self.reg[L]) },
            0xb6 => { self.ora(self.read_byte_at_address(self.reg[HL])) },
            0xb7 => { self.ora(self.reg.a) },

            // b8
            0xb8 => { self.cmp(self.reg[B]) },
            0xb9 => { self.cmp(self.reg[C]) },
            0xba => { self.cmp(self.reg[D]) },
            0xbb => { self.cmp(self.reg[E]) },
            0xbc => { self.cmp(self.reg[H]) },
            0xbd => { self.cmp(self.reg[L]) },
            0xbe => { self.cmp(self.read_byte_at_address(self.reg[HL])) },
            0xbf => { self.cmp(self.reg.a) },

            // c0
            0xc0 => {0}, // If not 0 RET
            0xc1 => {0}, // POP B
            0xc2 => {0}, // JNZ addr
            0xc3 => {0}, // JMP addr
            0xc4 => {0}, // if NZ CALL addr
            0xc5 => {0}, // PUSH B
            0xc6 => {0}, // ADI (add immediate to acc)
            0xc7 => {0}, // CALL $0 (??)

            // c8
            0xc8 => {0}, // If Z RET
            0xc9 => {0}, // RET
            0xca => {0}, // JZ addr
            0xcb => {0}, // NOP
            0xcc => {0}, // if Z CALL addr
            0xcd => {0}, // CALL addr
            0xce => {0}, // ACI (add immediate byte & carry to acc)
            0xcf => {0}, // CALL $8

            // d0
            0xd0 => {0}, // if !C RET
            0xd1 => {0}, // POP D
            0xd2 => {0}, // JNC addr
            0xd3 => {0}, // OUT (??)
            0xd4 => {0}, // if !C CALL addr
            0xd5 => {0}, // PUSH D
            0xd6 => {0}, // subtract immediate byte from acc & set all flags
            0xd7 => {0}, // CALL $10

            // d8
            0xd8 => {0}, // if C RET
            0xd9 => {0}, // NOP
            0xda => {0}, // if C jmp addr
            0xdb => {0}, // IN (??)
            0xdc => {0}, // if C CALL addr
            0xdd => {0}, // NOP
            0xde => {0}, // sutract immediate byte & carry from acc & set all flags
            0xdf => {0}, // CALL $18 (??)

            // e0
            0xe0 => {0}, // if PO RET
            0xe1 => {0}, // POP H
            0xe2 => {0}, // JPO addr
            0xe3 => {0}, // XTHL
            0xe4 => {0}, // if PO call addr
            0xe5 => {0}, // PUSH H
            0xe6 => {0}, // bitwise AND acc with immediate byte & set flags
            0xe7 => {0}, // CALL $20

            // e8
            0xe8 => {0}, // if PE RET
            0xe9 => {0}, // PCHL
            0xea => {0}, // if PE move immediate word to PC
            0xeb => {0}, // XCHG
            0xec => {0}, // if PE call addr
            0xed => {0}, // NOP
            0xee => {0}, // bitwise XOR immediate byte with acc and set flags
            0xef => {0}, // CALL $28

            // f0
            0xf0 => {0}, // if P RET
            0xf1 => {0}, // POP psw
            0xf2 => {0}, // if P jmp addr
            0xf3 => {0}, // DI (??)
            0xf4 => {0}, // if P jmp addr
            0xf5 => {0}, // PUSH PSW
            0xf6 => {0}, // bitwise OR immediate byte with acc and set flags
            0xf7 => {0}, // CALL $30

            // f8
            0xf8 => {0}, // if M, RET
            0xf9 => {0}, // SPHL
            0xfa => {0}, // if M jmp addr
            0xfb => {0}, // EI (??)
            0xfc => {0}, // if M call addr
            0xfd => {0}, // NOP
            0xfe => {0}, // compare acc to immediate byte & set vlags
            0xff => {0}, // CALL $38
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
        cpu.memory[6] = 0x87;
        cpu.memory[7] = 0x86;

        cpu.reg[A] = 0b1;
        cpu.reg[B] = 0b1;
        cpu.tick();
        assert_eq!(cpu.reg[A], 0b10);
        assert_eq!(cpu.reg.get_flag(Zero), false);
        assert_eq!(cpu.reg.get_flag(Sign), false);
        assert_eq!(cpu.reg.get_flag(Parity), false);
        assert_eq!(cpu.reg.get_flag(Carry), false);
        assert_eq!(cpu.reg.get_flag(AuxCarry), false);

        cpu.reg[C] = 0x1;
        cpu.tick();
        assert_eq!(cpu.reg[A], 0b11);
        assert_eq!(cpu.reg.get_flag(Zero), false);
        assert_eq!(cpu.reg.get_flag(Sign), false);
        assert_eq!(cpu.reg.get_flag(Parity), true);
        assert_eq!(cpu.reg.get_flag(Carry), false);
        assert_eq!(cpu.reg.get_flag(AuxCarry), false);

        cpu.reg[D] = 0x1;
        cpu.tick();
        assert_eq!(cpu.reg[A], 0b100);
        assert_eq!(cpu.reg.get_flag(Zero), false);
        assert_eq!(cpu.reg.get_flag(Sign), false);
        assert_eq!(cpu.reg.get_flag(Parity), false);
        assert_eq!(cpu.reg.get_flag(Carry), false);
        assert_eq!(cpu.reg.get_flag(AuxCarry), false);

        cpu.reg[E] = 0x1;
        cpu.tick();
        assert_eq!(cpu.reg[A], 0b101);
        assert_eq!(cpu.reg.get_flag(Zero), false);
        assert_eq!(cpu.reg.get_flag(Sign), false);
        assert_eq!(cpu.reg.get_flag(Parity), true);
        assert_eq!(cpu.reg.get_flag(Carry), false);
        assert_eq!(cpu.reg.get_flag(AuxCarry), false);

        cpu.reg[H] = 0x1;
        cpu.tick();
        assert_eq!(cpu.reg[A], 0b110);
        assert_eq!(cpu.reg.get_flag(Zero), false);
        assert_eq!(cpu.reg.get_flag(Sign), false);
        assert_eq!(cpu.reg.get_flag(Parity), true);
        assert_eq!(cpu.reg.get_flag(Carry), false);
        assert_eq!(cpu.reg.get_flag(AuxCarry), false);

        cpu.reg[L] = 0x1;
        cpu.tick();
        assert_eq!(cpu.reg[A], 0b111);
        assert_eq!(cpu.reg.get_flag(Zero), false);
        assert_eq!(cpu.reg.get_flag(Sign), false);
        assert_eq!(cpu.reg.get_flag(Parity), false);
        assert_eq!(cpu.reg.get_flag(Carry), false);
        assert_eq!(cpu.reg.get_flag(AuxCarry), false);

        cpu.tick();
        assert_eq!(cpu.reg[A], 0b1110);
        assert_eq!(cpu.reg.get_flag(Zero), false);
        assert_eq!(cpu.reg.get_flag(Sign), false);
        assert_eq!(cpu.reg.get_flag(Parity), false);
        assert_eq!(cpu.reg.get_flag(Carry), false);
        assert_eq!(cpu.reg.get_flag(AuxCarry), false);

        cpu.reg[A] = 0x0;
        cpu.memory[0x1001] = 0xFF;
        cpu.reg[HL] = 0x1001;

        cpu.tick();

        assert_eq!(cpu.reg[A], 0xFF);

        assert_eq!(cpu.reg.get_flag(Zero), false);
        assert_eq!(cpu.reg.get_flag(Sign), true);
        assert_eq!(cpu.reg.get_flag(Parity), true);
        assert_eq!(cpu.reg.get_flag(Carry), false);
        assert_eq!(cpu.reg.get_flag(AuxCarry), true);
    }

    #[test]
    fn test_pc_increment() { 
        let mut cpu = CPU::new();

        cpu.reg[A] = 0x1;
        cpu.reg[B] = 0x1;

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
        assert_eq!(cpu.reg[BC], 0x7635);

        cpu.tick();
        assert_eq!(cpu.reg[DE], 0x7635);

        cpu.tick();
        assert_eq!(cpu.reg[HL], 0x7635);

        cpu.tick();
        assert_eq!(cpu.reg.sp, 0x7635);
    }

    #[test]
    fn test_stax() {
        let mut cpu = CPU::new();

        cpu.memory[cpu.reg.pc] = 0x2;
        cpu.reg[A] = 0x42;
        cpu.reg[BC] = 0xF00F;

        cpu.tick();

        assert_eq!(cpu.memory[0xF00F], 0x42);

        cpu.memory[cpu.reg.pc] = 0x12;
        cpu.reg[A] = 0x82;
        cpu.reg[DE] = 0xEA3;

        cpu.tick();

        assert_eq!(cpu.memory[0xEA3], 0x82);
    }

    #[test]
    fn test_inx() {
        let mut cpu = CPU::new();

        cpu.memory[0] = 0x3;
        cpu.memory[1] = 0x13;
        cpu.memory[2] = 0x23;

        cpu.reg[BC] = 0;
        cpu.reg[DE] = 0;
        cpu.reg[HL] = 0;

        cpu.tick();
        cpu.tick();
        cpu.tick();

        assert_eq!(cpu.reg[BC], 0x1);
        assert_eq!(cpu.reg[DE], 0x1);
        assert_eq!(cpu.reg[HL], 0x1);
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

        assert_eq!(cpu.reg[A], 1);
        assert_eq!(cpu.reg[B], 1);
        assert_eq!(cpu.reg[C], 1);
        assert_eq!(cpu.reg[D], 1);
        assert_eq!(cpu.reg[E], 1);
        assert_eq!(cpu.reg[H], 1);
        assert_eq!(cpu.reg[L], 1);

        cpu.reg[HL] = 8;
        cpu.tick();
        assert_eq!(cpu.memory[8], 1);
    }

    #[test]
    fn test_dcr() {
        let mut cpu = CPU::new();

        cpu.reg[A] = 2;
        cpu.reg[B] = 2;
        cpu.reg[C] = 2;
        cpu.reg[D] = 2;
        cpu.reg[E] = 2;
        cpu.reg[H] = 2;
        cpu.reg[L] = 2;
        
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

        assert_eq!(cpu.reg[A], 1);
        assert_eq!(cpu.reg[B], 1);
        assert_eq!(cpu.reg[C], 1);
        assert_eq!(cpu.reg[D], 1);
        assert_eq!(cpu.reg[E], 1);
        assert_eq!(cpu.reg[H], 1);
        assert_eq!(cpu.reg[L], 1);

        cpu.memory[8] = 2;
        cpu.reg[HL] = 8;
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

        assert_eq!(cpu.reg[A], 0x10);
        assert_eq!(cpu.reg[B], 0x10);
        assert_eq!(cpu.reg[C], 0x10);
        assert_eq!(cpu.reg[D], 0x10);
        assert_eq!(cpu.reg[E], 0x10);
        assert_eq!(cpu.reg[H], 0x10);
        assert_eq!(cpu.reg[L], 0x10);

        cpu.reg[HL] = 0xFF;
        cpu.tick();
        assert_eq!(cpu.memory[cpu.reg[HL]], 0x10);
    }

    #[test]
    fn test_dad() {
        let mut cpu = CPU::new();
        cpu.memory[0] = 0x9;
        cpu.memory[1] = 0x19;
        cpu.memory[2] = 0x29;
        cpu.reg[BC] = 0x1;
        cpu.reg[DE] = 0x1;
        cpu.reg[HL] = 0x1;

        cpu.tick();
        cpu.tick();
        cpu.tick();

        assert_eq!(cpu.reg[HL], 0x6);

        cpu.memory[3] = 0x39;
        cpu.reg.sp = 1;
        cpu.tick();
        assert_eq!(cpu.reg[HL], 0x7);
    }

    #[test]
    fn test_ldax() {
        let mut cpu = CPU::new();
        cpu.memory[0] = 0x0A;
        cpu.memory[1] = 0x1A;

        cpu.memory[0xF1] = 0x10;
        cpu.memory[0xF2] = 0x20;

        cpu.reg[BC] = 0xF1;
        cpu.reg[DE] = 0xF2;

        cpu.tick();
        assert_eq!(cpu.reg[A], 0x10);
        cpu.tick();
        assert_eq!(cpu.reg[A], 0x20);
    }

    #[test]
    fn test_dcx() {
        let mut cpu = CPU::new();
        cpu.memory[0] = 0x0B;
        cpu.memory[1] = 0x1B;
        cpu.memory[2] = 0x2B;
        cpu.memory[3] = 0x3B;

        cpu.reg[BC] = 1;
        cpu.reg[DE] = 1;
        cpu.reg[HL] = 1;
        cpu.reg.sp = 1;

        for x in 0..4 {
            cpu.tick();
        }

        assert_eq!(cpu.reg[BC], 0);
        assert_eq!(cpu.reg[DE], 0);
        assert_eq!(cpu.reg[HL], 0);
        assert_eq!(cpu.reg.sp, 0);
    }

    #[test]
    fn test_rlc() {
        let mut cpu = CPU::new();
        cpu.memory[0] = 0x07;
        cpu.reg[A] = 0b10101010;

        cpu.tick();
        assert_eq!(cpu.reg[A], 0b01010101);
        assert_eq!(cpu.reg.get_flag(Carry), true);

        cpu.reg.pc = 0;
        cpu.tick();

        assert_eq!(cpu.reg[A], 0b10101010);
        assert_eq!(cpu.reg.get_flag(Carry), false);
    }

    #[test]
    fn test_rrc() {
        let mut cpu = CPU::new();
        cpu.memory[0] = 0x0F;
        cpu.reg[A] = 0b10000001;

        cpu.tick();
        assert_eq!(cpu.reg[A], 0b11000000);
        assert_eq!(cpu.reg.get_flag(Carry), true);

        cpu.reg.pc = 0;
        cpu.tick();
        cpu.reg[A] = 0b01100000;
         assert_eq!(cpu.reg.get_flag(Carry), false);
    }

    #[test]
    fn test_ral() {
        let mut cpu = CPU::new();
        cpu.memory[0] = 0x17;
        cpu.reg[A] = 0b10101010;

        cpu.tick();
        assert_eq!(cpu.reg[A], 0b01010100);
        assert_eq!(cpu.reg.get_flag(Carry), true);

        cpu.reg.pc = 0;
        cpu.tick();

        assert_eq!(cpu.reg[A], 0b10101001);
        assert_eq!(cpu.reg.get_flag(Carry), false);
    }

    #[test]
    fn test_rar() {
        let mut cpu = CPU::new();
        cpu.memory[0] = 0x1F;
        cpu.reg[A] = 0b10000001;

        cpu.tick();
        assert_eq!(cpu.reg[A], 0b01000000);
        assert_eq!(cpu.reg.get_flag(Carry), true);

        cpu.reg.pc = 0;
        cpu.tick();

        assert_eq!(cpu.reg[A], 0b10100000);
    }

    #[test]
    fn test_shld() {
        let mut cpu = CPU::new();
        cpu.memory[0] = 0x22;
        cpu.memory[1] = 0xAA;
        cpu.memory[2] = 0xBB;
        cpu.reg[L] = 0xCC;
        cpu.reg[H] = 0xDD;

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
        cpu.reg[L] = 0xCC;
        cpu.reg[H] = 0xDD;

        cpu.tick();

        assert_eq!(cpu.reg[L], 0xEE);
        assert_eq!(cpu.reg[H], 0xFF);
    }

    #[test]
    fn test_cma() {
        let mut cpu = CPU::new();
        cpu.memory[0] = 0x2f;
        cpu.reg[A] = 0b00000001;
        cpu.tick();
        assert_eq!(cpu.reg[A], 0b11111110);
    }

    #[test]
    fn test_sta() {
        let mut cpu = CPU::new();
        cpu.memory[0] = 0x32;
        cpu.memory[1] = 0xBB;
        cpu.memory[2] = 0xAA;
        cpu.reg[A] = 0xFF;
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
        assert_eq!(cpu.reg.get_flag(Carry), true);
    }

    #[test]
    fn test_lda() {
        let mut cpu = CPU::new();
        cpu.memory[0] = 0x3a;
        cpu.memory[1] = 0xFF;
        cpu.memory[2] = 0x00;
        cpu.memory[0x00FF] = 0xAA;
        cpu.tick();
        assert_eq!(cpu.reg[A], 0xAA);
    }

    #[test]
    fn test_cmc() {
        let mut cpu = CPU::new();
        cpu.memory[0] = 0x3F;
        cpu.memory[1] = 0x3F;
        cpu.tick();
        assert_eq!(cpu.reg.get_flag(Carry), true);
        cpu.tick();
        assert_eq!(cpu.reg.get_flag(Carry), false);
    }

    #[test]
    fn test_mov() {
        let mut cpu = CPU::new();
        let mut index = 0;

        for x in 0x40..0x80 {
            cpu.memory[index] = x;
            index += 1;
        }

        let registers = [B, C, D, E, H, L, M, A];

        for &dest_register in registers.iter() {
            for &source_register in registers.iter() {
                // println!("Source: {:?}, Dest: {:?}", source_register, dest_register);
                if source_register == M {
                    cpu.reg[HL] = 0xFFFE;
                    cpu.memory[cpu.reg[HL]] = 0x1;
                    // println!("Setting memory at {:04X} to 1", cpu.reg[HL]);
                }
                if source_register != M {
                    cpu.reg[source_register] = 0x1;
                    // println!("Setting SR to 1");
                    // println!("SR pre-tick: {:?}", cpu.reg[source_register]);
                }

                cpu.tick();

                if source_register != M && dest_register != M {
                    if source_register == M {
                    // println!("DR post-tick: {:?}", cpu.reg[dest_register]);
                    // println!("M@HL({:04X}): {} vs V->{:?}: {}", cpu.reg[HL], cpu.memory[cpu.reg[HL]], dest_register, cpu.reg[dest_register]);
                    assert_eq!(cpu.memory[0xFFFE], cpu.reg[dest_register]);
                    } 
                    else if dest_register == M {
                        // println!("SR post-tick: {:?}", cpu.reg[source_register]);
                        // println!("Memory at HL({:?}): {:04X}", cpu.reg[HL], cpu.memory[cpu.reg[HL]]);
                        assert_eq!(cpu.memory[cpu.reg[HL]], cpu.reg[source_register]);
                        
                    } 
                    else {
                        // println!("DR post-tick: {:?}", cpu.reg[dest_register]);
                        // println!("SR post-tick: {:?}", cpu.reg[source_register]);
                        assert_eq!(cpu.reg[dest_register], cpu.reg[source_register]);
                    }
                }
            }
        }
    }

    #[test]
    fn test_adc() {
        let mut cpu = CPU::new();

        cpu.memory[0] = 0x80;
        cpu.memory[1] = 0x88;
        cpu.memory[2] = 0x89;
        cpu.memory[3] = 0x8a;
        cpu.memory[4] = 0x8b;
        cpu.memory[5] = 0x8c;
        cpu.memory[6] = 0x8d;
        cpu.memory[7] = 0x8e;
        cpu.memory[8] = 0x8f;

        cpu.reg[A] = 0b11111111;
        cpu.reg[B] = 0b00000001;

        cpu.tick();
        assert_eq!(cpu.reg[A], 0b00000000);

        cpu.tick();
        assert_eq!(cpu.reg[A], 0b00000010);

        cpu.reg[C] = 1;
        cpu.reg[D] = 1;
        cpu.reg[E] = 1;
        cpu.reg[H] = 1;
        cpu.reg[L] = 1;
        cpu.memory[cpu.reg[HL]] = 1;

        for x in 0..6 {
            cpu.tick();
        }

        assert_eq!(cpu.reg[A], 0b1000);

        cpu.tick();

        assert_eq!(cpu.reg[A], 0b10000);
    }

    #[test]
    fn test_sub() {
        let mut cpu = CPU::new();
        let mut index = 0;
    
        for x in 0x90..0x98 {
            cpu.memory[index] = x;
            index += 1;
        }

        cpu.reg[A] = 0b0000000;
        cpu.reg[B] = 0b0000001;
        cpu.reg[C] = 0b0000001;
        cpu.reg[D] = 0b0000001;
        cpu.reg[E] = 0b0000001;
        cpu.reg[H] = 0b0000001;
        cpu.reg[L] = 0b0000001;
        cpu.memory[cpu.reg[HL]] = 0b0000001;

        cpu.tick();
        assert_eq!(cpu.reg[A], 0b11111111);
        assert_eq!(cpu.reg.get_flag(Carry), true);

        for x in 0..6 {
            cpu.tick();
        }
        
        assert_eq!(cpu.reg[A], 0b11111111 - 6);

        cpu.tick();

        assert_eq!(cpu.reg[A], 0);
    }

    #[test]
    fn test_sbb() {
        let mut cpu = CPU::new();
        let mut index = 0;
    
        for x in 0x98..0xa0 {
            cpu.memory[index] = x;
            index += 1;
        }

        cpu.reg[A] = 0b0000000;
        cpu.reg[B] = 0b0000001;
        cpu.reg[C] = 0b0000001;
        cpu.reg[D] = 0b0000001;
        cpu.reg[E] = 0b0000001;
        cpu.reg[H] = 0b0000001;
        cpu.reg[L] = 0b0000001;
        cpu.memory[cpu.reg[HL]] = 0b0000001;

        cpu.tick();
        assert_eq!(cpu.reg[A], 0b11111111);
        assert_eq!(cpu.reg.get_flag(Carry), true);

        for x in 0..6 {
            cpu.tick();
        }
        
        assert_eq!(cpu.reg[A], 0b11111111 - 7);

        cpu.tick();

        assert_eq!(cpu.reg[A], 0);
    }
}