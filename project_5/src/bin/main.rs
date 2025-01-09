use anyhow::{bail, Result};
use embedded_svc::{http::Method, io::Write, wifi::*};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{peripheral::Peripheral, prelude::Peripherals, io::EspIOError,},
    http::server::{Configuration as OtherConfiguration, EspHttpServer},
    nvs::EspDefaultNvsPartition,
};
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use log::*;
use heapless::String;

// Wifi credentials
struct WifiCredentials {
    ssid: String<32>,
    pass: String<64>,
}

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    // Connect to the WIFI network
    let _wifi = match wifi(peripherals.modem) {
        Ok(inner) => {
            println!("Connected to Wi-Fi network!");
            inner
        }
        Err(err) => {
            // Red!
            bail!("Could not connect to Wi-Fi network: {:?}", err)
        }
    };

    // Create an http server
    let mut server = EspHttpServer::new(&OtherConfiguration::default())?;
    
    server.fn_handler(
        "/",
        Method::Get,
        |request| -> core::result::Result<(), EspIOError> {
            let mut response = request.into_ok_response()?;
            let html = format!(
                r#"
                <!DOCTYPE html>
                <html>
                    <head>
                        <meta charset="utf-8">
                        <title>esp-rs web server</title>
                    </head>
                    <body>
                        <h1>Hello from ESP32</h1>
                    </body>
                </html>
                "#
            );
            response.write_all(html.as_bytes())?;
            Ok(())
        },
    )?;

    Ok(())
}

fn wifi(modem: impl Peripheral<P = esp_idf_svc::hal::modem::Modem> + 'static,
) -> Result<esp_idf_svc::wifi::EspWifi<'static>> {

    let mut wifi_credentials = WifiCredentials {
        ssid: String::new(),
        pass: String::new()
    };

    // wifi_credentials.ssid.push_str("Livebox-0960").unwrap();
    // wifi_credentials.pass.push_str("xid63LTupNQXxakaoS").unwrap();

    wifi_credentials.ssid.push_str("Galaxy A506CA8").unwrap();
    wifi_credentials.pass.push_str("never-never").unwrap();

    let nvs = EspDefaultNvsPartition::take()?;
    let sysloop = EspSystemEventLoop::take()?;
    let mut esp_wifi = EspWifi::new(modem, sysloop.clone(), Some(nvs))?;

    let mut wifi = BlockingWifi::wrap(&mut esp_wifi, sysloop)?;

    wifi.set_configuration(&Configuration::Client(ClientConfiguration::default()))?;

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
