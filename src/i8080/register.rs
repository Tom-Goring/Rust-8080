use super::cpu::Byte;
use super::cpu::Word;
use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    F,
    M,
}

#[derive(Debug)]
pub enum Reg16 {
    BC,
    DE,
    HL,
    SP,
    PC,
}

#[derive(Copy, Clone)]
pub enum Flag {
    Carry = 0b00000001,
    Parity = 0b00000100,
    AuxCarry = 0b00100000,
    Sign = 0b10000000,
    Zero = 0b01000000,
}

use Reg16::{BC, DE, HL, PC, SP};
use Reg8::{A, B, C, D, E, F, H, L, M};

pub union RegisterPair {
    word: Word,
    bytes: (Byte, Byte),
}

pub struct Register {
    pub a: Byte,
    pub bc: RegisterPair,
    pub de: RegisterPair,
    pub hl: RegisterPair,
    pub sp: Word,
    pub pc: Word,
    pub f: Byte,
}

impl Register {
    pub fn new() -> Self {
        Register {
            a: 0,
            bc: RegisterPair { word: 0 },
            de: RegisterPair { word: 0 },
            hl: RegisterPair { word: 0 },
            sp: 0,
            pc: 0,
            f: 0,
        }
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        (self.f & flag as Byte) == flag as Byte
    }

    pub fn set_flag(&mut self, flag: Flag, set: bool) {
        if set {
            self.f |= flag as Byte;
        } else {
            self.f &= !(flag as Byte);
        }
    }
}

impl Index<Reg8> for Register {
    type Output = Byte;
    fn index(&self, register: Reg8) -> &Self::Output {
        unsafe {
            match register {
                A => &self.a,
                B => &self.bc.bytes.1,
                C => &self.bc.bytes.0,
                D => &self.de.bytes.1,
                E => &self.de.bytes.0,
                H => &self.hl.bytes.1,
                L => &self.hl.bytes.0,
                F => &self.f,
                M => panic!("Cannot access memory through use of fake `M` register!"),
            }
        }
    }
}

impl IndexMut<Reg8> for Register {
    fn index_mut(&mut self, register: Reg8) -> &mut Byte {
        unsafe {
            match register {
                A => &mut self.a,
                B => &mut self.bc.bytes.1,
                C => &mut self.bc.bytes.0,
                D => &mut self.de.bytes.1,
                E => &mut self.de.bytes.0,
                H => &mut self.hl.bytes.1,
                L => &mut self.hl.bytes.0,
                F => &mut self.f,
                M => panic!("Cannot access memory through use of fake `M` register!"),
            }
        }
    }
}

impl Index<Reg16> for Register {
    type Output = Word;
    fn index(&self, register: Reg16) -> &Self::Output {
        unsafe {
            match register {
                BC => &self.bc.word,
                DE => &self.de.word,
                HL => &self.hl.word,
                PC => &self.pc,
                SP => &self.sp,
            }
        }
    }
}

impl IndexMut<Reg16> for Register {
    fn index_mut(&mut self, register: Reg16) -> &mut Word {
        unsafe {
            match register {
                BC => &mut self.bc.word,
                DE => &mut self.de.word,
                HL => &mut self.hl.word,
                PC => &mut self.pc,
                SP => &mut self.sp,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_word_registers() {
        let mut reg = Register::new();
        reg[B] = 0xAA;
        reg[C] = 0xBB;
        assert_eq!(reg[BC], 0xAABB);

        reg[D] = 0xCC;
        reg[E] = 0xDD;
        assert_eq!(reg[DE], 0xCCDD);

        reg[H] = 0xEE;
        reg[L] = 0xFF;
        assert_eq!(reg[HL], 0xEEFF);
    }

    #[test]
    fn test_set_word_registers() {
        let mut reg = Register::new();

        reg[BC] = 0xAABB;
        assert_eq!(reg[B], 0xAA);
        assert_eq!(reg[C], 0xBB);

        reg[DE] = 0xCCDD;
        assert_eq!(reg[D], 0xCC);
        assert_eq!(reg[E], 0xDD);

        reg[HL] = 0xEEFF;
        assert_eq!(reg[H], 0xEE);
        assert_eq!(reg[L], 0xFF);
    }

    #[test]
    fn test_get_flags() {
        let mut reg = Register::new();

        reg.set_flag(Flag::Carry, true);
        assert_eq!(reg.get_flag(Flag::Carry), true);

        reg.set_flag(Flag::Sign, true);
        assert_eq!(reg.f, 0b10000001);
        assert_eq!(reg.get_flag(Flag::Sign), true);

        reg.set_flag(Flag::Carry, false);

        assert_eq!(reg.get_flag(Flag::Carry), false);
        assert_eq!(reg.get_flag(Flag::Sign), true);
    }

    #[test]
    fn test_set_flags() {
        let mut reg = Register::new();

        reg.set_flag(Flag::Carry, true);

        assert_eq!(reg.f, Flag::Carry as Byte);

        reg.set_flag(Flag::Carry, false);

        assert_eq!(reg.f, 0b00000000);

        reg.set_flag(Flag::Sign, true);

        assert_eq!(reg.f, Flag::Sign as Byte);

        reg.set_flag(Flag::Sign, false);

        assert_eq!(reg.f, 0b00000000);
    }
}
