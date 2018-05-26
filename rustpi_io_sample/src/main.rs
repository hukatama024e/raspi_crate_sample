extern crate rustpi_io;

use std::thread;
use std::time::Duration;
use rustpi_io::gpio::{GPIO, GPIOData, GPIOMode};

// GPIO BCM pin number
const GPIO_PIN_NUM : u8 = 24;

// Count for blinking an LED 
const BLINK_CNT : u8 = 10;

fn main() {
    let gpio = GPIO::new( GPIO_PIN_NUM, GPIOMode::Write ).expect( "Failed GPIO::new()" );
    let mut blinking_cnt = 0;
    
    // Toggle GPIO pinfor blinking an LED
    while blinking_cnt < BLINK_CNT {
        gpio.set( GPIOData::High ).expect( "Failed GPIO.set()" );
        thread::sleep( Duration::from_secs( 1 ) );

        gpio.set( GPIOData::Low ).expect( "Failed GPIO.set()" );
        thread::sleep( Duration::from_secs( 1 ) );
        blinking_cnt = blinking_cnt + 1;
    }
}
