//! LED Running Light Example
//! 
//! This example demonstrates a LED chasing/sequencing effect using multiple GPIO pins.
//! Hardware requirements:
//! - 8 LEDs connected to GPIO pins (17, 18, 19, 23, 5, 2, 22)


#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output, Input, Pull},
    prelude::*,
};
use esp_println::println;

// 常量命名
const LED_COUNT: usize = 7;
const DELAY_MS: u32 = 500;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    
    // 初始化重置按钮
    let reset_button = Input::new(peripherals.GPIO16, Pull::Up);
    
    let mut running_leds = [
        Output::new(peripherals.GPIO17, Level::Low),
        Output::new(peripherals.GPIO18, Level::Low),
        Output::new(peripherals.GPIO19, Level::Low),
        Output::new(peripherals.GPIO23, Level::Low),
        Output::new(peripherals.GPIO5, Level::Low),
        Output::new(peripherals.GPIO2, Level::Low),
        Output::new(peripherals.GPIO22, Level::Low),
    ];
    
    let delay = Delay::new();
    
    // 传入重置按钮
    run_led_sequence(&mut running_leds, &reset_button, &delay);
}

fn run_led_sequence(leds: &mut [Output], reset_button: &Input, delay: &Delay) -> ! {
    let mut current_led = 0;  // 跟踪当前 LED 位置
    
    loop {
        // 检查重置按钮
        if reset_button.is_low() {  // 按钮按下时为低电平
            println!("重置按钮被按下！");
            current_led = 0;  // 重置 LED 位置
            // 等待按钮释放
            while reset_button.is_low() {
                delay.delay_millis(10);  // 防抖
            }
            continue;  // 从头开始新的循环
        }

        // LED 控制逻辑
        leds[current_led].set_high();
        delay.delay_millis(DELAY_MS);
        leds[current_led].set_low();
        
        // 更新 LED 位置
        current_led = (current_led + 1) % LED_COUNT;
    }
}