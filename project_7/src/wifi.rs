
pub fn connect_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>) -> anyhow::Result<()> {
  // If instead of creating a new network you want to serve the page
  // on your local network, you can replace this configuration with
  // the client configuration from the http_client example.
  let wifi_configuration = wifi::Configuration::AccessPoint(AccessPointConfiguration {
      ssid: SSID.try_into().unwrap(),
      ssid_hidden: true,
      auth_method: AuthMethod::WPA2Personal,
      password: PASSWORD.try_into().unwrap(),
      channel: CHANNEL,
      ..Default::default()
  });

  wifi.set_configuration(&wifi_configuration)?;

  wifi.start()?;
  info!("Wifi started");

  // If using a client configuration you need
  // to connect to the network with:
  //
  //  ```
  //  wifi.connect()?;
  //  info!("Wifi connected");
  // ```

  wifi.wait_netif_up()?;
  info!("Wifi netif up");

  info!(
      "Created Wi-Fi with WIFI_SSID `{}` and WIFI_PASS `{}`",
      SSID, PASSWORD
  );

  Ok(())
}