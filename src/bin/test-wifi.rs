use std::thread;
use std::time::Duration;
use embedded_svc::http::client::*;
use embedded_svc::io::Read;
use embedded_svc::wifi::{ClientConfiguration, Configuration};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::http::client::EspHttpConnection;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::EspWifi;
use log::{error, info, warn};

//  cargo build --release --bin test-wifi
//  cargo run --release --bin test-wifi

fn main() {
    esp_idf_svc::log::EspLogger::initialize_default();
    esp_idf_sys::link_patches();//Needed for esp32-rs
    let peripherals = Peripherals::take().unwrap();

    let  wifi_driver = init_wifi_connexion(peripherals);
    info!("IP info: {:?}", wifi_driver.sta_netif().get_ip_info().unwrap());

    http_client_get( "https://google.fr");

    loop {
        thread::sleep(Duration::from_millis(1000));
        info!("loop");
    }
}

fn http_client_get(url: &str) {
    let http_client = EspHttpConnection::new(&esp_idf_svc::http::client::Configuration {
        crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
        ..Default::default()
    }).unwrap();
    let mut client = Client::wrap(http_client);
    let request = client.get(url).unwrap();
    let response = request.submit().unwrap();
    match response.status() {
        200..=299 => {
            // If the status is OK, read response data chunk by chunk into a buffer and print it until done.
            let mut buf = [0_u8; 256];
            let mut reader = response;
            loop {
                if let Ok(size) = Read::read(&mut reader, &mut buf) {
                    if size == 0 {
                        break;
                    }
                    // Try converting the bytes into a Rust (UTF-8) string and print it.
                    let response_text = std::str::from_utf8(&buf[..size]).unwrap();
                    info!("response_text {}", response_text);
                }
            }
        }
        _ => error!("Unexpected response code: {}", response.status()),
    }
}

fn init_wifi_connexion(peripherals: Peripherals) -> EspWifi<'static>  {
    // Configure Wifi
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    let mut wifi_driver = EspWifi::new(
        peripherals.modem,
        sys_loop,
        Some(nvs),
    ).unwrap();

    wifi_driver.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: "Nom_ssid".into(),
        password: "1234567890".into(),
        ..Default::default()
    })).unwrap();

    // Start Wifi
    info!("Strating wifi module");
    wifi_driver.start().unwrap();

    wifi_driver.connect().unwrap();

    while !wifi_driver.is_connected().unwrap() {
        let config = wifi_driver.get_configuration().unwrap();
        warn!("Waiting for station {:?}", config);
    }
    info!("Connexion wifi up");
    wifi_driver
}