use std::fmt;

#[derive(Debug, PartialEq)]
pub enum CpuError {
    StackOverflowError,
    EmptyStackError,
    InvalidRegisterError,
}

impl std::error::Error for CpuError {}

impl fmt::Display for CpuError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CpuError::StackOverflowError => write!(
                f,
                "Stack Overflow! Trying to push more values that what the stack can handle!"
            ),
            CpuError::EmptyStackError => {
                write!(f, "Stack is empty! Trying to pop from an empty stack!")
            }
            CpuError::InvalidRegisterError => write!(
                f,
                "Invalid Register Offset! Trying to READ/WRITE to an invalid register!"
            ),
        }
    }
}
