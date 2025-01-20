use anyhow::{Ok, Result};
use embedded_svc::{http::{Headers, Method}, io::{Read, Write}, wifi::*};
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{peripheral::Peripheral, prelude::Peripherals},
    http::server::{Configuration as HttpServerConfig, EspHttpServer},
    nvs::EspDefaultNvsPartition,
};
use heapless::String;
use log::*;
use std::time::Duration;

use serde::Deserialize;

// Wifi credentials
struct WifiCredentials {
    ssid: String<32>,
    pass: String<64>,
}

#[derive(Deserialize)]
struct FormData<'a> {
    red: &'a str,
    green: &'a str,
    blue: &'a str,
}

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();


    // Init WIFI
    let _wifi = wifi(peripherals.modem)?;

    // Init an http server
    let _httpd = httpd()?;

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

fn httpd() -> Result<esp_idf_svc::http::server::EspHttpServer<'static>> {

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
        |mut req| {
            let len = req.content_len().unwrap_or(0) as usize;

            if len > 128 {
                req.into_status_response(413)?
                    .write_all("Request too big".as_bytes())?;
                return Ok(());
            }
    
            let mut buf = vec![0; len];
            req.read_exact(&mut buf)?;
            let mut resp = req.into_ok_response()?;

            if let Ok(form) = serde_json::from_slice::<FormData>(&buf) {
                write!(resp, "r:{}, g:{}, b:{}", form.red, form.green, form.blue)?;
            } else {
                resp.write_all("JSON error".as_bytes())?;
            }

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
