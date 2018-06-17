extern crate sysfs_gpio;

use std::thread;
use std::time::Duration;
use sysfs_gpio::Pin;

// GPIO BCM pin number
const GPIO_PIN_NUM : u64 = 24;

// Count for blinking an LED 
const BLINK_CNT : u8 = 10;

// GPIO setting value
const LOW : u8  = 0;
const HIGH : u8 = 1;


fn main() {
    let pin = Pin::new( GPIO_PIN_NUM );
    let mut blinking_cnt = 0;

    pin.with_exported( || {

        // Toggle GPIO pinfor blinking an LED
        while blinking_cnt < BLINK_CNT {
            pin.set_value( LOW ).expect( "Failed Pin::set_value" );
            thread::sleep( Duration::from_secs( 1 ) );

            pin.set_value( HIGH ).expect( "Failed Pin::set_value" );
            thread::sleep( Duration::from_secs( 1 ) );
            blinking_cnt = blinking_cnt + 1;
        }

        Ok( () )
    } ).expect( "Failed Pin::with_exported" );
}