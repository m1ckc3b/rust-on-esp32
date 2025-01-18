use esp_idf_svc::http::server::{Configuration as HttpServerConfig, EspHttpServer};
use embedded_svc::{http::Method, wifi::*};


pub fn httpd<R: esp_idf_svc::hal::gpio::Pin, G: esp_idf_svc::hal::gpio::Pin,>(
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
