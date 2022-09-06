use super::{error::CpuError};

pub struct Vregisters {
    regs: [u8; 16],
}

impl Vregisters {
    pub fn new() -> Self {
        Self {
            regs: [0x0; 16]
        }
    }

    pub fn read(&self, offset: usize) -> Result<u8, CpuError> {
        match offset {
            0..=15 => Ok(self.regs[offset]),
            _ => Err(CpuError::InvalidRegisterError),
        }
    }

    pub fn write(&mut self, offset: usize, value: u8 ) -> Result<(), CpuError> {
        match offset {
            0..=15 => {
                self.regs[offset] = value;
                Ok(())
            },
            _ => Err(CpuError::InvalidRegisterError),
        }
    }
}
    }
}