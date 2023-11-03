use embedded_svc::http::client::*;
use embedded_svc::io::Read;
use embedded_svc::wifi::{ClientConfiguration, Configuration};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::http::client::EspHttpConnection;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::EspWifi;
use log::{error, info, warn};

//  cargo build --release --bin test-affichage
//  cargo run --release --bin test-affichage

fn main() {
    esp_idf_svc::log::EspLogger::initialize_default();
    esp_idf_sys::link_patches();//Needed for esp32-rs
    let peripherals = Peripherals::take().unwrap();
}
