#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    analog::adc::{Adc, AdcConfig, Attenuation}, delay::Delay, prelude::*
};
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let analog_pin = peripherals.GPIO4;
    let mut adc1_config = AdcConfig::new();
    let mut pin = adc1_config.enable_pin(
        analog_pin,
        Attenuation::Attenuation0dB,
    );
    let mut adc1 = Adc::new(peripherals.ADC1, adc1_config);
    
    let delay = Delay::new();
    
    loop {
        let pin_value: u16 = nb::block!(adc1.read_oneshot(&mut pin)).unwrap();

        println!("ADC1 value: {}", pin_value);
    
        delay.delay_millis(1000);
    }
}
