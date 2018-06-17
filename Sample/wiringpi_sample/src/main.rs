extern crate wiringpi;

use std::thread;
use std::time::Duration;
use wiringpi::pin::Value::{High, Low};

// WiringPi Pin number for GPIO24
const WPI_GPIO24_PIN_NUM : u16 = 5;

// Count for blinking an LED 
const BLINK_CNT : u8 = 10;

fn main() {
    let wiringpi = wiringpi::setup();
    let pin = wiringpi.output_pin( WPI_GPIO24_PIN_NUM );
    let mut blinking_cnt = 0;
    
    // Toggle GPIO pin for blinking an LED
    while blinking_cnt < BLINK_CNT {
        pin.digital_write( High );
        thread::sleep( Duration::from_secs( 1 ) );

        pin.digital_write( Low );
        thread::sleep( Duration::from_secs( 1 ) );
        blinking_cnt = blinking_cnt + 1;
    }
}
