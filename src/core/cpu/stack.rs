use super::error::CpuError;

pub struct Stack {
    contents: [u16; 16],
    sp: usize,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            contents: [0x0;16],
            sp: 0x0,
        }
    }

    pub fn push(&mut self, value: u16) -> Result<(),CpuError> {
        match self.sp {
            16 => Err(CpuError::StackOverflowError),
            _ => {
                self.contents[self.sp] = value;
                self.sp += 1;
                Ok(())
            }
        }
    }

    pub fn pop(&mut self) -> Result<(), CpuError> {
        match self.sp {
            0 => Err(CpuError::EmptyStackError),
            _ => {
                self.contents[self.sp] = 0x0;
                self.sp -= 1;
                Ok(())
            }
        }
    }

    pub fn peek(&self) -> u16 {
        self.contents[self.sp]
    }
}