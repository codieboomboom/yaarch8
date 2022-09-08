use super::error::CpuError;

pub struct Stack {
    contents: [u16; 16],
    sp: usize,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            contents: [0x0; 16],
            sp: 0x0,
        }
    }

    pub fn push(&mut self, value: u16) -> Result<(), CpuError> {
        match self.sp {
            15.. => Err(CpuError::StackOverflowError),
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

#[cfg(test)]
mod tests {
    use super::CpuError;
    use super::Stack;

    #[test]
    fn test_push_cause_overflow_of_stack() {
        let mut mock_stack = Stack::new();
        mock_stack.sp = 15;
        let result = mock_stack.push(1);
        assert_eq!(result, Err(CpuError::StackOverflowError));

        mock_stack.sp = 16;
        let result = mock_stack.push(1);
        assert_eq!(result, Err(CpuError::StackOverflowError));
    }

    #[test]
    fn test_push() {
        let mut mock_stack = Stack::new();
        let result = mock_stack.push(1);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_peek() {
        let mock_stack = Stack::new();
        let result = mock_stack.peek();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_pop_empty_stack() {
        let mut mock_stack = Stack::new();
        let result = mock_stack.pop();
        assert_eq!(result, Err(CpuError::EmptyStackError));
    }

    #[test]
    fn test_pop() {
        let mut mock_stack = Stack::new();
        mock_stack.contents[1] = 10;
        mock_stack.sp = 1;
        let result = mock_stack.pop();

        assert_eq!(result, Ok(()));
        assert_eq!(mock_stack.sp, 0);
        assert_eq!(mock_stack.contents[1], 0);
    }
}
