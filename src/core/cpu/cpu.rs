use super::vreg::Vregisters;
use super::stack::Stack;

pub struct Cpu {
    registers: Vregisters,
    pc: u16, // 12 bits
    i: u16,  // 12 bits
    stack: Stack,
}

impl Cpu {
    pub fn new() -> Self{
        Self {
            registers: Vregisters::new(),
            pc: 0x0,
            i: 0x0,
            stack: Stack::new(),
        }
    }
}