use ds323x::{DateTimeAccess, Ds323x, NaiveDate, Rtcc, Timelike};
use esp_idf_hal::i2c;
use esp_idf_hal::peripherals::Peripherals;
use log::info;

//  . $HOME/export-esp.sh
//  get_esprs
//  cargo build --release --bin test-real-time-clock
//  cargo run --release --bin test-real-time-clock

// https://docs.rs/ds323x/latest/ds323x/#
fn main() {
    esp_idf_svc::log::EspLogger::initialize_default();
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    // Initialisation du driver i2c
    let i2c_driver = i2c::I2cDriver::new(
        peripherals.i2c0,
        pins.gpio21, // sda
        pins.gpio22, // sci
        &i2c::I2cConfig::default(),
    ).unwrap();
    let mut rtc = Ds323x::new_ds3231(i2c_driver);
    let datetime = NaiveDate::from_ymd_opt(2024, 2, 28)
        .unwrap()
        .and_hms_opt(18, 15, 00)
        .unwrap();

    rtc.set_datetime(&datetime).unwrap();

    loop {
        let time = rtc.time().unwrap();
        info!("Time: {}", time);
    }

}
