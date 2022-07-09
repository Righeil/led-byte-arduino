#![no_std]
#![no_main]

use arduino_hal::port::{Pin, mode::Output};

mod mode;
use mode::{ModeType, Mode, Counter, CharDisplay};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    let dp = unsafe { arduino_hal::Peripherals::steal() };
    let pins = arduino_hal::pins!(dp);
    let mut led = pins.d13.into_output();
    loop {
        led.toggle();
        arduino_hal::delay_ms(100);
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut leds: [Pin<Output>; 8] = [
        pins.d2.into_output().downgrade(),
        pins.d3.into_output().downgrade(),
        pins.d4.into_output().downgrade(),
        pins.d5.into_output().downgrade(),
        pins.d6.into_output().downgrade(),
        pins.d7.into_output().downgrade(),
        pins.d8.into_output().downgrade(),
        pins.d9.into_output().downgrade()
    ];

    let button = pins.d12.into_floating_input().downgrade();

    let mut mode: ModeType = ModeType::CharDisplay;

    let mut counter = Counter::new();
    let mut char_display = CharDisplay::new("ze_santasinnation_skial_12");

    let mut byte: u8;

    loop {
        for led in &mut leds {
            led.set_low();
        }

        if button.is_high() {
            mode = match mode {
                ModeType::Counter => ModeType::CharDisplay,
                ModeType::CharDisplay => ModeType::Counter
            };

            counter.reset();
            char_display.reset();
        }

        byte = match mode {
            ModeType::Counter => counter.next(),
            ModeType::CharDisplay => char_display.next()
        };

        visualize(&mut leds, byte);
        arduino_hal::delay_ms(250);
    }
}

fn visualize(leds: &mut [Pin<Output>; 8], byte: u8) {
    let length = byte.count_ones() + byte.count_zeros();

    let mut i = 0;
    for n in 0..length {
        if (byte >> n & 1) == 1 {
            leds[i].set_high();
        }
        i += 1;
    }
}
