pub struct Memory {
    ram: [u8; 4096],
}

impl Memory {
    pub fn new() -> Self {
        Self { ram: [0x0; 4096] }
    }
}
