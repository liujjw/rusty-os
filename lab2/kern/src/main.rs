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

// FIXME: You need to add dependencies here to
// test your drivers (Phase 2). Add them as needed.

//-------------------------------------------------------------
// blinky
const GPIO_BASE: usize = 0xFE000000 + 0x200000;
const GPIO_FSEL1: *mut u32 = (GPIO_BASE + 0x04) as *mut u32;
const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;
use pi::timer::spin_sleep;
use core::time::Duration;
//-------------------------------------------------------------

unsafe fn kmain() -> ! {
    // FIXME: Start the shell.

    loop {
        GPIO_FSEL1.write_volatile(0b1 << 18);    
        GPIO_SET0.write_volatile(0b1 << 16);
        spin_sleep(Duration::new(3, 0));
        GPIO_CLR0.write_volatile(0b1 << 16);
        spin_sleep(Duration::new(3, 0));
    }
}
