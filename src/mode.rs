mod counter;
pub use counter::Counter;
mod char_display;
pub use char_display::CharDisplay;

#[derive(PartialEq)]
pub enum ModeType {
    Counter,
    CharDisplay
}

pub trait Mode {
    fn reset(&mut self);
    fn next(&mut self) -> u8;
}

