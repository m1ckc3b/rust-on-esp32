mod wifi;
mod httpd;

use wifi::wifi;
use httpd::httpd;

use anyhow::{Ok, Result};

use esp_idf_svc::hal::gpio::{Output, PinDriver};

use esp_idf_svc::hal::{peripheral::Peripheral, prelude::Peripherals};
use heapless::String;
use log::*;
use std::cell::RefCell;
use std::time::Duration;

// Wifi credentials
struct WifiCredentials {
    ssid: String<32>,
    pass: String<64>,
}

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let red_led = RefCell::new(PinDriver::output(peripherals.pins.gpio13).unwrap());
    let green_led = RefCell::new(PinDriver::output(peripherals.pins.gpio12).unwrap());
    let blue_led = RefCell::new(PinDriver::output(peripherals.pins.gpio14).unwrap());

    // Init WIFI
    let _wifi = wifi(peripherals.modem)?;

    // Init an http server
    let _httpd = httpd(red_led, green_led)?;

    // LOOP
    let stop = false;

    while !stop {
        std::thread::sleep(Duration::from_millis(500));
    }

    Ok(())
}
