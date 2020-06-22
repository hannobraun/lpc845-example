#![no_main]
#![no_std]

extern crate panic_halt;

use lpc8xx_hal::cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let p = lpc8xx_hal::Peripherals::take().unwrap();

    let mut syscon = p.SYSCON.split();
    let     gpio   = p.GPIO.enable(&mut syscon.handle);

    let button = p.pins.pio0_4.into_input_pin(gpio.tokens.pio0_4);
    let mut led = p.pins.pio1_1.into_output_pin(
        gpio.tokens.pio1_1,
        lpc8xx_hal::gpio::Level::High,
    );

    loop {
        if button.is_high() {
            led.set_high();
        }
        else {
            led.set_low();
        }
    }
}
