extern crate rppal;

use std::thread;
use std::time::Duration;
use rppal::{gpio, spi};
use rppal::gpio::{Gpio, Level};
use rppal::spi::{Spi, Bus, SlaveSelect};

// GPIO BCM pin number
const GPIO_PIN_NUM : u8 = 24;

// Count for blinking an LED 
const BLINK_CNT : u8 = 10;

fn main() {
    let mut gpio = Gpio::new().expect( "Failed Gpio::new" );
    let mut blinking_cnt = 0;

    // Change GPIO mode to output
    gpio.set_mode( GPIO_PIN_NUM, gpio::Mode::Output );

    // Toggle GPIO pin for blinking an LED
    while blinking_cnt < BLINK_CNT {
        gpio.write( GPIO_PIN_NUM, Level::High );
        thread::sleep( Duration::from_secs( 1 ) );

        gpio.write( GPIO_PIN_NUM, Level::Low );
        thread::sleep( Duration::from_secs( 1 ) );
        blinking_cnt = blinking_cnt + 1;
    }

    // Return GPIO mode to input
    gpio.set_mode( GPIO_PIN_NUM, gpio::Mode::Input );

    // SPI setting
    let spi = Spi::new( Bus::Spi0, SlaveSelect::Ss0, 6_2500_000, spi::Mode::Mode0 )
                    .expect( "Failed Spi::new" );

    // write and read SPI data
    let write_data = vec![0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];
    let mut read_data = [0; 8];
    let read_size = spi.transfer( &mut read_data, &write_data ).expect( "Failed Spi::transfer" );

    for i in 0..read_size {
        println!( "read_data[{}] = 0x{:X}", i, read_data[i] );
    }
}
