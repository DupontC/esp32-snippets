#![feature(generic_arg_infer)]

use std::thread;
use std::time::Duration;
use esp_idf_hal::adc::{AdcChannelDriver, AdcDriver};
use esp_idf_svc::hal::adc;
use esp_idf_hal::adc::config::Config;
use esp_idf_hal::peripherals::Peripherals;
use log::info;

//  . $HOME/export-esp.sh
//  cargo build --release --bin test-capteur-yl69-humidite-du-sol
//  cargo run --release --bin test-capteur-yl69-humidite-du-sol

// https://www.elektormagazine.fr/news/capteurs-d-humidite-pour-les-systemes-d-arrosage

fn main() {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    // Initialisation du driver ADC pour lire et convertir une valeur analogique en digital
    let mut adc = AdcDriver::new(peripherals.adc2, &Config::new())
        .unwrap();
    let mut adc_pin_yl69 = AdcChannelDriver::<{ adc::attenuation::DB_11 }, _>::new(pins.gpio4)
        .unwrap();

    loop {
        let value = adc.read(&mut adc_pin_yl69).unwrap();
        info!("valeur {}", value);
        thread::sleep(Duration::from_millis(1000));
    };

}