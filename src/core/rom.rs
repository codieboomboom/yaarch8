use std::fs;

pub struct Rom {
    bytes: Vec<u8>,
}

impl Rom {
    pub fn new(rom_path: &str) -> Result<Self, Box<dyn std::error::Error + 'static>> {
        let read_bytes = fs::read(rom_path)?;
        Ok(Self { bytes: read_bytes })
    }

    pub fn size(&self) -> usize {
        self.bytes.len()
    }
}

// TODO: Implement a fmt or debug trait to print the ROM out nicely
