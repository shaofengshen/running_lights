//! LED Running Light Example
//! 
//! This example demonstrates a LED chasing/sequencing effect using multiple GPIO pins.
//! Hardware requirements:
//! - 8 LEDs connected to GPIO pins (16, 17, 18, 19, 23, 5, 2, 22)
//! 
//! The LEDs will light up in sequence, creating a "running light" effect
//! with a 500ms interval between each transition.

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    gpio::{Input, Level, Output, Pull},
    prelude::*,
};
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    println!("Hello world!");

    // Set GPIO7 as an output, and set its state high initially.
    let mut led = Output::new(peripherals.GPIO17, Level::Low);
    let button = Input::new(peripherals.GPIO16, Pull::Up);

    // Check the button state and set the LED state accordingly.
    loop {
        if button.is_high() {
            led.set_high();
        } else {
            led.set_low();
        }
    }
}
