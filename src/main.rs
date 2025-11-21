#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_halt as _;
use ra4m1::Peripherals;

#[entry]
fn main() -> ! {
    // Get access to the peripherals
    let p = unsafe { Peripherals::steal() };
    // Set p102 as an output
    p.PORT1.pdr().write(|w| unsafe { w.bits(1 << 2) });

    loop {
        // Set output high
        p.PORT1.podr().write(|w| unsafe { w.bits(1 << 2) });
        for _ in 0..50_000 {
            // safe alternative: `cortex_m::asm::nop()`
            asm::nop();
        }
        // Set output low
        p.PORT1.podr().write(|w| unsafe { w.bits(0) });
        for _ in 0..50_000 {
            // safe alternative: `cortex_m::asm::nop()`
            asm::nop();
        }
    }
}
