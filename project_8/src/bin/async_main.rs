#![no_std]
#![no_main]

use embassy_executor::Spawner;
use esp_alloc as _;
use esp_backtrace as _;
use esp_hal::{clock::CpuClock, rng::Rng};
use esp_hal_embassy::main;
use esp_println::println;
use esp_wifi::EspWifiController;
use log::info;

extern crate alloc;

use async_webserver as lib;

#[main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    esp_alloc::heap_allocator!(72 * 1024);

    esp_println::logger::init_logger_from_env();

    let timer0 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);

    let rng = Rng::new(peripherals.RNG);

    info!("Embassy initialized!");

    let timg0 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG0);

    let wifi_init = lib::mk_static!(
        EspWifiController<'static>,
        esp_wifi::init(timg0.timer0, rng, peripherals.RADIO_CLK).unwrap()
    );

    let stack = lib::wifi::start_wifi(wifi_init, peripherals.WIFI, rng, &spawner).await;

    let web_app = lib::web::WebApp::default();
    for id in 0..lib::web::WEB_TASK_POOL_SIZE {
        spawner.must_spawn(lib::web::web_task(
            id,
            *stack,
            web_app.router,
            web_app.config,
        ));
    }
    println!("Web server started...");
}