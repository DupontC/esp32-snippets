use std::thread;
use std::time::Duration;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use esp_idf_hal::i2c;
use esp_idf_hal::peripherals::Peripherals;
use ssd1306::{I2CDisplayInterface, prelude::*, Ssd1306};

//  cargo build --release --bin test-affichage
//  cargo run --release --bin test-affichage

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
        &i2c::I2cConfig::default()
    ).unwrap();

    // configure le i2c driver pour de l'affichage sur un composant de type tssd1306
    let mut display = Ssd1306::new(
        I2CDisplayInterface::new(i2c_driver),
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
    Text::with_baseline("Hello", Point::zero(), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();


    // flush pour forcer l'affichage
    display.flush().unwrap();


    loop {
        thread::sleep(Duration::from_millis(4000));
        display.clear_buffer();
        Text::with_baseline("Hello World", Point::new(0, 16), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();
        display.flush().unwrap();
    }
}
