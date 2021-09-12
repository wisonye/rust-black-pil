#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use rtt_target::{rprintln, rtt_init_print};
// Halt on panic
// use panic_halt as _; // panic handler
use panic_rtt_target as _;

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    // Init buffers for debug printing
    rtt_init_print!();

    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the LED. On the Nucleo-446RE it's connected to pin PA5.
        let gpioc = dp.GPIOC.split();
        let mut led = gpioc.pc13.into_push_pull_output();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        // Create a delay abstraction based on SysTick
        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

        rprintln!(">>> Ready");
        loop {
            // On for 1s, off for 1s.
            let _ = led.set_high();
            rprintln!(">>> On");
            delay.delay_ms(500_u32);
            let _ = led.set_low();
            rprintln!(">>> Off");
            delay.delay_ms(500_u32);
        }
    }

    loop {}
}
