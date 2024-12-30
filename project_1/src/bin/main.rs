#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::gpio::{Input, Level, Output, Pull};
use esp_hal::prelude::*;
use log::info;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });


    let mut led = Output::new(peripherals.GPIO5, Level::Low);
    let button = Input::new(peripherals.GPIO4, Pull::Down);

    loop {

        if button.is_high() {
            info!("ON");
            led.set_high();


        } 

        if button.is_low() {
            info!("OFF");
            led.set_low();
        }

    }

}
