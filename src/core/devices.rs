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

    //TODO: Do this but in one line?
    pub fn read_bytes(&self, start_address: usize, size: usize) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        for idx in 0..size {
            result.push(self.memoryController.read(start_address+idx));
        }
        result
    }
}
