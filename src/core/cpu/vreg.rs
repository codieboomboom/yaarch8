use super::error::CpuError;

pub struct Vregisters {
    regs: [u8; 16],
}

impl Vregisters {
    pub fn new() -> Self {
        Self { regs: [0x0; 16] }
    }

    pub fn read(&self, offset: usize) -> Result<u8, CpuError> {
        match offset {
            0..=15 => Ok(self.regs[offset]),
            _ => Err(CpuError::InvalidRegisterError),
        }
    }

    pub fn write(&mut self, offset: usize, value: u8) -> Result<(), CpuError> {
        match offset {
            0..=15 => {
                self.regs[offset] = value;
                Ok(())
            }
            _ => Err(CpuError::InvalidRegisterError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CpuError;
    use super::Vregisters;

    #[test]
    fn test_invalid_offset_register_read() {
        let mock_reg = Vregisters::new();
        let result = mock_reg.read(16);
        assert_eq!(result, Err(CpuError::InvalidRegisterError));
    }

    #[test]
    fn test_register_read() {
        let mock_reg = Vregisters::new();
        let result = mock_reg.read(10);
        assert_eq!(result, Ok(0));
    }

    #[test]
    fn test_invalid_offset_register_write() {
        let mut mock_reg = Vregisters::new();
        let result = mock_reg.write(16, 1);
        assert_eq!(result, Err(CpuError::InvalidRegisterError));
    }

    #[test]
    fn test_register_write() {
        let mut mock_reg = Vregisters::new();
        let result = mock_reg.write(10, 1);
        assert_eq!(result, Ok(()));
        assert_eq!(mock_reg.regs[10], 1);
    }
}
