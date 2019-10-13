use std::ops::{Index, IndexMut};
use super::cpu::Byte;
use super::cpu::Address;

/* This is purely for vanity - I hate having to constantly convert index values to usize. */

pub struct Memory {
     pub memory: [Byte; 0xFFFF],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            memory: [0; 0xFFFF],
        }
    }

    pub fn view(&self, start: Address, end: Address) -> &[Byte] {
        &self.memory[(start as usize)..=(end as usize)]
    }

    pub fn load(&mut self, address: usize, data: &[u8]) {
        self.memory[address..(address + data.len())].copy_from_slice(data);
    }
}

impl Index<Address> for Memory {
    type Output = Byte;
    fn index(&self, index: Address) -> &Self::Output {
        &self.memory[index as usize]
    }
}

impl IndexMut<Address> for Memory {
    fn index_mut(&mut self, index: Address) -> &mut Byte {
        &mut self.memory[index as usize]
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_access_with_u16() {
        let mut memory = Memory::new();

        memory[1] = 2;

        assert_eq!(memory[1], 2);

        let address: Address = 0x78;

        memory[address] = 45;

        assert_eq!(memory[address], 45);

        memory[address] = 0;

        assert_eq!(memory[address], 0);
    }
}