use super::{memory::Memory, rom::Rom};

pub struct DevicesController {
    memoryController: Memory,
}

impl DevicesController {
    pub fn new() -> Self {
        Self {
            memoryController: Memory::new(),
        }
    }

    pub fn load_rom_into_memory(&mut self, rom: Rom) {
        let rom_storage_start_address = 0x200;
        for (idx, byte) in rom.read().iter().enumerate() {
            self.memoryController
                .write(rom_storage_start_address + idx, *byte);
        }
    }

    pub fn read_bytes(&self, start: usize, offset: usize) -> Vec<u8> {}
}
