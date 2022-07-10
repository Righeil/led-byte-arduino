mod counter;
pub use counter::Counter;
mod char_display;
pub use char_display::CharDisplay;
mod running_light;
pub use running_light::RunningLight;

#[derive(PartialEq, Clone)]
pub enum ModeType {
    Counter,
    CharDisplay,
    RunningLight
}

pub trait Mode {
    fn reset(&mut self);
    fn next(&mut self) -> u8;
}

