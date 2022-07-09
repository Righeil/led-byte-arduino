use crate::mode::Mode;
pub struct Counter {
    value: u8
}

impl Counter {
    pub fn new() -> Self{
        Self { value: 0 }
    }
}

impl Mode for Counter {
    fn reset(&mut self) {
        self.value = 0
    }

    fn next(&mut self) -> u8{
        if self.value != 255 {
            self.value += 1
        } 
        else {
            self.value = 0
        }

        return self.value
    }
}