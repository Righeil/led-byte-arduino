use crate::mode::Mode;

pub struct RunningLight {
    value: u8,
    increase: bool
}

impl RunningLight {
    pub fn new() -> Self {
        Self { 
            value: 0b00000001, 
            increase: true 
        }
    }
}

impl Mode for RunningLight {
    fn next(&mut self) -> u8 {
        if self.value == 1 { self.increase = true}
        if self.value == 128 { self.increase = false}

        if self.increase {
            self.value <<= 1;
        }
        else {
            self.value >>= 1;
        }
        
        return self.value
    }

    fn reset(&mut self) {
        self.value = 0b00000001;
    }
}