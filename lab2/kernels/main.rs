#![feature(alloc_error_handler)]
#![feature(const_fn)]
#![feature(decl_macro)]
#![feature(asm)]
#![feature(global_asm)]
#![feature(optin_builtin_traits)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

//----------------------------
// pi 
#![feature(core_intrinsics)]
#![no_std]
#![feature(never_type)]
//----------------------------

#[cfg(not(test))]
mod init;

pub mod console;
pub mod mutex;
pub mod shell;

use console::kprintln;

//-------------------------------------------------------------
// blinky
// const GPIO_BASE: usize = 0xFE000000 + 0x200000;
// const GPIO_FSEL1: *mut u32 = (GPIO_BASE + 0x04) as *mut u32;
// const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
// const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;
//-------------------------------------------------------------

use pi::timer::spin_sleep;
use core::time::Duration;
use pi::gpio::{Function, Gpio};

unsafe fn kmain() -> ! {
    // FIXME: Start the shell.

    // Christmas light rgb pattern mappings:
    // Brown -> GPIO6 (green rgb)
    // Orange -> GPIO5 (green)
    // Blue -> GPIO13 (red)
    // Purple -> GPIO19 (red)
    let gp6 = Gpio::new(6);
    let mut gp6_ = gp6.into_output();
    let gp5 = Gpio::new(5);
    let mut gp5_ = gp5.into_output();
    let gp13 = Gpio::new(13);
    let mut gp13_ = gp13.into_output();
    let gp19 = Gpio::new(19);
    let mut gp19_ = gp19.into_output();

    loop {
        // turn reds on for a bit
        gp13_.set();
        gp19_.set();
        spin_sleep(Duration::from_millis(150));
        // turn off
        gp13_.clear();
        gp19_.clear();

        // turn on greens
        gp6_.set();
        gp5_.set();
        spin_sleep(Duration::from_millis(150));
        // turn off
        gp6_.clear();
        gp5_.clear();
    }
}
