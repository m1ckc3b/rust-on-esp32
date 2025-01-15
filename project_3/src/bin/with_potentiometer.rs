#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    analog::adc::{Adc, AdcConfig, Attenuation},
    delay::Delay,
    gpio::{Level, Output}, 
    ledc::{channel, timer, LSGlobalClkSource, Ledc, LowSpeed},
    prelude::*,
};
use libm::round;
use esp_println::println;


#[entry]
fn main() -> ! {
    #[allow(unused)]
    // Init peripherals
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // Init LED
    let led = Output::new(peripherals.GPIO5, Level::Low);

    // Config analog pin and channel
    let mut adc_config = AdcConfig::new();
    let mut adc_pin = adc_config.enable_pin(peripherals.GPIO15, Attenuation::Attenuation11dB);

    // create ADC driver
    let mut adc1 = Adc::new(peripherals.ADC2, adc_config);

    // PWM
    let mut ledc = Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);
    let mut ledctimer = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    ledctimer.configure(timer::config::Config {
      duty: timer::config::Duty::Duty12Bit, // resolution du signal PWM (4096 niveaux de précision)
      clock_source: timer::LSClockSource::APBClk,
      frequency: 4u32.kHz(), // période de 250 microsecondes
  }).unwrap();
  // channel
  let mut channel0 = ledc.channel(channel::Number::Channel0, led);
  channel0.configure(channel::config::Config {
      timer: &ledctimer,
      duty_pct: 0,
      pin_config: channel::config::PinConfig::PushPull,
  }).unwrap();

    loop {
        let sample: u16 = nb::block!(adc1.read_oneshot(&mut adc_pin)).unwrap();
        let value = sample as f64 * 100.0 / 4095.0;
        let duty = round(value) as u8;

        println!("Duty: {}%", duty);

        channel0.set_duty(duty).unwrap();
        delay.delay_millis(500);
    }
}