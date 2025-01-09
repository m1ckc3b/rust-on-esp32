use anyhow::{Ok, Result};
use embedded_svc::{http::Method, wifi::*};
use esp_idf_svc::hal::gpio::{Output, PinDriver};
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{peripheral::Peripheral, prelude::Peripherals},
    http::server::{Configuration as HttpServerConfig, EspHttpServer},
    nvs::EspDefaultNvsPartition,
};
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

    let red_led = RefCell::new(PinDriver::output(peripherals.pins.gpio26).unwrap());
    let green_led = RefCell::new(PinDriver::output(peripherals.pins.gpio27).unwrap());

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

fn httpd<R: esp_idf_svc::hal::gpio::Pin, G: esp_idf_svc::hal::gpio::Pin,>(
    red_led: RefCell<PinDriver<'static, R, Output>>, 
    green_led: RefCell<PinDriver<'static, G, Output>>
) -> Result<esp_idf_svc::http::server::EspHttpServer<'static>> {

    // Create an http server
    let mut server = EspHttpServer::new(&HttpServerConfig::default())?;

    // Homepage
    server.fn_handler(
        "/",
        Method::Get,
        move |req| {
            let mut resp = req.into_response(200, Some("Ok"), &[("Content-Type", "text/html")])?;

            resp.write(include_bytes!("../../assets/index.html"))?;

            Ok(())
        },
    )?;

    // Red LED on
    server.fn_handler(
        "/red",
        Method::Get,
        move |req| {
            let mut _resp = req.into_ok_response()?;

            red_led.borrow_mut().toggle()?;

            Ok(())
        },
    )?;

    // Green LED on
    server.fn_handler(
        "/green",
        Method::Get,
        move |req| {
            let mut _resp = req.into_ok_response()?;

            green_led.borrow_mut().toggle()?;

            Ok(())
        },
    )?;

    // Stylesheet
    server.fn_handler(
        "/style.css",
        Method::Get,
        move |req| {
            let mut resp = req.into_response(200, Some("Ok"), &[("Content-Type", "text/css")])?;

            resp.write(include_bytes!("../../assets/style.css"))?;

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

            resp.write(include_bytes!("../../assets/script.js"))?;

            Ok(())
        },
    )?;

    Ok(server)
}
