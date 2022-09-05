pub struct Stack {
    inner: [u16; 16],
    sp: usize,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            inner: [0x0;16],
            sp: 0x0,
        }
    }

    pub fn push(value: u16) {
        
    }

    pub fn pop() {

    }

    pub fn peek(&self) -> u16 {
        self.inner[self.sp]
    }
}