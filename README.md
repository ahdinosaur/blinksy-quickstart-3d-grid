# [Blinksy][blinksy] Quickstart: 3D Grid 🌈

Get started with [Blinksy][blinksy] LED control using a [Gledopto GL-C-016WL-D controller](https://www.aliexpress.com/item/1005008707989546.html) and a [Treedix 3D volumetric cube](https://treedix.com/products/treedix-ws2812b-5050-rgb-led-light-board-led-cube-light-diy-kit-squared-led-digital-individually-addressable-5x5x5-for-children-and-teenagers-learning-compatible-with-arduino-and-raspberry-pi?variant=44841187148030)

https://github.com/user-attachments/assets/f320d1e9-5a05-424c-b64f-a529151e9847

The initial project is setup to run a scrolling rainbow on a 5x5x5 WS2812B cube.

(If you want to start with a 1D LED rope instead, see [`blinksy-quickstart-1d-rope`](https://github.com/ahdinosaur/blinksy-quickstart-3d-grid)).

## Bill of Materials

To get started now, buy:

- Gledopto GL-C-016WL-D: [AliExpress](https://www.aliexpress.com/item/1005008707989546.html), [Amazon](https://www.amazon.com/Controller-Dynamic-Lighting-Download-Addressable/dp/B0DT9QM25R)
- Treedix 3D Volumetric Cube: [Treedix.com](https://treedix.com/products/treedix-ws2812b-5050-rgb-led-light-board-led-cube-light-diy-kit-squared-led-digital-individually-addressable-5x5x5-for-children-and-teenagers-learning-compatible-with-arduino-and-raspberry-pi?variant=44841187148030), [Amazon](https://www.amazon.com/Treedix-Individually-Addressable-Teenagers-Compatible/dp/B0CGCH393Z)
- A BTF-LIGHTING 5V power supply: [AliExpress](https://www.aliexpress.com/item/32810906485.html)
  - Power usage depends on brightness and patterns: 5V 3A should be more than enough for 5x5x5, but more LEDs could use 5V 10A. Please read the [QuinLED power usage](https://quinled.info/2020/03/12/digital-led-power-usage/) guide before blowing up your power supply. Blinksy does not protect you here, [yet](https://github.com/ahdinosaur/blinksy/issues/47).

If you need a general LED supplier recommendation, I've only had success with "BTF-Lighting". You can find them on [AliExpress](https://btf-lighting.aliexpress.com/), [Amazon](https://www.amazon.com/stores/BTF-LIGHTING/BTF-LIGHTING/page/0FF60378-45DE-44E7-B0D7-8F5CD6478971), or on [their own website](https://www.btf-lighting.com/).

If you need more help with LED hardware, look at [QuinLED's helpful guides][quinled-guides].

[quinled-guides]: https://quinled.info/addressable-digital-leds/

## [How Blinksy works][how-blinksy-works]

[how-blinksy-works]: https://github.com/ahdinosaur/blinksy/#how-blinksy-works

- Define your LED [`layout`][layout] in 1D, 2D, or 3D space
- Create your visual [`pattern`][pattern] (effect), or choose from our built-in [`patterns`][patterns] library
  - The pattern will compute colors for each LED based on its position
- Setup a [`driver`][driver] to send each frame of colors to your LEDs, using our built-in [`drivers`][drivers] library.

[layout]: https://docs.rs/blinksy/0.6/blinksy/layout/index.html
[pattern]: https://docs.rs/blinksy/0.6/blinksy/pattern/index.html
[patterns]: https://docs.rs/blinksy/0.6/blinksy/patterns/index.html
[driver]: https://docs.rs/blinksy/0.6/blinksy/driver/index.html
[drivers]: https://docs.rs/blinksy/0.6/blinksy/drivers/index.html

## Development

### Pre-requisites

- Install Rust with [`rustup`][rustup]
- Install [`just`][just]

```shell
cargo install just
```

- Install [`espup`][espup]

```shell
cargo install espup
espup install
```

- Install [`espflash`][espflash]

```shell
cargo install espflash
```

- On Linux, add user to `dialout` group

```shell
sudo adduser $USER dialout
```

- Source the [ESP environment variables][esp-env-vars]

```shell
. $HOME/export-esp.sh
```

[rustup]: https://rustup.rs/
[just]: https://github.com/casey/just
[espup]: https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html
[espflash]: https://docs.esp-rs.org/book/tooling/espflash.html
[esp-env-vars]: https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html#3-set-up-the-environment-variables

### Run on microcontroller

```shell
just mcu
```

### Simulate on desktop

```shell
just desktop
```

## Resources

- [`blinksy`][blinksy]: Rust _no-std_ _no-alloc_  LED control library for 1D, 2D, and 3D LED setups
- [`gledopto`][gledopto]: Board support crate for Gledopto ESP32 Digital LED controllers

[blinksy]: https://github.com/ahdinosaur/blinksy
[gledopto]: https://github.com/ahdinosaur/blinksy/tree/main/esp/gledopto

As the Gledopto controller is an ESP32, here are some more ESP resources to help:

- [The Rust on ESP Book](https://docs.esp-rs.org/book/introduction.html): An overall guide on ESP32 on Rust
- [esp-hal](https://docs.espressif.com/projects/rust/esp-hal/1.0.0-beta.0/esp32/esp_hal/index.html): The Hardware Abstraction Layer for an ESP32 on Rust
- [espup](https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html): How to install the Xtensa target for Rust, required for ESP32
- [esp-generate](https://docs.esp-rs.org/book/writing-your-own-application/generate-project/esp-generate.html): A template to help you kickstart your project

And in case they are helpful:

- [ESP no-std book](https://docs.esp-rs.org/no_std-training)
- [ESP no-std examples](https://github.com/esp-rs/no_std-training)
- [Gledopto GL-C-016WL-D page](https://www.gledopto.eu/gledopto-esp32-wled-uart_1)
- [Gledopto GL-C-016WL-D user instructions](https://www.gledopto.eu/mediafiles/anleitungen/7002-gl-c-016wl-d-eng.pdf)

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>
