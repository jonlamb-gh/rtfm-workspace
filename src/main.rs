#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate stm32f4xx_hal as hal;

use crate::hal::stm32;
//use crate::hal::prelude::*;
use cortex_m_semihosting::{debug, hprintln};
use panic_semihosting as _;

#[rtfm::app(device = crate::hal::stm32, peripherals = true)]
const APP: () = {
    #[init]
    fn init(cx: init::Context) {
        static mut X: u32 = 0;

        // Cortex-M peripherals
        let _core: cortex_m::Peripherals = cx.core;

        // Device specific peripherals
        let _device: stm32::Peripherals = cx.device;

        // Safe access to local `static mut` variable
        let _x: &'static mut u32 = X;

        hprintln!("init").unwrap();

        //debug::exit(debug::EXIT_SUCCESS);
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        static mut X: u32 = 0;

        // Safe access to local `static mut` variable
        let _x: &'static mut u32 = X;

        hprintln!("idle").unwrap();

        debug::exit(debug::EXIT_SUCCESS);

        loop {}
    }
};
