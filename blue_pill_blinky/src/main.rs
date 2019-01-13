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

#[entry]
fn main() -> ! {

    let systick = unsafe { &mut *(0xE000_E010 as *mut SysTick) };

    // Reload  Value Register set to 0x00FFFFFF
    // when timer starts or reachs 0, set automatically set is back to this value
    unsafe { core::ptr::write_volatile(&mut systick.rvr, 0x00FFFFFF) };
    
    // Timer Control Register set so:
    // -Timer uses processor clock
    // -No exception is raised when value reaches zero
    // -Counter is enabled
    unsafe { core::ptr::write_volatile(&mut systick.cvr, 0b000000000000000_0_0000000000000_101) };

    loop {
        let current_value_register = unsafe { core::ptr::read_volatile(&mut systick.cvr) };
        hprintln!("System timer current value is now {}.", current_value_register).unwrap();
    }
}
