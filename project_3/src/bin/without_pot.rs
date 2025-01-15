#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output},
    ledc::{channel, timer, LSGlobalClkSource, Ledc, LowSpeed},
    prelude::*,
};

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    let led = Output::new(peripherals.GPIO4, Level::Low);

    // LEDC
    let mut ledc = Ledc::new(peripherals.LEDC);
    // Global Clock Source
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);
    // Timer
    let mut ledctimer = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    // Congig
    ledctimer
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty12Bit, // PWM resolution (precision level of 4096)
            clock_source: timer::LSClockSource::APBClk,
            frequency: 4u32.kHz(), // periode of 250 microseconds
        })
        .unwrap();
    // channel
    let mut channel0 = ledc.channel(channel::Number::Channel0, led);
    channel0
        .configure(channel::config::Config {
            timer: &ledctimer,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();

    let max_duty = 100u8;
    let min_duty = 0_u8;

    loop {
        // Increasing brightness
        for duty in min_duty..max_duty {
            channel0.set_duty(duty).unwrap();
            delay.delay_millis(10_u32);
        }

        // Decreasing
        for duty in (min_duty..max_duty).rev() {
            channel0.set_duty(duty).unwrap();
            delay.delay_millis(10_u32);
        }
    }
}
