#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output};
use esp_hal::prelude::*;
use log::info;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    esp_println::logger::init_logger_from_env();

    let mut led_green = Output::new(peripherals.GPIO27, Level::Low);
    let mut led_red = Output::new(peripherals.GPIO26, Level::Low);

    let delay = Delay::new();

    info!("Blinky...");

    loop {
        led_green.toggle();
        delay.delay(500.millis());
        led_red.toggle();
        delay.delay(500.millis());
    }
}
