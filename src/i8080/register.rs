use super::cpu::Word;
use super::cpu::Byte;

// TODO: Possibly `use` this in the module scope for external access?
bitflags! {
    pub struct Flags: Byte {
        const S =  0b10000000;
        const Z =  0b01000000;
        const AC = 0b00100000;
        const P =  0b00000100;
        const C =  0b00000001;
    }
}

#[derive(Default)]
pub struct Register {
    a: Byte,
    b: Byte,
    c: Byte,
    d: Byte,
    e: Byte,
    h: Byte,
    l: Byte,
    f: Byte,
    sp: Word,
    pc: Word,
    flags: Byte,
}

impl Register {
    pub fn get_bc(&self) -> Word {
        (u16::from(self.b) << 8) | u16::from(self.c) // get first register, put it into a u16 (such that it becomes 0x00XX), 
                                                     // shift to the left (0xXX00), then bitwise or with second reg to make (0xXXYY)
    }

    pub fn get_de(&self) -> Word {
        (u16::from(self.d) << 8) | u16::from(self.e)
    }

    pub fn get_hl(&self) -> Word {
        (u16::from(self.h) << 8) | u16::from(self.l)
    }

    pub fn get_psw(&self) -> Word {
        (u16::from(self.a) << 8) | u16::from(self.f)
    }

    pub fn set_bc(&mut self, word: Word) {
        self.b = (word >> 8) as Byte;
        self.c = (word & 0x00FF) as Byte;
    }

    pub fn set_de(&mut self, word: Word) {
        self.d = (word >> 8) as Byte;
        self.e = (word & 0x00FF) as Byte;
    }

    pub fn set_hl(&mut self, word: Word) {
        self.h = (word >> 8) as Byte;
        self.l = (word & 0x00FF) as Byte;
    }

    // Flag functions

    pub fn get_flag(&self, flag: Flags) {
        
    }

    pub fn set_flag(&self, byte: Byte) {
        
    }

    pub fn new() -> Self {
        Self::default()
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_word_registers() {
        let mut reg = Register::new();
        reg.b = 0xAA;
        reg.c = 0xBB;
        assert_eq!(reg.get_bc(), 0xAABB);

        reg.d = 0xCC;
        reg.e = 0xDD;
        assert_eq!(reg.get_de(), 0xCCDD);

        reg.h = 0xEE;
        reg.l = 0xFF;
        assert_eq!(reg.get_hl(), 0xEEFF);
    }

    #[test]
    fn test_set_word_registers() {
        let mut reg = Register::new();

        reg.set_bc(0xAABB);
        assert_eq!(reg.b, 0xAA);
        assert_eq!(reg.c, 0xBB);

        reg.set_de(0xCCDD);
        assert_eq!(reg.d, 0xCC);
        assert_eq!(reg.e, 0xDD);

        reg.set_hl(0xEEFF);
        assert_eq!(reg.h, 0xEE);
        assert_eq!(reg.l, 0xFF);
    }
}