use std::thread;
use std::time::Duration;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::peripherals::Peripherals;
use log::info;


//  cargo build --release --bin test-led
//  cargo run --release --bin test-led

fn main() {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio4).unwrap();

    loop {

        led.set_high().unwrap();
        thread::sleep(Duration::from_millis(1000));

        led.set_low().unwrap();
        thread::sleep(Duration::from_millis(1000));
        info!("blink");
    }
}