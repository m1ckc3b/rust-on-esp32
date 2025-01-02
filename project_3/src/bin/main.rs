#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::gpio::{Level, Output};
use esp_hal::mcpwm::operator::PwmPinConfig;
use esp_hal::mcpwm::timer::PwmWorkingMode;
use esp_hal::mcpwm::{McPwm, PeripheralClockConfig};
use esp_hal::{delay, prelude::*};

#[entry]
fn main() -> ! {
    let resolution = 255;
    let duration = 15;
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let led = Output::new(peripherals.GPIO4, Level::Low);

    // initialize peripheral
    let clock_cfg = PeripheralClockConfig::with_frequency(32.MHz()).unwrap();
    let mut mcpwm = McPwm::new(peripherals.MCPWM0, clock_cfg);

    // connect operator0 to timer0
    mcpwm.operator0.set_timer(&mcpwm.timer0);

    // connect operator0 to pin
    let mut pwm_pin = mcpwm
        .operator0
        .with_pin_a(led, PwmPinConfig::UP_ACTIVE_HIGH);

    // start timer with timestamp values in the range of 0..=255 and a frequency of 5 kHz
    let timer_clock_cfg_increase = clock_cfg
        .timer_clock_with_frequency(resolution, PwmWorkingMode::Increase, 5.kHz())
        .unwrap();

    mcpwm.timer0.start(timer_clock_cfg_increase);

    let delay = delay::Delay::new();
    
    loop {
        // TODO: increase/decrease the LED brightness
        for i in 0..=resolution {
            pwm_pin.set_timestamp(i);
            delay.delay_millis(duration);

            if i == resolution {
                for i in (0..=resolution).rev() {
                    pwm_pin.set_timestamp(i);
                    delay.delay_millis(duration);
                }
            }
        }
    }
}
