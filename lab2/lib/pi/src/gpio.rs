use core::marker::PhantomData;

use crate::common::{IO_BASE, states};
use volatile::prelude::*;
use volatile::{Volatile, WriteVolatile, ReadVolatile, Reserved};

/// An alternative GPIO function.
#[repr(u8)]
pub enum Function {
    Input = 0b000,
    Output = 0b001,
    Alt0 = 0b100,
    Alt1 = 0b101,
    Alt2 = 0b110,
    Alt3 = 0b111,
    Alt4 = 0b011,
    Alt5 = 0b010
}

/// There are 6 32-bit FSEL registers which the GPIO pins are grouped into 
/// (registers are mapped to (physical) memory locations (MMIO)). Subindex 
/// each register to get to a specific GPIO pin within its FSEL group. 
/// 10 pins for every FSEL group, except 8 for last.
/// 
/// Each memory address maps to a single byte. Every fifth address maps to the 
/// first 8 bits/1 byte of a set of 32 bits/4 bytes. We only care about a set of 
/// 4 addresses which constitute a 4 byte/32 bit word, which we represent as u32.
#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    FSEL: [Volatile<u32>; 6],
    __r0: Reserved<u32>,
    SET: [WriteVolatile<u32>; 2],
    __r1: Reserved<u32>,
    CLR: [WriteVolatile<u32>; 2],
    __r2: Reserved<u32>,
    LEV: [ReadVolatile<u32>; 2],
    __r3: Reserved<u32>,
    EDS: [Volatile<u32>; 2],
    __r4: Reserved<u32>,
    REN: [Volatile<u32>; 2],
    __r5: Reserved<u32>,
    FEN: [Volatile<u32>; 2],
    __r6: Reserved<u32>,
    HEN: [Volatile<u32>; 2],
    __r7: Reserved<u32>,
    LEN: [Volatile<u32>; 2],
    __r8: Reserved<u32>,
    AREN: [Volatile<u32>; 2],
    __r9: Reserved<u32>,
    AFEN: [Volatile<u32>; 2],
    __r10: Reserved<u32>,
    PUD: Volatile<u32>,
    PUDCLK: [Volatile<u32>; 2],
}

/// Possible states for a GPIO pin.
#[allow(unused_doc_comments)]
states! {
    Uninitialized, Input, Output, Alt
}

/// A GPIO pin in state `State`.
///
/// The `State` generic always corresponds to an uninstantiatable type that is
/// use solely to mark and track the state of a given GPIO pin. A `Gpio`
/// structure starts in the `Uninitialized` state and must be transitions into
/// one of `Input`, `Output`, or `Alt` via the `into_input`, `into_output`, and
/// `into_alt` methods before it can be used.
pub struct Gpio<State> {
    pin: u8,
    registers: &'static mut Registers,
    _state: PhantomData<State>
}

/// The base address of the `GPIO` registers.
const GPIO_BASE: usize = IO_BASE + 0x200000;

impl<T> Gpio<T> {
    /// Transitions `self` to state `S`, consuming `self` and returning a new
    /// `Gpio` instance in state `S`. This method should _never_ be exposed to
    /// the public!
    #[inline(always)]
    fn transition<S>(self) -> Gpio<S> {
        Gpio {
            pin: self.pin,
            registers: self.registers,
            _state: PhantomData
        }
    }
}

impl Gpio<Uninitialized> {
    /// Returns a new `GPIO` structure for pin number `pin`.
    ///
    /// # Panics
    ///
    /// Panics if `pin` > `53`.
    pub fn new(pin: u8) -> Gpio<Uninitialized> {
        if pin > 57 {
            panic!("Gpio::new(): pin {} exceeds maximum of 57", pin);
        }

        Gpio {
            registers: unsafe { &mut *(GPIO_BASE as *mut Registers) },
            pin: pin,
            _state: PhantomData
        }
    }

    /// Enables the alternative function `function` for `self`. Consumes self
    /// and returns a `Gpio` structure in the `Alt` state.
    pub fn into_alt(self, function: Function) -> Gpio<Alt> {
        let outer: usize = (self.pin / 10) as usize;
        let inner = self.pin % 10;
        let inner_shift_multiple = 3;
        let shift_amnt = inner_shift_multiple * inner;
        
        // zero out previous value first, then OR new value in 
        // to zero out prev but keep other bits, AND 1's for save and 0's for destroy
        self.registers.FSEL[outer].and_mask(!(0b111 << shift_amnt));
        self.registers.FSEL[outer].or_mask((function as u32) << shift_amnt);

        Gpio {
            pin: self.pin,
            registers: self.registers,
            _state: PhantomData
        }
    }

    /// Sets this pin to be an _output_ pin. Consumes self and returns a `Gpio`
    /// structure in the `Output` state.
    pub fn into_output(self) -> Gpio<Output> {
        self.into_alt(Function::Output).transition()
    }

    /// Sets this pin to be an _input_ pin. Consumes self and returns a `Gpio`
    /// structure in the `Input` state.
    pub fn into_input(self) -> Gpio<Input> {
        self.into_alt(Function::Input).transition()
    }
}

impl Gpio<Output> {
    /// Sets (turns on) the pin.
    pub fn set(&mut self) {
        if self.pin < 32 {
            // NOTE: just using write for one bit is fine since 
            // writing 0 to other bits has no effect 
            self.registers.SET[0].write(0b1 << self.pin)
        } else {
            self.registers.SET[1].write(0b1 << (self.pin - 32))
        }
    }

    /// Clears (turns off) the pin.
    pub fn clear(&mut self) {
        if self.pin < 32 {
            self.registers.CLR[0].write(0b1 << self.pin)
        } else {
            self.registers.CLR[1].write(0b1 << (self.pin - 32))
        }
    }
}

impl Gpio<Input> {
    /// Reads the pin's value. Returns `true` if the level is high and `false`
    /// if the level is low.
    pub fn level(&mut self) -> bool {
        if self.pin < 32 {
            let mask = 0b1 << self.pin;
            self.registers.LEV[0].has_mask(mask)
        } else {
            let mask = 0b1 << (self.pin - 32);
            self.registers.LEV[1].has_mask(mask)
        }
    }
}
