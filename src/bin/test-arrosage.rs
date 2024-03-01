use std::thread;
use std::time::Duration;

use ds323x::Ds323x;
use ds323x::Rtcc;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use esp_idf_hal::adc::{AdcChannelDriver, AdcDriver, attenuation};
use esp_idf_hal::adc::config::Config;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::i2c;
use esp_idf_hal::i2c::I2cConfig;
use esp_idf_hal::peripherals::Peripherals;
use log::info;
use ssd1306::{I2CDisplayInterface, prelude::*, Ssd1306};

//  . $HOME/export-esp.sh
//  get_esprs
//  cargo build --release --bin test-arrosage
//  cargo run --release --bin test-arrosage

const LIMITE_DECLENCHEMENT_POMPE: u16 = 1_100;

fn main() {
    esp_idf_svc::log::EspLogger::initialize_default();
    esp_idf_sys::link_patches();
    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    // Initialisation du driver i2c
    let  i2c_driver = i2c::I2cDriver::new(
        peripherals.i2c0,
        pins.gpio21, // sda
        pins.gpio22, // sci
        &i2c::I2cConfig::default(),
    ).unwrap();

    let bus = shared_bus::BusManagerSimple::new(i2c_driver);
    let mut rtc = Ds323x::new_ds3231(bus.acquire_i2c());

    // configure le i2c driver pour de l'affichage sur un composant de type ssd1306
    let mut display = Ssd1306::new(
        I2CDisplayInterface::new(bus.acquire_i2c()),
        DisplaySize128x64,
        DisplayRotation::Rotate0,
    ).into_buffered_graphics_mode();
    display.init().unwrap();

    // definition de la police d'écriture
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    // ecriture sur l'ecran ssd1306
    Text::with_baseline("Initialisation...", Point::zero(), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();
    display.flush().unwrap();


    // Initialisation du driver ADC pour lire et convertir une valeur analogique en digital
    let mut adc = AdcDriver::new(peripherals.adc2, &Config::new())
        .unwrap();
    let mut adc_pin_yl69 = AdcChannelDriver::<{ attenuation::DB_11 }, _>::new(pins.gpio2)
        .unwrap();


    let mut relay = PinDriver::output(pins.gpio4).unwrap();

    loop {
        let value = adc.read(&mut adc_pin_yl69).unwrap();
        info!("valeur yl69 {}", value);
        display.clear_buffer();
        let humiditeMessage = format!("{} {}","valeur yl69", value.to_string());
        Text::with_baseline(&*humiditeMessage, Point::zero(), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();
        let timeMessage = format!("Heure : {}", rtc.time().unwrap());
        info!("{}", timeMessage);
        Text::with_baseline(&*timeMessage,  Point::new(0, 15) , text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();
        if value > LIMITE_DECLENCHEMENT_POMPE {
            relay.set_low().unwrap();
            Text::with_baseline("pompe ouverte",  Point::new(0, 30) , text_style, Baseline::Top)
                .draw(&mut display)
                .unwrap();
            info!("open relay");
        }else {
            relay.set_high().unwrap();
            Text::with_baseline("pompe ferme" ,  Point::new(0, 30) , text_style, Baseline::Top)
                .draw(&mut display)
                .unwrap();
            info!("close relay");
        }
        display.flush().unwrap();

        thread::sleep(Duration::from_millis(30_000));
    }
}
