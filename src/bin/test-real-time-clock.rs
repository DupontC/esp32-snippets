use esp_idf_hal::i2c;
use esp_idf_hal::peripherals::Peripherals;
use log::info;
use ds323x::{DateTimeAccess, Ds323x, NaiveDate, Rtcc};

//  . $HOME/export-esp.sh
//  get_esprs
//  cargo build --release --bin test-real-time-clock
//  cargo run --release --bin test-real-time-clock

// https://docs.rs/ds323x/latest/ds323x/#
fn main() {
    esp_idf_svc::log::EspLogger::initialize_default();
    esp_idf_sys::link_patches();


    loop {
        let peripherals = Peripherals::take().unwrap();
        let pins = peripherals.pins;

        // Initialisation du driver i2c
        let i2c_driver = i2c::I2cDriver::new(
            peripherals.i2c0,
            pins.gpio21, // sda
            pins.gpio22, // sci
            &i2c::I2cConfig::default()
        ).unwrap();
        let mut rtc = Ds323x::new_ds3231(i2c_driver);

        let datetime = NaiveDate::from_ymd_opt(2020, 5, 1)
            .unwrap()
            .and_hms_opt(19, 59, 58)
            .unwrap();
        rtc.set_datetime(&datetime).unwrap();
        // do something else...
        let time = rtc.time().unwrap();
        info!("Time: {}", time);

        let _dev = rtc.destroy_ds3231();
    }
}
