//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

#[repr(C)]
struct SysTick {
    pub csr: u32,
    pub rvr: u32,
    pub cvr: u32,
    pub calib: u32,
}

#[repr(C)]
struct PortConfiguration {
    pub GPIOx_CRL: u32,
    pub GPIOx_CRH: u32,
    pub GPIOx_IDR: u32,
    pub GPIOx_ODR: u32,
    pub GPIOx_BSRR: u32,
    pub GPIOx_BRR: u32,
    pub GPIOx_LCKR: u32,
}

const PORT_C_BASE_ADDRESS: u32 = 0x4001_1000;
const RCC_APB2ENR_ADDRESS: u32 = 0x4002_1000 + 0x18;

#[entry]
fn main() -> ! {

    let systick = unsafe { &mut *(0xE000_E010 as *mut SysTick) };
    let port_c_sfr = unsafe { &mut *(PORT_C_BASE_ADDRESS as *mut PortConfiguration) };

    // Enables IO port C clock, disable many other that are probably already disabled.
    unsafe { core::ptr::write_volatile(RCC_APB2ENR_ADDRESS as *mut u32, 1 << 4) };

    // Reload  Value Register set to 0x00FFFFFF
    // when timer starts or reachs 0, set automatically set is back to this value
    unsafe { core::ptr::write_volatile(&mut systick.rvr, 0x00FFFFFF) };
    
    // Timer Control and Status Register set so:
    // -Timer uses processor clock
    // -No exception is raised when value reaches zero
    // -Counter is enabled
    unsafe { core::ptr::write_volatile(&mut systick.csr, 0b000000000000000_0_0000000000000_101) };

    // Port Configuration Register High for Port E:
    // -everything is floating input, exceptpin PC13 which is open drain output.
    unsafe { core::ptr::write_volatile(&mut port_c_sfr.GPIOx_CRH, 0b0100_0100_0110_0100_0100_0100_0100_0100 ) };

    loop {

        unsafe { core::ptr::write_volatile(&mut port_c_sfr.GPIOx_ODR, 0b0000000000000000_0010000000000000 ) };

        let current_value_register = unsafe { core::ptr::read_volatile(&mut systick.cvr) };
        hprintln!("System timer current value is now {}.", current_value_register).unwrap();

        unsafe { core::ptr::write_volatile(&mut port_c_sfr.GPIOx_ODR, 0b0000000000000000_0000000000000000 ) };

        let current_value_register = unsafe { core::ptr::read_volatile(&mut systick.cvr) };
        hprintln!("System timer current value is now {}.", current_value_register).unwrap();
    }
}
