#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_halt;
extern crate stm32f1xx_hal as hal;
#[macro_use(block)]
extern crate nb;
extern crate embedded_hal;

use hal::prelude::*;
use hal::stm32;
use hal::timer::Timer;
use rt::{entry};
use cortex_m::peripheral::SYST;
use embedded_hal::digital::OutputPin;

#[entry]
fn main() -> ! {

    // Get control of the PC13 pin
    let device_peripherals = stm32::Peripherals::take().unwrap();
    let mut rcc = device_peripherals.RCC.constrain();
    let mut gpioc = device_peripherals.GPIOC.split(&mut rcc.apb2);
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let cortex_peripherals = cortex_m::Peripherals::take().unwrap();
    let mut flash = device_peripherals.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut timer = Timer::syst(cortex_peripherals.SYST, 5.hz(), clocks);

    led.set_high();
    loop {
        blink(&mut timer, &mut led, 2);
        wait(&mut timer, 10);
    }
}

fn blink(timer: &mut Timer<SYST>, led: &mut OutputPin, times: usize) -> () {
    for _n in 0..times {
        led.set_low();
        block!(timer.wait()).unwrap();
        led.set_high();
        block!(timer.wait()).unwrap();
    }
}

fn wait(timer: &mut Timer<SYST>, times: usize) -> () {
    for _n in 0..(times*2) {
        block!(timer.wait()).unwrap();
    }
}

