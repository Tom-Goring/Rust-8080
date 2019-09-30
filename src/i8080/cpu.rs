pub type Address = u16;
pub type Word = u16;
pub type Byte = u8;

use super::register::Register;

pub struct CPU {
    pub reg: Register,
}