#![no_main]
#![no_std]

extern crate panic_halt;

use lpc8xx_hal::cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    loop {
        lpc8xx_hal::cortex_m::asm::nop();
    }
}
