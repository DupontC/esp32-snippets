use std::thread;
use std::time::Duration;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::peripherals::Peripherals;
use log::info;


//  cargo build --release --bin test-relay
//  cargo run --release --bin test-relay

fn main() {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take().unwrap();
    let mut relay = PinDriver::output(peripherals.pins.gpio4).unwrap();

    loop {
        // off
        relay.set_high().unwrap();
        thread::sleep(Duration::from_millis(5000));

        relay.set_low().unwrap();
        thread::sleep(Duration::from_millis(5000));
        info!("open relay");
    }
}