//#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_halt;
extern crate stm32f103xx_hal;

use cortex_m_rt::entry;
use nb::block;
use crate::stm32f103xx_hal::{
    prelude::*,
    device,
    timer::Timer,
};

#[entry]
fn main() -> ! {

    // Get control of the PC13 pin
    let device_peripherals = device::Peripherals::take().unwrap();
    let mut rcc = device_peripherals.RCC.constrain();
    let mut gpioc = device_peripherals.GPIOC.split(&mut rcc.apb2);
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let cortex_peripherals = cortex_m::Peripherals::take().unwrap();
    let mut flash = device_peripherals.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut timer = Timer::syst(cortex_peripherals.SYST, 5.hz(), clocks);

    loop {
        block!(timer.wait()).unwrap();
        block!(timer.wait()).unwrap();
        block!(timer.wait()).unwrap();
        block!(timer.wait()).unwrap();
        block!(timer.wait()).unwrap();
        led.set_high();
        block!(timer.wait()).unwrap();
        led.set_low();
        block!(timer.wait()).unwrap();
        led.set_high();
        block!(timer.wait()).unwrap();
        led.set_low();
    }
}

