#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_halt;

use cortex_m_rt::entry;

#[entry] //~ ERROR custom attribute panicked
//~^ HELP `#[entry]` function must have signature `[unsafe] fn() -> !`
fn foo(undef: i32) -> ! {}
