# ESP32 Snippets


## Carte ESP (type wroom)

[ESP32 WROOM](https://www.upesy.fr/blogs/tutorials/esp32-pinout-reference-gpio-pins-ultimate-guide)

![Schema esp32 wroom](./doc/img/esp32_wroom_rev2_pinout.jpeg)

#### Les interfaces

- 3V3 = pour l'alimentation des circuits
- GND = sortie circuit pour la terre
- GPIO 
- UART = avec les broches de type RX et TX
- ANALOGUE = les broches ADC2 permettent de lire des données analogique.
- DAC 
- TOUCH 
- RTC 
- I2C = avec les broches SCL & SDA, connecter des périgique comme des ecrans
- SPI = avec les broches de type VSPI & HSPI

## Composants

[Les résistances](https://www.alloprof.qc.ca/fr/eleves/bv/sciences/les-resistors-et-leur-code-de-couleurs-s1533)


## Tests
[led](./doc/led.md)

[capteurs d'humidité](./doc/capteur-humidite.md)

[Wifi](./src/bin/test-wifi.rs)

[Relay](./src/bin/test-relay.rs)

