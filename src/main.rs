#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use core::sync::atomic::{AtomicBool, Ordering};
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

static CHANGE_MODE: AtomicBool = AtomicBool::new(false);

#[avr_device::interrupt(atmega328p)]
#[allow(non_snake_case)]
fn PCINT2() {
    avr_device::interrupt::free(|_| {
        CHANGE_MODE.store(true, Ordering::SeqCst)
    })
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut leds: [Pin<Output>; 8] = [
        pins.d5.into_output().downgrade(),
        pins.d6.into_output().downgrade(),
        pins.d7.into_output().downgrade(),
        pins.d8.into_output().downgrade(),
        pins.d9.into_output().downgrade(),
        pins.d10.into_output().downgrade(),
        pins.d11.into_output().downgrade(),
        pins.d12.into_output().downgrade()
    ];

    let mut mode: ModeType = ModeType::CharDisplay;

    let mut counter = Counter::new();
    let mut char_display = CharDisplay::new("ze_santassination_skial_12");

    let mut byte: u8;

    dp.EXINT.pcicr.write(|w| unsafe { w.bits(0b100) });
    dp.EXINT.pcmsk2.write(|w| unsafe { w.bits(0b100) });

    unsafe { avr_device::interrupt::enable() };

    loop {
        avr_device::interrupt::free(|_| {
            if CHANGE_MODE.load(Ordering::SeqCst) {
                mode = match mode {
                    ModeType::Counter => ModeType::CharDisplay,
                    ModeType::CharDisplay => ModeType::Counter
                };
        
                counter.reset();
                char_display.reset();

                CHANGE_MODE.store(false, Ordering::SeqCst)
            }
        });

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
        else {
            leds[i].set_low();
        }
        i += 1;
    }
}
