use std::thread;
use std::time::Duration;
use ds323x::{DateTimeAccess, Ds323x, NaiveDateTime, Timelike};
use ds323x::Rtcc;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X13, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use esp_idf_hal::adc::{AdcChannelDriver, AdcDriver, attenuation};
use esp_idf_hal::adc::config::Config;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::i2c;
use esp_idf_hal::i2c::{I2cDriver};
use esp_idf_hal::peripherals::Peripherals;
use log::info;
use shared_bus::{I2cProxy, NullMutex};
use ssd1306::{I2CDisplayInterface, prelude::*, Ssd1306};
use ssd1306::mode::BufferedGraphicsMode;

//  . $HOME/export-esp.sh
//  get_esprs
//  cargo build --release --bin test-arrosage
//  cargo run --release --bin test-arrosage

const LIMITE_DECLENCHEMENT_PUMP: u16 = 1_100;
const END_HOUR: u32 = 9;
const START_HOUR: u32 = 18;
const CYCLE_TIME: u64 = 60_000;

#[derive(Debug, Copy, Clone, PartialEq)]
enum RelayState {
    OPEN,
    CLOSE,
}

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

    let bus = shared_bus::BusManagerSimple::new(i2c_driver);
    // configure le i2c driver pour de l'horloge rtc
    let mut rtc = Ds323x::new_ds3231(bus.acquire_i2c());
    // configure le i2c driver pour de l'affichage sur un composant de type ssd1306
    let mut display = Ssd1306::new(
        I2CDisplayInterface::new(bus.acquire_i2c()),
        DisplaySize128x64,
        DisplayRotation::Rotate0,
    ).into_buffered_graphics_mode();

    // configure le driver ADC pour lire et convertir une valeur analogique en digital
    let mut adc = AdcDriver::new(peripherals.adc2, &Config::new())
        .unwrap();
    let mut adc_pin_yl69 = AdcChannelDriver::<{ attenuation::DB_11 }, _>::new(pins.gpio2)
        .unwrap();
    // configure la pin gpio du controle du relay
    let mut relay = PinDriver::output(pins.gpio4).unwrap();

    loop {
        let value = adc.read(&mut adc_pin_yl69).unwrap();
        let state = get_new_relay_state(value, rtc.datetime().unwrap());
        if state == RelayState::OPEN {
            relay.set_low().unwrap();
        } else {
            relay.set_high().unwrap();
        }
        display.clear_buffer();

        let humidity_message = format!("{} : {}", "CAPTOR YL69", value.to_string());
        print_message_in_position(humidity_message, Point::zero(), &mut display);

        let time_message = format!("TIME : {}:{}", rtc.time().unwrap().hour(), rtc.time().unwrap().minute());
        print_message_in_position(time_message, Point::new(0, 15), &mut display);

        let relay_message = format!("RELAY: {:?}", state);
        print_message_in_position(relay_message, Point::new(0, 30), &mut display);

        display.flush().unwrap();
        thread::sleep(Duration::from_millis(CYCLE_TIME));
    }

    fn get_new_relay_state(value_captor: u16, time: NaiveDateTime) -> RelayState {
        let condition = value_captor > LIMITE_DECLENCHEMENT_PUMP
            && time.hour() < END_HOUR
            && time.hour() > START_HOUR;
        match condition {
            true => RelayState::OPEN,
            false => RelayState::CLOSE,
        }
    }

    fn print_message_in_position(message: String,
                                 position: Point,
                                 screen: &mut Ssd1306<I2CInterface<I2cProxy<NullMutex<I2cDriver>>>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>) {
        info!("{}", message);
        // definition de la police d'écriture
        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X13)
            .text_color(BinaryColor::On)
            .build();

        // ecriture sur l'ecran ssd1306 à la positino indiqué
        Text::with_baseline(&*message, position, text_style, Baseline::Top)
            .draw(screen)
            .unwrap();
    }
}
