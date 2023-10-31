use std::thread;
use std::time::Duration;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::peripherals::Peripherals;

use log::info;

fn main() {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio4).unwrap();

    loop {

        led.set_high().unwrap();
        thread::sleep(Duration::from_millis(10000));

        led.set_low().unwrap();
        thread::sleep(Duration::from_millis(10000));
        info!("blink");
    }
}