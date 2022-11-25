use arduino_hal::port::{mode::Output, Pin};

pub fn one(leds: &mut [&mut Pin<Output>; 7]){
    reset_dice(leds);
    leds[3].set_high();
}

pub fn tow(leds: &mut [&mut Pin<Output>; 7]){
    reset_dice(leds);
    leds[0].set_high();
    leds[6].set_high();
}

pub fn three(leds: &mut [&mut Pin<Output>; 7]){
    reset_dice(leds);
    leds[0].set_high();
    leds[3].set_high();
    leds[6].set_high();
}

pub fn four(leds: &mut [&mut Pin<Output>; 7]){
    reset_dice(leds);
    leds[0].set_high();
    leds[2].set_high();
    leds[4].set_high();
    leds[6].set_high();
}

pub fn five(leds: &mut [&mut Pin<Output>; 7]){
    reset_dice(leds);
    leds[3].set_high();
    leds[0].set_high();
    leds[2].set_high();
    leds[4].set_high();
    leds[6].set_high();
}

pub fn six(leds: &mut [&mut Pin<Output>; 7]){
    reset_dice(leds);
    leds[0].set_high();
    leds[1].set_high();
    leds[2].set_high();
    leds[4].set_high();
    leds[5].set_high();
    leds[6].set_high();
}

pub fn reset_dice(leds: &mut [&mut Pin<Output>; 7]){
    leds.iter_mut().for_each(|led|{
        led.set_high();
    });
}