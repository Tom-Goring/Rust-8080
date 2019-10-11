pub type Address = u16;
pub type Word = u16;
pub type Byte = u8;

const CYCLES: [u8; 256] = [
    4, 10, 7, 5, 5, 5, 7, 4, 4, 10, 7, 5, 5, 5, 7, 4, //0x00..0x0f
    4, 10, 7, 5, 5, 5, 7, 4, 4, 10, 7, 5, 5, 5, 7, 4, //0x10..0x1f
    4, 10, 16, 5, 5, 5, 7, 4, 4, 10, 16, 5, 5, 5, 7, 4, //etc
    4, 10, 13, 5, 10, 10, 10, 4, 4, 10, 13, 5, 5, 5, 7, 4,

    5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5, //0x40..0x4f
    5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5,
    5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5,
    7, 7, 7, 7, 7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 7, 5,

    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4, //0x80..8x4f
    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4,
    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4,
    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4,

    11, 10, 10, 10, 17, 11, 7, 11, 11, 10, 10, 10, 10, 17, 7, 11, //0xc0..0xcf
    11, 10, 10, 10, 17, 11, 7, 11, 11, 10, 10, 10, 10, 17, 7, 11,
    11, 10, 10, 18, 17, 11, 7, 11, 11, 5, 10, 5, 17, 17, 7, 11,
    11, 10, 10, 4, 17, 11, 7, 11, 11, 5, 10, 4, 17, 17, 7, 11,
];

use super::register::Register;
use super::memory::Memory;
use super::machine_interface::Machine;

use crate::disassembler::disassemble_8080_op;

use super::register::Reg8;
use super::register::Reg16;
use super::register::Reg8::{A, B, C, D, E, H, L, M};
use super::register::Reg16::{BC, DE, HL, SP, PC, PSW};
use super::register::Flag::{Carry, Parity, Sign, AuxCarry, Zero};

pub struct CPU {
    pub reg: Register,
    pub memory: Memory,
    pub input_ports: [u8; 4],
    pub output_ports: [u8; 5],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            reg: Register::new(),
            memory: Memory::new(),
            input_ports: [0; 4],
            output_ports: [0; 5],
        }
    }

    pub fn tick(&mut self, machine: &mut impl Machine) -> Byte {
        let opcode = self.fetch();
        disassemble_8080_op(&self.memory, self.reg.pc);
        self.reg.pc += self.execute_opcode(opcode, machine);
        println!("{}", CYCLES[opcode as usize]);
        CYCLES[opcode as usize]
    }

    pub fn fetch(&self) -> Byte {
        self.memory[self.reg.pc]
    }

    pub fn trigger_interrupt(&mut self, interrupt_num: Word) {
        self.reg[SP] -= 2;
        self.write_word_to_memory(self.reg[SP], self.reg[PC]);
        self.reg[PC] = 8 * interrupt_num;
    }
}

impl CPU { // Helper functions
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

impl CPU { // ARITHMETIC GROUP
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

    fn inr(&mut self, x: Reg8) -> Word {
        self.reg[x] += 1;
        self.set_zspac_flags_on_byte(self.reg[x]);
        1
    }

    fn inr_m(&mut self) -> Word {
        self.memory[self.reg[HL]] += 1;
        self.set_zspac_flags_on_byte(self.memory[self.reg[HL]]);
        1
    }

    fn dcr(&mut self, x: Reg8) -> Word {
        self.reg[x] = self.reg[x].wrapping_sub(1);
        self.set_zspac_flags_on_byte(self.reg[x]);
        1
    }

    fn dcr_m(&mut self) -> Word {
        self.memory[self.reg[HL]] = self.memory[self.reg[HL]].wrapping_sub(1);
        self.set_zspac_flags_on_byte(self.memory[self.reg[HL]]);
        1
    }

    fn inx(&mut self, x: Reg16) -> Word {
        self.reg[x] += 1;
        1
    }

    fn dcx(&mut self, x: Reg16) -> Word {
        self.reg[x] -= 1;
        1
    }

    fn dad(&mut self, x: Reg16) -> Word {
        self.reg[HL] += self.reg[x];
        1
    }

    fn adi(&mut self) -> Word {
        let (result, overflow) = self.reg[A].overflowing_add(self.read_byte_immediate());
        self.set_flags_on_result(result, overflow);
        self.reg[A] = result;
        2
    }

    fn aci(&mut self) -> Word {
        let (mut result, overflow) = self.reg[A].overflowing_add(self.read_byte_immediate());
        self.set_flags_on_result(result, overflow);
        if self.reg.get_flag(Carry) {
            result += 1;
        }
        self.reg[A] = result;
        2
    }

    fn sui(&mut self) -> Word {
        let (result, overflow) = self.reg[A].overflowing_sub(self.read_byte_immediate());
        self.set_flags_on_result(result, overflow);
        2
    }

    fn sbi(&mut self) -> Word {
        let (mut result, overflow) = self.reg[A].overflowing_sub(self.read_byte_immediate());
        self.set_flags_on_result(result, overflow);
        if self.reg.get_flag(Carry) {
            result -= 1;
        }
        self.reg[A] = result;
        2
    }
}

impl CPU { // LOGICAL GROUP
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
        self.reg.set_flag(Zero, self.reg[A] == byte);
        1
    }

    fn cma(&mut self) -> Word {
        self.reg[A] = !(self.reg[A]);
        1
    }

    fn cmc(&mut self) -> Word {
        self.reg.set_flag(Carry, !self.reg.get_flag(Carry));
        1
    }

    fn stc(&mut self) -> Word {
        self.reg.set_flag(Carry, true);
        1
    }

    fn rlc(&mut self) -> Word {
        self.reg.set_flag(Carry, self.reg[A] >> 7 != 0);
        self.reg[A] = self.reg[A] << 1 | self.reg[A] >> 7;
        1
    }

    fn rrc(&mut self) -> Word {
        self.reg.set_flag(Carry, self.reg[A] << 7 != 0);
        self.reg[A] = self.reg[A] << 7 | self.reg[A] >> 1;
        1
    }

    fn ral(&mut self) -> Word {
        let set_flag = self.reg[A] >> 7 != 0;
        self.reg[A] = self.reg[A] << 1;
        if self.reg.get_flag(Carry) { self.reg[A] |= 0b00000001; }
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

    fn ani(&mut self) -> Word {
        self.reg[A] &= self.read_byte_immediate();
        self.set_flags_on_result(self.reg[A], false);
        2
    }

    fn xri(&mut self) -> Word {
        self.reg[A] ^= self.read_byte_immediate();
        self.set_flags_on_result(self.reg[A], false);
        2
    }

    fn ori(&mut self) -> Word {
        self.reg[A] |= self.read_byte_immediate();
        self.set_flags_on_result(self.reg[A], false);
        2
    }

    fn cpi(&mut self) -> Word {
        let byte = self.read_byte_immediate();
        self.reg.set_flag(Carry, self.reg[A] < byte);
        self.reg.set_flag(Zero, self.reg[A] == byte);
        2
    }
}

impl CPU { // DATA TRANSFER GROUP
    fn mov(&mut self, dest: Reg8, src: Reg8) -> Word {
        if dest == M {
            self.memory[self.reg[HL]] = self.reg[src];
        } else if src == M {
            self.reg[dest] = self.memory[self.reg[HL]];
        } else {
            self.reg[dest] = self.reg[src];
        }
        1
    }

    fn mvi(&mut self, x: Reg8) -> Word { // TODO: Combine mvi & mvi_m functions
        self.reg[x] = self.read_byte_immediate();
        2
    }

    fn mvi_m(&mut self) -> Word {
        self.memory[self.reg[HL]] = self.read_byte_immediate();
        2
    }

    fn lda(&mut self) -> Word {
        self.reg[A] = self.memory[self.read_word_immediate()];
        3
    }

    fn ldax(&mut self, x: Reg16) -> Word {
        self.reg[A] = self.memory[self.reg[x]];
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

    fn lxi(&mut self, x: Reg16) -> Word {
        self.reg[x] = self.read_word_immediate();
        3
    }

    fn stax(&mut self, x: Reg16) -> Word {
        self.memory[self.reg[x]] = self.reg[A];
        1
    }

    fn sta(&mut self) -> Word {
        let address = self.read_word_immediate();
        self.memory[address] = self.reg[A];
        3
    }
}

impl CPU { // BRANCH GROUP
    fn call(&mut self) -> Word {
        self.reg[SP] -= 2;
        self.write_word_to_memory(self.reg[SP], self.reg[PC] + 3);
        self.jmp();
        0
    }

    fn cc(&mut self) -> Word {
        if self.reg.get_flag(Carry) {
            self.call()
        }
        else {
            3
        }
    }

    fn cnc(&mut self) -> Word {
        if !self.reg.get_flag(Carry) {
            self.call()
        }
        else {
            3
        }
    }

    fn cp(&mut self) -> Word {
        if self.reg.get_flag(Sign) {
            self.call()
        }
        else {
            3
        }
    }

    fn cm(&mut self) -> Word {
        if !self.reg.get_flag(Sign) {
            self.call()
        }
        else {
            3
        }
    }

    fn cz(&mut self) -> Word {
        if self.reg.get_flag(Zero) {
            self.call()
        }
        else {
            3
        }
    }

    fn cnz(&mut self) -> Word {
        if !self.reg.get_flag(Zero) {
            self.call()
        }
        else {
            3
        }
    }

    fn cpe(&mut self) -> Word {
        if self.reg.get_flag(Parity) {
            self.call()
        }
        else {
            3
        }
    }

    fn cpo(&mut self) -> Word {
        if !self.reg.get_flag(Parity) {
            self.call()
        }
        else {
            3
        }
    }

    fn ret(&mut self) -> Word {
        self.reg[PC] = self.read_word_at_address(self.reg[SP]);
        self.reg[SP] += 2;
        0
    }

    fn rc(&mut self) -> Word {
        if self.reg.get_flag(Carry) {
            self.ret()
        }
        else {
            1
        }
    }  

    fn rnc(&mut self) -> Word {
        if !self.reg.get_flag(Carry) {
            self.ret()
        }
        else {
            1
        }
    }

    fn rp(&mut self) -> Word {
        if self.reg.get_flag(Sign) {
            self.ret()
        }
        else {
            1
        }
    }

    fn rm(&mut self) -> Word {
        if !self.reg.get_flag(Sign) {
            self.ret()
        }
        else {
            1
        }
    }

    fn rz(&mut self) -> Word {
        if self.reg.get_flag(Zero) {
            self.ret()
        }
        else {
            1
        }
    }

    fn rnz(&mut self) -> Word {
        if !self.reg.get_flag(Zero) {
            self.ret()
        }
        else {
            1
        }
    }

    fn rpe(&mut self) -> Word {
        if self.reg.get_flag(Parity) {
            self.ret()
        }
        else {
            1
        }
    }

    fn rpo(&mut self) -> Word {
        if !self.reg.get_flag(Parity) {
            self.ret()
        }
        else {
            1
        }
    }

    fn jmp(&mut self) -> Word {
        self.reg[PC] = self.read_word_immediate();
        0
    }

    fn jc(&mut self) -> Word {
        if self.reg.get_flag(Carry) {
            self.jmp()
        }
        else {
            3
        }
    }

    fn jnc(&mut self) -> Word {
        if !self.reg.get_flag(Carry) {
            self.jmp()
        }
        else {
            3
        }
    }

    fn jp(&mut self) -> Word {
        if self.reg.get_flag(Sign) {
            self.jmp()
        }
        else {
            3
        }
    }

    fn jm(&mut self) -> Word {
        if !self.reg.get_flag(Sign) {
            self.jmp()
        }
        else {
            3
        }
    }

    fn jz(&mut self) -> Word {
        if self.reg.get_flag(Zero) {
            self.jmp()
        }
        else {
            3
        }
    }

    fn jnz(&mut self) -> Word {
        if !self.reg.get_flag(Zero) {
            self.jmp()
        } else {
            3
        }
    }

    fn jpe(&mut self) -> Word {
        if self.reg.get_flag(Parity) {
            self.jmp()
        }
        else {
            3
        }
    }

    fn jpo(&mut self) -> Word {
        if !self.reg.get_flag(Parity) {
            self.jmp()
        }
        else {
            3
        }
    }

    fn rst(&mut self, address: Address) -> Word {
        self.reg[SP] -= 2;
        self.write_word_to_memory(self.reg[SP], self.reg[PC] + 3);
        self.reg[PC] = address;
        0
    }
}

impl CPU { // STACK GROUP
    fn push(&mut self, x: Reg16) -> Word {
        self.reg[SP] -= 2;
        self.write_word_to_memory(self.reg[SP], self.reg[x]);
        1
    }

    fn pop(&mut self, x: Reg16) -> Word {
        self.reg[x] = self.read_word_at_address(self.reg[SP]);
        self.reg[SP] += 2;
        1
    }

    fn xhtl(&mut self) -> Word {
        let tmp_h = self.reg[H];
        let tmp_l = self.reg[L];
        self.reg[H] = self.memory[self.reg[SP]];
        self.reg[L] = self.memory[self.reg[SP] + 1];
        self.memory[self.reg[SP]] = tmp_h;
        self.memory[self.reg[SP] + 1] = tmp_l;
        1
    }

    fn pchl(&mut self) -> Word {
        self.reg[PC] = self.reg[HL];
        1
    }

    fn xchg(&mut self) -> Word {
        let tmp_hl = self.reg[HL];
        self.reg[HL] = self.reg[DE];
        self.reg[DE] = tmp_hl;
        1
    }

    fn sphl(&mut self) -> Word {
        self.reg[SP] = self.reg[HL];
        1
    }
}

impl CPU {
    pub fn execute_opcode(&mut self, opcode: Byte, machine: &mut impl Machine) -> Word {
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
            0x87 => { self.add(self.reg[A]) },

            // 88
            0x88 => { self.adc(self.reg[B]) },
            0x89 => { self.adc(self.reg[C]) },
            0x8a => { self.adc(self.reg[D]) },
            0x8b => { self.adc(self.reg[E]) },
            0x8c => { self.adc(self.reg[H]) },
            0x8d => { self.adc(self.reg[L]) },
            0x8e => { self.adc(self.read_byte_at_address(self.reg[HL])) },
            0x8f => { self.adc(self.reg[A]) },

            // 90
            0x90 => { self.sub(self.reg[B]) },
            0x91 => { self.sub(self.reg[C]) },
            0x92 => { self.sub(self.reg[D]) },
            0x93 => { self.sub(self.reg[E]) },
            0x94 => { self.sub(self.reg[H]) },
            0x95 => { self.sub(self.reg[L]) },
            0x96 => { self.sub(self.read_byte_at_address(self.reg[HL])) },
            0x97 => { self.sub(self.reg[A]) },

            // 98
            0x98 => { self.sbb(self.reg[B]) },
            0x99 => { self.sbb(self.reg[C]) },
            0x9a => { self.sbb(self.reg[D]) },
            0x9b => { self.sbb(self.reg[E]) },
            0x9c => { self.sbb(self.reg[H]) },
            0x9d => { self.sbb(self.reg[L]) },
            0x9e => { self.sbb(self.read_byte_at_address(self.reg[HL])) },
            0x9f => { self.sbb(self.reg[A]) },

            // a0
            0xa0 => { self.ana(self.reg[B]) },
            0xa1 => { self.ana(self.reg[C]) },
            0xa2 => { self.ana(self.reg[D]) },
            0xa3 => { self.ana(self.reg[E]) },
            0xa4 => { self.ana(self.reg[H]) },
            0xa5 => { self.ana(self.reg[L]) },
            0xa6 => { self.ana(self.read_byte_at_address(self.reg[HL])) },
            0xa7 => { self.ana(self.reg[A]) },

            // a8
            0xa8 => { self.xra(self.reg[B]) },
            0xa9 => { self.xra(self.reg[C]) },
            0xaa => { self.xra(self.reg[D]) },
            0xab => { self.xra(self.reg[E]) },
            0xac => { self.xra(self.reg[H]) },
            0xad => { self.xra(self.reg[L]) },
            0xae => { self.xra(self.read_byte_at_address(self.reg[HL])) },
            0xaf => { self.xra(self.reg[A]) },

            // b0
            0xb0 => { self.ora(self.reg[B]) },
            0xb1 => { self.ora(self.reg[C]) },
            0xb2 => { self.ora(self.reg[D]) },
            0xb3 => { self.ora(self.reg[E]) },
            0xb4 => { self.ora(self.reg[H]) },
            0xb5 => { self.ora(self.reg[L]) },
            0xb6 => { self.ora(self.read_byte_at_address(self.reg[HL])) },
            0xb7 => { self.ora(self.reg[A]) },

            // b8
            0xb8 => { self.cmp(self.reg[B]) },
            0xb9 => { self.cmp(self.reg[C]) },
            0xba => { self.cmp(self.reg[D]) },
            0xbb => { self.cmp(self.reg[E]) },
            0xbc => { self.cmp(self.reg[H]) },
            0xbd => { self.cmp(self.reg[L]) },
            0xbe => { self.cmp(self.read_byte_at_address(self.reg[HL])) },
            0xbf => { self.cmp(self.reg[A]) },

            // c0
            0xc0 => { self.rnz() }, // If not 0 RET
            0xc1 => { self.pop(BC) }, // POP B
            0xc2 => { self.jnz() }, // JNZ addr
            0xc3 => { self.jmp() }, // JMP addr
            0xc4 => { self.cnz() }, // if NZ CALL addr
            0xc5 => { self.push(BC) }, // PUSH B
            0xc6 => { self.adi() }, // ADI (add immediate to acc)
            0xc7 => { self.rst(0x0) }, // CALL $0 (??)

            // c8
            0xc8 => { self.rz() }, // If Z RET
            0xc9 => { self.ret() }, // RET
            0xca => { self.jz() }, // JZ addr
            0xcb => {1}, // NOP
            0xcc => { self.cz() }, // if Z CALL addr
            0xcd => { self.call() }, // CALL addr
            0xce => { self.aci() }, // ACI (add immediate byte & carry to acc)
            0xcf => { self.rst(0x8) }, // CALL $8

            // d0
            0xd0 => { self.rnc() }, // if !C RET
            0xd1 => { self.pop(DE) }, // POP D
            0xd2 => { self.jnc() }, // JNC addr 
            0xd3 => { machine.output(self.read_byte_immediate(), self.reg[A]); 2 }, // OUT (??)
            0xd4 => { self.cnc() }, // if !C CALL addr
            0xd5 => { self.push(DE) }, // PUSH D
            0xd6 => { self.sui() }, // subtract immediate byte from acc & set all flags
            0xd7 => { self.rst(0x10) }, // CALL $18

            // d8
            0xd8 => { self.rc() }, // if C RET
            0xd9 => {1}, // NOP
            0xda => { self.jc() }, // if C jmp addr
            0xdb => { self.reg[A] = machine.input(self.read_byte_immediate()); 2 }, // IN (??)
            0xdc => { self.cc() }, // if C CALL addr
            0xdd => {1}, // NOP
            0xde => { self.sbi() }, // sutract immediate byte & carry from acc & set all flags
            0xdf => { self.rst(0x18) }, // CALL $18 (??)

            // e0
            0xe0 => { self.rpo() }, // if PO RET
            0xe1 => { self.pop(HL) }, // POP H
            0xe2 => { self.jpo() }, // JPO addr
            0xe3 => { self.xhtl() }, // XTHL
            0xe4 => { self.cpo() }, // if PO call addr
            0xe5 => { self.push(HL) }, // PUSH H
            0xe6 => { self.ani() }, // bitwise AND acc with immediate byte & set flags
            0xe7 => { self.rst(0x20) }, // CALL $20

            // e8
            0xe8 => { self.rpe() }, // if PE RET
            0xe9 => { self.pchl() }, // PCHL
            0xea => { self.jpe() }, // if PE move immediate word to PC
            0xeb => { self.xchg() }, // XCHG
            0xec => { self.cpe() }, // if PE call addr
            0xed => {1}, // NOP
            0xee => { self.xri() }, // bitwise XOR immediate byte with acc and set flags
            0xef => { self.rst(0x28) }, // CALL $28

            // f0
            0xf0 => { self.rp() }, // if P RET
            0xf1 => { self.pop(PSW) }, // POP psw
            0xf2 => { self.jp() }, // if P jmp addr
            0xf3 => { 1 }, // DI (??)
            0xf4 => { self.cp() }, // if P call addr
            0xf5 => { self.push(PSW) }, // PUSH PSW
            0xf6 => { self.ori() }, // bitwise OR immediate byte with acc and set flags
            0xf7 => { self.rst(0x30) }, // CALL $30

            // f8
            0xf8 => { self.rm() }, // if M, RET
            0xf9 => { self.sphl() }, // SPHL
            0xfa => { self.jm() }, // if M jmp addr
            0xfb => { println!("EI"); 1 }, // EI (??)
            0xfc => { self.cm() }, // if M call addr
            0xfd => {1}, // NOP
            0xfe => { self.cpi() }, // compare acc to immediate byte & set flags
            0xff => { self.rst(0x38) }, // CALL $38
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
        let mut machine = crate::machine::SpaceInvadersMachine::new();

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
        cpu.tick(&mut machine);
        assert_eq!(cpu.reg[A], 0b10);
        assert_eq!(cpu.reg.get_flag(Zero), false);
        assert_eq!(cpu.reg.get_flag(Sign), false);
        assert_eq!(cpu.reg.get_flag(Parity), false);
        assert_eq!(cpu.reg.get_flag(Carry), false);
        assert_eq!(cpu.reg.get_flag(AuxCarry), false);

        cpu.reg[C] = 0x1;
        cpu.tick(&mut machine);
        assert_eq!(cpu.reg[A], 0b11);
        assert_eq!(cpu.reg.get_flag(Zero), false);
        assert_eq!(cpu.reg.get_flag(Sign), false);
        assert_eq!(cpu.reg.get_flag(Parity), true);
        assert_eq!(cpu.reg.get_flag(Carry), false);
        assert_eq!(cpu.reg.get_flag(AuxCarry), false);

        cpu.reg[D] = 0x1;
        cpu.tick(&mut machine);
        assert_eq!(cpu.reg[A], 0b100);
        assert_eq!(cpu.reg.get_flag(Zero), false);
        assert_eq!(cpu.reg.get_flag(Sign), false);
        assert_eq!(cpu.reg.get_flag(Parity), false);
        assert_eq!(cpu.reg.get_flag(Carry), false);
        assert_eq!(cpu.reg.get_flag(AuxCarry), false);

        cpu.reg[E] = 0x1;
        cpu.tick(&mut machine);
        assert_eq!(cpu.reg[A], 0b101);
        assert_eq!(cpu.reg.get_flag(Zero), false);
        assert_eq!(cpu.reg.get_flag(Sign), false);
        assert_eq!(cpu.reg.get_flag(Parity), true);
        assert_eq!(cpu.reg.get_flag(Carry), false);
        assert_eq!(cpu.reg.get_flag(AuxCarry), false);

        cpu.reg[H] = 0x1;
        cpu.tick(&mut machine);
        assert_eq!(cpu.reg[A], 0b110);
        assert_eq!(cpu.reg.get_flag(Zero), false);
        assert_eq!(cpu.reg.get_flag(Sign), false);
        assert_eq!(cpu.reg.get_flag(Parity), true);
        assert_eq!(cpu.reg.get_flag(Carry), false);
        assert_eq!(cpu.reg.get_flag(AuxCarry), false);

        cpu.reg[L] = 0x1;
        cpu.tick(&mut machine);
        assert_eq!(cpu.reg[A], 0b111);
        assert_eq!(cpu.reg.get_flag(Zero), false);
        assert_eq!(cpu.reg.get_flag(Sign), false);
        assert_eq!(cpu.reg.get_flag(Parity), false);
        assert_eq!(cpu.reg.get_flag(Carry), false);
        assert_eq!(cpu.reg.get_flag(AuxCarry), false);

        cpu.tick(&mut machine);
        assert_eq!(cpu.reg[A], 0b1110);
        assert_eq!(cpu.reg.get_flag(Zero), false);
        assert_eq!(cpu.reg.get_flag(Sign), false);
        assert_eq!(cpu.reg.get_flag(Parity), false);
        assert_eq!(cpu.reg.get_flag(Carry), false);
        assert_eq!(cpu.reg.get_flag(AuxCarry), false);

        cpu.reg[A] = 0x0;
        cpu.memory[0x1001] = 0xFF;
        cpu.reg[HL] = 0x1001;

        cpu.tick(&mut machine);

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
        let mut machine = crate::machine::SpaceInvadersMachine::new();

        cpu.reg[A] = 0x1;
        cpu.reg[B] = 0x1;

        cpu.memory[0] = 0x80;

        cpu.tick(&mut machine);

        assert_eq!(cpu.reg.pc, 0x1);
    }

    #[test]
    fn test_lxi() {
        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();

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

        cpu.tick(&mut machine);
        assert_eq!(cpu.reg[BC], 0x7635);

        cpu.tick(&mut machine);
        assert_eq!(cpu.reg[DE], 0x7635);

        cpu.tick(&mut machine);
        assert_eq!(cpu.reg[HL], 0x7635);

        cpu.tick(&mut machine);
        assert_eq!(cpu.reg.sp, 0x7635);
    }

    #[test]
    fn test_stax() {
        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();

        cpu.memory[cpu.reg.pc] = 0x2;
        cpu.reg[A] = 0x42;
        cpu.reg[BC] = 0xF00F;

        cpu.tick(&mut machine);;

        assert_eq!(cpu.memory[0xF00F], 0x42);

        cpu.memory[cpu.reg.pc] = 0x12;
        cpu.reg[A] = 0x82;
        cpu.reg[DE] = 0xEA3;

        cpu.tick(&mut machine);;

        assert_eq!(cpu.memory[0xEA3], 0x82);
    }

    #[test]
    fn test_inx() {
        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();

        cpu.memory[0] = 0x3;
        cpu.memory[1] = 0x13;
        cpu.memory[2] = 0x23;

        cpu.reg[BC] = 0;
        cpu.reg[DE] = 0;
        cpu.reg[HL] = 0;

        cpu.tick(&mut machine);;
        cpu.tick(&mut machine);;
        cpu.tick(&mut machine);;

        assert_eq!(cpu.reg[BC], 0x1);
        assert_eq!(cpu.reg[DE], 0x1);
        assert_eq!(cpu.reg[HL], 0x1);
    }

    #[test]
    fn test_inr() {
        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();

        cpu.memory[0] = 0x4;
        cpu.memory[1] = 0xC;
        cpu.memory[2] = 0x14;
        cpu.memory[3] = 0x1C;
        cpu.memory[4] = 0x24;
        cpu.memory[5] = 0x2C;
        cpu.memory[6] = 0x3C;
        cpu.memory[7] = 0x34; // we execute M last because it changes the l register
        
        for x in 0..7 {
            cpu.tick(&mut machine);;
        }

        assert_eq!(cpu.reg[A], 1);
        assert_eq!(cpu.reg[B], 1);
        assert_eq!(cpu.reg[C], 1);
        assert_eq!(cpu.reg[D], 1);
        assert_eq!(cpu.reg[E], 1);
        assert_eq!(cpu.reg[H], 1);
        assert_eq!(cpu.reg[L], 1);

        cpu.reg[HL] = 8;
        cpu.tick(&mut machine);;
        assert_eq!(cpu.memory[8], 1);
    }

    #[test]
    fn test_dcr() {
        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();

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
            cpu.tick(&mut machine);;
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
        cpu.tick(&mut machine);;
        assert_eq!(cpu.memory[8], 1);
    }

    #[test]
    fn test_mvi() {
        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();
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
            cpu.tick(&mut machine);;
        }

        assert_eq!(cpu.reg[A], 0x10);
        assert_eq!(cpu.reg[B], 0x10);
        assert_eq!(cpu.reg[C], 0x10);
        assert_eq!(cpu.reg[D], 0x10);
        assert_eq!(cpu.reg[E], 0x10);
        assert_eq!(cpu.reg[H], 0x10);
        assert_eq!(cpu.reg[L], 0x10);

        cpu.reg[HL] = 0xFF;
        cpu.tick(&mut machine);;
        assert_eq!(cpu.memory[cpu.reg[HL]], 0x10);
    }

    #[test]
    fn test_dad() {
        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();
        cpu.memory[0] = 0x9;
        cpu.memory[1] = 0x19;
        cpu.memory[2] = 0x29;
        cpu.reg[BC] = 0x1;
        cpu.reg[DE] = 0x1;
        cpu.reg[HL] = 0x1;

        cpu.tick(&mut machine);;
        cpu.tick(&mut machine);;
        cpu.tick(&mut machine);;

        assert_eq!(cpu.reg[HL], 0x6);

        cpu.memory[3] = 0x39;
        cpu.reg.sp = 1;
        cpu.tick(&mut machine);;
        assert_eq!(cpu.reg[HL], 0x7);
    }

    #[test]
    fn test_ldax() {
        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();
        cpu.memory[0] = 0x0A;
        cpu.memory[1] = 0x1A;

        cpu.memory[0xF1] = 0x10;
        cpu.memory[0xF2] = 0x20;

        cpu.reg[BC] = 0xF1;
        cpu.reg[DE] = 0xF2;

        cpu.tick(&mut machine);;
        assert_eq!(cpu.reg[A], 0x10);
        cpu.tick(&mut machine);;
        assert_eq!(cpu.reg[A], 0x20);
    }

    #[test]
    fn test_dcx() {
        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();
        cpu.memory[0] = 0x0B;
        cpu.memory[1] = 0x1B;
        cpu.memory[2] = 0x2B;
        cpu.memory[3] = 0x3B;

        cpu.reg[BC] = 1;
        cpu.reg[DE] = 1;
        cpu.reg[HL] = 1;
        cpu.reg.sp = 1;

        for x in 0..4 {
            cpu.tick(&mut machine);;
        }

        assert_eq!(cpu.reg[BC], 0);
        assert_eq!(cpu.reg[DE], 0);
        assert_eq!(cpu.reg[HL], 0);
        assert_eq!(cpu.reg.sp, 0);
    }

    #[test]
    fn test_rlc() {
        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();
        cpu.memory[0] = 0x07;
        cpu.reg[A] = 0b10101010;

        cpu.tick(&mut machine);;
        assert_eq!(cpu.reg[A], 0b01010101);
        assert_eq!(cpu.reg.get_flag(Carry), true);

        cpu.reg.pc = 0;
        cpu.tick(&mut machine);;

        assert_eq!(cpu.reg[A], 0b10101010);
        assert_eq!(cpu.reg.get_flag(Carry), false);
    }

    #[test]
    fn test_rrc() {
        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();
        cpu.memory[0] = 0x0F;
        cpu.reg[A] = 0b10000001;

        cpu.tick(&mut machine);;
        assert_eq!(cpu.reg[A], 0b11000000);
        assert_eq!(cpu.reg.get_flag(Carry), true);

        cpu.reg.pc = 0;
        cpu.tick(&mut machine);;
        cpu.reg[A] = 0b01100000;
         assert_eq!(cpu.reg.get_flag(Carry), false);
    }

    #[test]
    fn test_ral() {
        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();
        cpu.memory[0] = 0x17;
        cpu.reg[A] = 0b10101010;

        cpu.tick(&mut machine);;
        assert_eq!(cpu.reg[A], 0b01010100);
        assert_eq!(cpu.reg.get_flag(Carry), true);

        cpu.reg.pc = 0;
        cpu.tick(&mut machine);;

        assert_eq!(cpu.reg[A], 0b10101001);
        assert_eq!(cpu.reg.get_flag(Carry), false);
    }

    #[test]
    fn test_rar() {
        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();
        cpu.memory[0] = 0x1F;
        cpu.reg[A] = 0b10000001;

        cpu.tick(&mut machine);;
        assert_eq!(cpu.reg[A], 0b01000000);
        assert_eq!(cpu.reg.get_flag(Carry), true);

        cpu.reg.pc = 0;
        cpu.tick(&mut machine);;

        assert_eq!(cpu.reg[A], 0b10100000);
    }

    #[test]
    fn test_shld() {
        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();
        cpu.memory[0] = 0x22;
        cpu.memory[1] = 0xAA;
        cpu.memory[2] = 0xBB;
        cpu.reg[L] = 0xCC;
        cpu.reg[H] = 0xDD;

        cpu.tick(&mut machine);;

        assert_eq!(cpu.memory[0xAA], 0xCC);
        assert_eq!(cpu.memory[0xBB], 0xDD);
    }

    #[test]
    fn test_lhld() {
        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();
        cpu.memory[0] = 0x2a;
        cpu.memory[1] = 0xAA;
        cpu.memory[2] = 0xBB;
        cpu.memory[0xAA] = 0xEE;
        cpu.memory[0xBB] = 0xFF;
        cpu.reg[L] = 0xCC;
        cpu.reg[H] = 0xDD;

        cpu.tick(&mut machine);;

        assert_eq!(cpu.reg[L], 0xEE);
        assert_eq!(cpu.reg[H], 0xFF);
    }

    #[test]
    fn test_cma() {
        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();
        cpu.memory[0] = 0x2f;
        cpu.reg[A] = 0b00000001;
        cpu.tick(&mut machine);;
        assert_eq!(cpu.reg[A], 0b11111110);
    }

    #[test]
    fn test_sta() {
        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();
        cpu.memory[0] = 0x32;
        cpu.memory[1] = 0xBB;
        cpu.memory[2] = 0xAA;
        cpu.reg[A] = 0xFF;
        cpu.tick(&mut machine);;
        assert_eq!(cpu.memory[0xAABB], 0xFF);
    }

    #[test]
    fn test_inxsp() {
        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();
        cpu.memory[0] = 0x33;
        cpu.tick(&mut machine);;
        assert_eq!(cpu.reg.sp, 1);
    }

    #[test]
    fn test_stc() {
        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();
        cpu.memory[0] = 0x37;
        cpu.tick(&mut machine);;
        assert_eq!(cpu.reg.get_flag(Carry), true);
    }

    #[test]
    fn test_lda() {
        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();
        cpu.memory[0] = 0x3a;
        cpu.memory[1] = 0xFF;
        cpu.memory[2] = 0x00;
        cpu.memory[0x00FF] = 0xAA;
        cpu.tick(&mut machine);;
        assert_eq!(cpu.reg[A], 0xAA);
    }

    #[test]
    fn test_cmc() {
        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();
        cpu.memory[0] = 0x3F;
        cpu.memory[1] = 0x3F;
        cpu.tick(&mut machine);;
        assert_eq!(cpu.reg.get_flag(Carry), true);
        cpu.tick(&mut machine);;
        assert_eq!(cpu.reg.get_flag(Carry), false);
    }

    #[test]
    fn test_mov() {
        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();
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

                cpu.tick(&mut machine);;

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
        let mut machine = crate::machine::SpaceInvadersMachine::new();

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

        cpu.tick(&mut machine);;
        assert_eq!(cpu.reg[A], 0b00000000);

        cpu.tick(&mut machine);;
        assert_eq!(cpu.reg[A], 0b00000010);

        cpu.reg[C] = 1;
        cpu.reg[D] = 1;
        cpu.reg[E] = 1;
        cpu.reg[H] = 1;
        cpu.reg[L] = 1;
        cpu.memory[cpu.reg[HL]] = 1;

        for x in 0..6 {
            cpu.tick(&mut machine);;
        }

        assert_eq!(cpu.reg[A], 0b1000);

        cpu.tick(&mut machine);;

        assert_eq!(cpu.reg[A], 0b10000);
    }

    #[test]
    fn test_sub() {
        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();
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

        cpu.tick(&mut machine);;
        assert_eq!(cpu.reg[A], 0b11111111);
        assert_eq!(cpu.reg.get_flag(Carry), true);

        for x in 0..6 {
            cpu.tick(&mut machine);;
        }
        
        assert_eq!(cpu.reg[A], 0b11111111 - 6);

        cpu.tick(&mut machine);;

        assert_eq!(cpu.reg[A], 0);
    }

    #[test]
    fn test_sbb() {
        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();
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

        cpu.tick(&mut machine);;
        assert_eq!(cpu.reg[A], 0b11111111);
        assert_eq!(cpu.reg.get_flag(Carry), true);

        for x in 0..6 {
            cpu.tick(&mut machine);;
        }
        
        assert_eq!(cpu.reg[A], 0b11111111 - 7);

        cpu.tick(&mut machine);;

        assert_eq!(cpu.reg[A], 0);
    }

    #[test]
    fn test_ana() {
        let registers = [A, B, C, D, E, H, L];

        let values = [
            0b11111111, // Initial
            0b11111110, // B
            0b11111100, // C
            0b11111000, // D
            0b11110000, // E
            0b11100000, // H
            0b11000000, // L
            0b10000000, // M
        ];

        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();
        let mut index = 0;
        
        for x in 0xa0..0xa8 {
            cpu.memory[index] = x;
            index += 1;
        }

        for x in 0..7 {
            cpu.reg[registers[x]] = values[x];
        }

        index = 1;
        for x in 0xa0..0xa8 {
            if x == 0xa6 {
                cpu.reg[HL] = 0xDDDD;
                cpu.memory[cpu.reg[HL]] = 0b10000000;
            }
            
            cpu.tick(&mut machine);;
            println!("{:08b}", cpu.reg[A]);

            if x == 0xa6 {
                assert_eq!(cpu.reg[A], 0b10000000);
            }
            else {
                assert_eq!(cpu.reg[A], values[(index) as usize]);
            }
            if index != 7 {index += 1};
        }
    }

    #[test]
    fn test_xra() {
        let registers = [A, B, C, D, E, H, L];

        let values = [
            0b11111111, // Initial
            0b11111110, // B
            0b00000011, // C
            0b00000110, // D
            0b00001100, // E
            0b00011000, // H
            0b00110000, // L
            0b01100000, // M
        ];

        let mut results = Vec::new();

        results.push(values[0] ^ values[1]);

        for x in 1..7 {
            results.push(results[x - 1] ^ values[x + 1]);
        }

        results.push(0b00000000);

        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();

        let mut index = 0;
        for x in 0xa8..0xb0 {
            cpu.memory[index] = x;
            index += 1;
        }

        for x in 0..7 {
            cpu.reg[registers[x as usize]] = values[x as usize];
        }

        index = 0;
        for x in 0xa8..0xb0 {
            if x == 0xae {
                cpu.reg[HL] = 0xDDDD;
                cpu.memory[cpu.reg[HL]] = 0b01100000;
            }

            cpu.tick(&mut machine);;
            
            assert_eq!(cpu.reg[A], results[index as usize]);

            if index != 7 {index += 1};
        }
    }

    #[test]
    fn test_ora() {
        let registers = [A, B, C, D, E, H, L];

        let values = [
            0b11111111, // Initial
            0b11111110, // B
            0b00000011, // C
            0b00000110, // D
            0b00001100, // E
            0b00011000, // H
            0b00110000, // L
            0b01100000, // M
        ];

        let mut results = Vec::new();

        results.push(values[0] | values[1]);

        for x in 1..7 {
            results.push(results[x - 1] | values[x + 1]);
        }

        results.push(0b11111111);

        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();

        let mut index = 0;
        for x in 0xb0..0xb8 {
            cpu.memory[index] = x;
            index += 1;
        }

        for x in 0..7 {
            cpu.reg[registers[x as usize]] = values[x as usize];
        }

        index = 0;
        for x in 0xa8..0xb0 {
            if x == 0xae {
                cpu.reg[HL] = 0xDDDD;
                cpu.memory[cpu.reg[HL]] = 0b01100000;
            }

            cpu.tick(&mut machine);;
            
            assert_eq!(cpu.reg[A], results[index as usize]);

            if index != 7 {index += 1};
        }
    }

    #[test]
    fn test_cmp() {
        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();

        cpu.memory[0] = 0xb8;
        cpu.memory[1] = 0xb9;
        cpu.memory[2] = 0xba;

        cpu.reg[A] = 0b00001111;
        cpu.reg[B] = 0b00001111; // Zero set
        cpu.reg[C] = 0b00011111; // Carry set
        cpu.reg[D] = 0b00000001; // Both reset

        cpu.tick(&mut machine);;

        assert_eq!(cpu.reg.get_flag(Zero), true);
        assert_eq!(cpu.reg.get_flag(Carry), false);

        cpu.tick(&mut machine);;

        assert_eq!(cpu.reg.get_flag(Zero), false);
        assert_eq!(cpu.reg.get_flag(Carry), true);

        cpu.tick(&mut machine);;

        assert_eq!(cpu.reg.get_flag(Zero), false);
        assert_eq!(cpu.reg.get_flag(Carry), false);
    }

    #[test]
    fn test_push_pop() {
        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();

        cpu.memory[0] = 0xc5;
        cpu.memory[1] = 0xd1;
        cpu.reg[BC] = 0xABBA;
        cpu.reg[SP] = 0xDDDD;

        cpu.tick(&mut machine);;
        cpu.tick(&mut machine);;

        assert_eq!(cpu.reg[BC], cpu.reg[DE]);
    }

    #[test]
    fn test_jmp() {
        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();

        cpu.memory[0] = 0xc3;
        cpu.memory[1] = 0xBB;
        cpu.memory[2] = 0xAA;

        cpu.tick(&mut machine);;

        assert_eq!(cpu.reg[PC], 0xAABB);
    }

    #[test]
    fn test_jnz() {
        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();

        cpu.memory[0] = 0xc2;
        cpu.memory[1] = 0xBB;
        cpu.memory[2] = 0xAA;
        cpu.memory[3] = 0xc2;
        cpu.memory[4] = 0xBB;
        cpu.memory[5] = 0xAA;
        cpu.reg.set_flag(Zero, true);

        cpu.tick(&mut machine);;

        assert_eq!(cpu.reg[PC], 3);

        cpu.reg.set_flag(Zero, false);

        cpu.tick(&mut machine);;

        assert_eq!(cpu.reg[PC], 0xAABB);
    }

    // TODO: Add test for jz

    #[test]
    fn test_call_ret() {
        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();

        cpu.reg[SP] = 0x100;

        cpu.reg[B] = 1;
        cpu.reg[C] = 2;

        cpu.memory[0] = 0xcd;
        cpu.memory[1] = 0xBB;
        cpu.memory[2] = 0xAA;
        cpu.memory[0xAABB] = 0x80;
        cpu.memory[0xAABC] = 0xc9;
        cpu.memory[3] = 0x81;

        for _ in 0..4 {
            cpu.tick(&mut machine);;
        }

        assert_eq!(cpu.reg[PC], 0x4);
        assert_eq!(cpu.reg[A], 0x3);
    }

    #[test]
    fn test_xhtl() {
        let mut cpu = CPU::new();
        let mut machine = crate::machine::SpaceInvadersMachine::new();

        cpu.memory[0] = 0xe3;

        cpu.reg[SP] = 0x100;

        cpu.reg[H] = 0xA;
        cpu.reg[L] = 0xB;

        cpu.memory[cpu.reg[SP]] = 0xFF;
        cpu.memory[cpu.reg[SP] + 1] = 0xFF;

        cpu.tick(&mut machine);;

        assert_eq!(cpu.reg[HL], 0xFFFF);
        assert_eq!(cpu.memory[cpu.reg[SP]], 0xA);
        assert_eq!(cpu.memory[cpu.reg[SP] + 1], 0xB);
    }

    // TODO: Add test for RZ

    // TODO: Add test for ADI

    // TODO: Add tests for RST

    // TODO: Add test for RNZ

    // TODO: Add test for ACI

    // TODO: Add test for SUI

    // TODO: Add test for PCHL, SPHL & XCHG
}