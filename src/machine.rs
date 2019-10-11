use crate::i8080::cpu::{Byte, Word};
use crate::i8080::machine_interface::Machine;

#[derive(Default)]
pub struct SpaceInvadersMachine {
    shift_offset: Byte,
    shift0: Byte,
    shift1: Byte,
    port: Word,
}

#[derive(Clone, Copy)]
pub enum Keys {
    Coin,
    Start1,
    Start2,
    Left1,
    Left2,
    Right1,
    Right2,
    Shoot1,
    Shoot2,
}

fn mask_for_key(key: Keys) -> u16 {
    match key {
        Keys::Coin => (1 << 0),
        Keys::Start2 => (1 << 1),
        Keys::Start1 => (1 << 2),
        Keys::Shoot1 => (1 << 4),
        Keys::Left1 => (1 << 5),
        Keys::Right1 => (1 << 6),
        Keys::Shoot2 => (1 << (8 + 4)),
        Keys::Left2 => (1 << (8 + 5)),
        Keys::Right2 => (1 << (8 + 6)),
    }
}

impl SpaceInvadersMachine {
    pub fn new() -> SpaceInvadersMachine {
        Default::default()
    }

    pub fn press_key(&mut self, key: Keys) {
        self.port |= mask_for_key(key);
    }

    pub fn release_key(&mut self, key: Keys) {
        self.port &= !mask_for_key(key);
    }
}

impl Machine for SpaceInvadersMachine {
    fn input(&self, port: u8) -> u8 {
        match port {
            1 => (self.port & 0xFF) as u8,
            2 => (self.port >> 8) as u8,
            3 => {
                let reg = u16::from(self.shift1) << 8 | u16::from(self.shift0);
                ((reg >> (8 - self.shift_offset)) as u8)
            }
            _ => panic!("Unknown port"),
        }
    }

    fn output(&mut self, port: u8, value: u8) {
        match port {
            2 => {
                self.shift_offset = value & 0x07;
            }
            3 => (),
            4 => {
                self.shift0 = self.shift1;
                self.shift1 = value;
            }
            5 => (),
            6 => (),
            _ => panic!("Unknown port"),
        }
    }
}