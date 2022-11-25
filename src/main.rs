#![no_std]
#![no_main]

mod dice;

use arduino_hal::{
    port::{mode::Output, Pin},
    Pins,
};
use dice::{one};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let mut led_1 = pins.d2.into_output().downgrade();
    let mut led_2 = pins.d3.into_output().downgrade();
    let mut led_3 = pins.d4.into_output().downgrade();
    let mut led_4 = pins.d8.into_output().downgrade();
    let mut led_5 = pins.d11.into_output().downgrade();
    let mut led_6 = pins.d12.into_output().downgrade();
    let mut led_7 = pins.d13.into_output().downgrade();
    loop {
        dice::tow(&mut [&mut led_1,&mut led_2,&mut led_3,&mut led_4,&mut led_5,&mut led_6,&mut led_7])
    }
}

fn sos(led: &mut Pin<Output>) {
    send_signal(led, 3, 1000);
    send_signal(led, 3, 2000);
    send_signal(led, 3, 1000);
}

fn send_signal(led: &mut Pin<Output>, repeat_count: i32, delay: u16) {
    (0..repeat_count).for_each(|_| {
        led.toggle();
        arduino_hal::delay_ms(delay);
    })
}

fn ample(r_pin: &mut Pin<Output>, y_pin: &mut Pin<Output>, g_pin: &mut Pin<Output>) {
    let delay: u16 = 1000;
    g_pin.set_high();
    arduino_hal::delay_ms(delay);
    g_pin.set_low();
    y_pin.set_high();
    arduino_hal::delay_ms(delay);
    y_pin.set_low();
    r_pin.set_high();
    arduino_hal::delay_ms(delay);
    y_pin.set_high();
    arduino_hal::delay_ms(delay);
    r_pin.set_low();
    y_pin.set_low();
}

