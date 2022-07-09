use crate::mode::Mode;

pub struct CharDisplay<'l> {
    value: &'l str,
    index: usize
}

impl<'l> CharDisplay<'l> {
    pub fn new(value: &'l str) -> Self {
        Self { value, index: 0 }
    }
}

impl Mode for CharDisplay<'_> {
    fn reset(&mut self) {
        self.index = 0;
    }

    fn next(&mut self) -> u8 {
        for (i, ch) in self.value.chars().enumerate() { 
            if self.index == i {
                if self.index != self.value.len() - 1 {
                    self.index += 1;
                }
                else {
                    self.index = 0;
                }
                return ch as u8;
            }
        }
        return 0
    }
}