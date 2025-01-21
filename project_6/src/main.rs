use anyhow::{Ok, Result};
use embedded_svc::{http::{Headers, Method}, io::{Read, Write}, wifi::*};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{peripheral::Peripheral, prelude::Peripherals},
    hal::ledc::{config::TimerConfig, LedcTimerDriver, LedcDriver, Resolution},
    hal::prelude::*,
    http::server::{Configuration as HttpServerConfig, EspHttpServer},
    nvs::EspDefaultNvsPartition,
    wifi::{BlockingWifi, EspWifi}
};
use heapless::String;
use log::*;
use std::{cell::{RefCell, RefMut}, time::Duration};
use serde::Deserialize;

// Wifi credentials
struct WifiCredentials {
    ssid: String<32>,
    pass: String<64>,
}

#[derive(Deserialize)]
struct FormData {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    // LEDC
    let timer_driver = LedcTimerDriver::new(
        peripherals.ledc.timer0, 
        &TimerConfig::default()
            .frequency(5000.Hz())
            .resolution(Resolution::Bits8)
    ).unwrap();

    let red_pin = RefCell::new(LedcDriver::new(peripherals.ledc.channel0, &timer_driver, peripherals.pins.gpio1).unwrap());
    let green_pin = RefCell::new(LedcDriver::new(peripherals.ledc.channel1, &timer_driver, peripherals.pins.gpio2).unwrap());
    let blue_pin = RefCell::new(LedcDriver::new(peripherals.ledc.channel2, &timer_driver, peripherals.pins.gpio3).unwrap());

    red_pin.borrow_mut().set_duty(67).unwrap();
    red_pin.borrow_mut().enable().unwrap();
    green_pin.borrow_mut().set_duty(89).unwrap();
    green_pin.borrow_mut().enable().unwrap();
    blue_pin.borrow_mut().set_duty(96).unwrap();
    blue_pin.borrow_mut().enable().unwrap();

    // Init WIFI
    let _wifi = wifi(peripherals.modem)?;

    // Init an http server
    let _httpd = httpd(red_pin, green_pin, blue_pin)?;

    // LOOP
    let stop = false;

    while !stop {
        std::thread::sleep(Duration::from_millis(500));
    }

    Ok(())
}

fn wifi(
    modem: impl Peripheral<P = esp_idf_svc::hal::modem::Modem> + 'static,
) -> Result<esp_idf_svc::wifi::EspWifi<'static>> {
    let mut wifi_credentials = WifiCredentials {
        ssid: String::new(),
        pass: String::new(),
    };

    wifi_credentials.ssid.push_str("Livebox-0960").unwrap();
    wifi_credentials
        .pass
        .push_str("xid63LTupNQXxakaoS")
        .unwrap();

    let nvs = EspDefaultNvsPartition::take()?;
    let sysloop = EspSystemEventLoop::take()?;
    let mut esp_wifi = EspWifi::new(modem, sysloop.clone(), Some(nvs))?;

    let mut wifi = BlockingWifi::wrap(&mut esp_wifi, sysloop)?;

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: wifi_credentials.ssid,
        password: wifi_credentials.pass,
        ..Default::default()
    }))?;

    info!("Starting WiFi");
    wifi.start()?;

    info!("Connecting...");
    wifi.connect()?;

    info!("Waiting for DCHP lease...");
    wifi.wait_netif_up()?;

    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;
    info!("Wifi connected. Go to http://{}", &ip_info.ip);

    Ok(esp_wifi)
}

fn httpd(
    red_pin: RefCell<LedcDriver<'static>>,
    green_pin: RefCell<LedcDriver<'static>>,
    blue_pin: RefCell<LedcDriver<'static>>,
) -> Result<esp_idf_svc::http::server::EspHttpServer<'static>> {

    // Create an http server
    let mut server = EspHttpServer::new(&HttpServerConfig::default())?;

    // Homepage
    server.fn_handler(
        "/",
        Method::Get,
        move |req| {
            let mut resp = req.into_response(200, Some("Ok"), &[("Content-Type", "text/html")])?;

            resp.write(include_bytes!("../assets/index.html"))?;

            Ok(())
        },
    )?;

    // Send RGB color
    server.fn_handler(
        "/setcolor",
        Method::Post,
        move |mut req| {

            let len = req.content_len().unwrap_or(0) as usize;

            if len > 128 {
                req.into_status_response(413)?
                    .write_all("Request too big".as_bytes())?;
                return Ok(());
            }
    
            let mut buf = vec![0; len];
            req.read_exact(&mut buf)?;

            let request = serde_json::from_slice::<FormData>(&buf).unwrap();
            info!("r:{}, g:{}, b:{}", request.red, request.green, request.blue);

            set_color(
                red_pin.borrow_mut(), 
                green_pin.borrow_mut(), 
                blue_pin.borrow_mut(), 
                request.red, 
                request.green, 
                request.blue);

            Ok(())
        },
    )?;

    // Stylesheet
    server.fn_handler(
        "/style.css",
        Method::Get,
        move |req| {
            let mut resp = req.into_response(200, Some("Ok"), &[("Content-Type", "text/css")])?;

            resp.write(include_bytes!("../assets/style.css"))?;

            Ok(())
        },
    )?;

    // Script
    server.fn_handler(
        "/script.js",
        Method::Get,
        move |req| {
            let mut resp =
                req.into_response(200, Some("Ok"), &[("Content-Type", "text/javascript")])?;

            resp.write(include_bytes!("../assets/script.js"))?;

            Ok(())
        },
    )?;

    Ok(server)
}

fn set_color(
    mut red_pin: RefMut<'_, LedcDriver<'static>>,
    mut green_pin: RefMut<'_, LedcDriver<'static>>,
    mut blue_pin: RefMut<'_, LedcDriver<'static>>,
    red: u8, 
    green: u8, 
    blue: u8
) {
    let max_value: f32 = 255.0;

    let red_duty = (red as f32 * 100.0 / max_value).round() as u32;
    let green_duty = (green as f32 * 100.0 / max_value).round() as u32;
    let blue_duty = (blue as f32 * 100.0 / max_value).round() as u32;

    info!("r:{}, g:{}, b:{}", red_duty, green_duty, blue_duty);

    red_pin.set_duty(red_duty).unwrap();
    green_pin.set_duty(green_duty).unwrap();
    blue_pin.set_duty(blue_duty).unwrap();
}
