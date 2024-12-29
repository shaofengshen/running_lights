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
    delay::Delay,
    gpio::{Level, Output},
    prelude::*,
};
use esp_println::println;

// 常量命名
const LED_COUNT: usize = 8;
const DELAY_MS: u32 = 500;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    
    // 数组命名
    let mut running_leds = [
        Output::new(peripherals.GPIO16, Level::Low),
        Output::new(peripherals.GPIO17, Level::Low),
        Output::new(peripherals.GPIO18, Level::Low),
        Output::new(peripherals.GPIO19, Level::Low),
        Output::new(peripherals.GPIO23, Level::Low),
        Output::new(peripherals.GPIO5, Level::Low),
        Output::new(peripherals.GPIO2, Level::Low),
        Output::new(peripherals.GPIO22, Level::Low),
    ];
    
    let delay = Delay::new();
    
    // 主循环命名
    run_led_sequence(&mut running_leds, &delay);
}

// 功能函数命名
fn run_led_sequence(leds: &mut [Output], delay: &Delay) -> ! {
    loop {
        for (led_index, led) in leds.iter_mut().enumerate() {
            led.set_high();
            delay.delay_millis(DELAY_MS);
            led.set_low();
        }
    }
}
