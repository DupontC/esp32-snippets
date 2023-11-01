use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::peripherals::Peripherals;
use log::info;

//  cargo build --release --bin test-wifi
//  cargo run --release --bin test-wifi

fn main() {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take().unwrap();
}