use crate::common::IO_BASE;
use core::time::Duration;

use volatile::prelude::*;
use volatile::{Volatile, ReadVolatile};

/// The base address for the ARM system timer registers.
const TIMER_REG_BASE: usize = IO_BASE + 0x3000;

#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    CS: Volatile<u32>,
    CLO: ReadVolatile<u32>,
    CHI: ReadVolatile<u32>,
    COMPARE: [Volatile<u32>; 4]
}

/// The Raspberry Pi ARM system timer.
pub struct Timer {
    registers: &'static mut Registers
}

impl Timer {
    /// Returns a new instance of `Timer`.
    pub fn new() -> Timer {
        Timer {
            registers: unsafe { &mut *(TIMER_REG_BASE as *mut Registers) },
        }
    }

    /// Reads the system timer's counter and returns Duration.
    /// `CLO` and `CHI` together can represent the number of elapsed microseconds.
    pub fn read(&self) -> Duration {
        let low = self.registers.CLO.read();
        let high: u64 = (self.registers.CHI.read() as u64) << 32;
        Duration::from_micros(high + (low as u64))
    }
}

/// Returns current time.
pub fn current_time() -> Duration {
    let timer = Timer::new();
    timer.read()
}

/// Spins until `t` duration have passed.
pub fn spin_sleep(t: Duration) {
    let start = current_time();
    loop {
        // assume if `t` is secs and `elapsed` is ms the partialeq trait impl handles it 
        if let Some(elapsed) = current_time().checked_sub(start.clone()) {
            if (&elapsed).ge(&t) {
                return;
            }
        } else {
            // if negative, sub goes to None, but sub cannot be negative by logic
            // sub overflows to None, in which case assume `t` passed 
            return;
        }
    }
}

