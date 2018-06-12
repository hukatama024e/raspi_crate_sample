extern crate rustpi_io;

use std::io::{Read, Write};
use std::thread;
use std::time::Duration;
use rustpi_io::gpio::{GPIO, GPIOData, GPIOMode};
use rustpi_io::serial::{SerialPi, Device, Speed, SpiMode, ComMode};

// GPIO BCM pin number
const GPIO_PIN_NUM : u8 = 24;

// Count for blinking an LED 
const BLINK_CNT : u8 = 10;

fn main() {
    let gpio = GPIO::new( GPIO_PIN_NUM, GPIOMode::Write ).expect( "Failed GPIO::new" );
    let mut blinking_cnt = 0;
    
    // Toggle GPIO pinfor blinking an LED
    while blinking_cnt < BLINK_CNT {
        gpio.set( GPIOData::High ).expect( "Failed GPIO.set" );
        thread::sleep( Duration::from_secs( 1 ) );

        gpio.set( GPIOData::Low ).expect( "Failed GPIO.set" );
        thread::sleep( Duration::from_secs( 1 ) );
        blinking_cnt = blinking_cnt + 1;
    }

    // SPI setting
    let mut spi = SerialPi::new( Device::CE0, Speed::Mhz62_5, SpiMode::Mode0, ComMode::FullDuplex )
                    .expect( "Failed SerialPi::new" );

    // Write SPI data
    let mut write_data = vec![0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];
    spi.write( &mut write_data ).expect( "Failed SerialPi::write" );

    // Read SPI data
    let mut read_data = [0; 8];
    let read_size = spi.read( &mut read_data ).expect( "Failed SerialPi::read" );

    for i in 0..read_size {
        println!( "read_data[{}] = 0x{:X}", i, read_data[i] );
    }
}
