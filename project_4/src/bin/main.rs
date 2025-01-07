#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::prelude::*;
use esp_hal::gpio::{Level, Output, Input, Pull};
use esp_println::println;
use log::info;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    esp_println::logger::init_logger_from_env();

    let mut buzzer = Output::new(peripherals.GPIO26, Level::Low);
    let motion_sensor = Input::new(peripherals.GPIO27, Pull::Up);

    let delay = Delay::new();
    info!("Detector is ready!");

    loop {

        if motion_sensor.is_high() {
            buzzer.set_high();
            println!("Motion detected!Buzzer alarm!");
            delay.delay_millis(500);
            buzzer.set_low();
            println!("Motion detected!Buzzer stop alarm!");
        }
    }

}
