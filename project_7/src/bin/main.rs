#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::main;
use log::info;

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);

    esp_println::logger::init_logger_from_env();

    let delay = Delay::new();
    loop {
        info!("Hello world!");
        delay.delay_millis(500);
    }

}
