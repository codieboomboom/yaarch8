pub struct Vregisters {
    regs: [u8; 16],
}

impl Vregisters {
    pub fn new() -> Self {
        Self {
            regs: [0x0; 16]
        }
    }

    pub fn read(&self, offset: usize) -> u8 {
        self.regs[offset]
    }

    pub fn write(&mut self, offset: usize, value: u8 ) {
        self.regs[offset] = value;
    }
}