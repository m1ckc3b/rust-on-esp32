use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_svc::eventloop::{
  EspSystemEventLoop,
  nvs::EspDefaultNvsPartition,
};
use embedded_svc::{http::Method, wifi::*};

pub fn wifi(
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