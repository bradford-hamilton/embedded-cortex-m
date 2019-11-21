// #![no_std] indicates that this program will not link to the standard
// crate, std. Instead it will link to its subset: the core crate.
#![no_std]

// #![no_main] indicates that this program won't use the standard main
// interface that most Rust programs use. The main (no pun intended)
// reason to go with no_main is that using the main interface in no_std
// context requires nightly.
#![no_main]

// Panicking behaviors
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;

// #[entry] is an attribute provided by the cortex-m-rt crate that's
// used to mark the entry point of the program. As we are not using
// the standard main interface we need another way to indicate the
// entry point of the program and that'd be #[entry].
#[entry]
// Our program will be the only process running on the target hardware
// so we don't want it to end! We use a divergent function (the -> ! bit
// in the function signature) to ensure at compile time that'll be the case.
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    loop {}
}
