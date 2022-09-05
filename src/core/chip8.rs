use super::cpu::cpu::Cpu;
use super::bus::Bus;

pub struct Chip8 {
    processor: Cpu,
    peripherals: Bus,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            processor: Cpu::new(),
            peripherals: Bus::new(),
        }
    }
}