# raspi_crate_sample

## Overview
Sample correction for crate(Rust library) to control Raspberry Pi

## Enable SPI
SPI is disabled when Raspberry pi is in default configration.  
For enabling SPI, you need run `raspi-config` or add `dtparam=spi=on` to `boot/config.txt` in root permission.

## Circuit for samples
The Following figure indicate circuit for crate samples.

![Circuit]

## Crate samples
Each crate sample is in "Sample" directory.
These sample worked in Rust 1.27.0.
Supported functions in each crate listed following table.

| | rppal | rustpi_io | sysfs_gpio | wiring_pi |
|:----:|:----:|:----:|:----:|:----:|
| GPIO | 〇 | △ | △ | 〇 |
| SPI | 〇 | 〇 | - | - |
  
〇: Supported and the function don't need root permission.  
△: Supported but the function need root permission.  
-: Not supported.

Crate repository url, crates.io url, license, listed below.

### rppal
rppal is Raspberry Pi Periphral Access Library.

#### crate.io
https://crates.io/crates/rppal

#### repository
https://github.com/golemparts/rppal

#### Licence
MIT

### rustpi_io
rustpi_io is library to access GPIO and SPI.

#### crate.io
https://crates.io/crates/rustpi_io

#### repository
https://github.com/Skasselbard/rustpiIO

#### Licence
GPL-3.0

### sysfs_gpio
sysfs_gpio is library to access Linux sysfs GPIO interface.

#### crate.io
https://crates.io/crates/sysfs-gpio

#### repository
https://github.com/rust-embedded/rust-sysfs-gpio

#### Licence
MIT/Apache 2.0

### wiringpi
wiringpi is wrapper library for WiringPi, GPIO access library written in C.

#### crate.io
https://crates.io/crates/wiringpi

#### repository
https://github.com/Ogeon/rust-wiringpi

#### Licence
MIT

## Licence of samples
Each sample conform to each crate lincence(See "Crate samples" section).

[Circuit]: Circuit.png