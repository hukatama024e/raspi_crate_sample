extern crate rppal;

use std::thread;
use std::time::Duration;
use rppal::gpio::{Gpio, Mode, Level};

// GPIO BCM pin number
const GPIO_PIN_NUM : u8 = 24;

// Count for blinking an LED 
const BLINK_CNT : u8 = 10;

fn main() {
    let mut gpio = Gpio::new().expect( "Failed Gpio::new()" );
    let mut blinking_cnt = 0;

    // Change GPIO mode to output
    gpio.set_mode( GPIO_PIN_NUM, Mode::Output );

    // Toggle GPIO pinfor blinking an LED
    while blinking_cnt < BLINK_CNT {
        gpio.write( GPIO_PIN_NUM, Level::High );
        thread::sleep( Duration::from_secs( 1 ) );

        gpio.write( GPIO_PIN_NUM, Level::Low );
        thread::sleep( Duration::from_secs( 1 ) );
        blinking_cnt = blinking_cnt + 1;
    }

    // Return GPIO mode to input
    gpio.set_mode( GPIO_PIN_NUM, Mode::Input );
}
