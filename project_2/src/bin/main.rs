#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
  analog::adc::{Adc, AdcConfig, Attenuation},
  delay::Delay,
  gpio::Io,
  prelude::*,
};
use esp_println::println;

#[entry]
fn main() -> ! {
  let peripherals = esp_hal::init(esp_hal::Config::default());
  let delay = Delay::new();

  let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

  let mut adc_config = AdcConfig::new();

  let mut adc_pin = adc_config.enable_pin(io.pins.gpio4, Attenuation::Attenuation11dB);

  let mut adc = Adc::new(peripherals.ADC2, adc_config);

  loop {
      let sample: u16 = nb::block!(adc.read_oneshot(&mut adc_pin)).unwrap();

      println!("ADC value: {}", sample);

      delay.delay_millis(500_u32);   
  }
}