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

use pi::uart::MiniUart;
use core::fmt::Write;

unsafe fn kmain() -> ! {
    let mut mu = MiniUart::new();
    loop {
        let read = mu.read_byte();
        mu.write_byte(read);
        mu.write_str("<-");
    }
}
