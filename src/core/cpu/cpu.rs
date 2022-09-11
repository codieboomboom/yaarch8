use super::super::devices::DevicesController;
use super::stack::Stack;
use super::vreg::Vregisters;

pub struct Cpu {
    registers: Vregisters,
    pc: u16, // 12 bits
    i: u16,  // 12 bits
    stack: Stack,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: Vregisters::new(),
            pc: 0x200,
            i: 0x0,
            stack: Stack::new(),
        }
    }

    pub fn step(&mut self, peripherals: &mut DevicesController) {}
}
