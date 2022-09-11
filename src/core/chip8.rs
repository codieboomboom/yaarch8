use super::devices::DevicesController;
use super::cpu::cpu::Cpu;

pub struct Chip8 {
    processor: Cpu,
    peripherals: DevicesController,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            processor: Cpu::new(),
            peripherals: DevicesController::new(),
        }
    }
}
