#![feature(array_value_iter)]
#![no_std]
#![no_main]

extern crate panic_msp430;

use msp430::asm;
use msp430_rt::entry;

use msp430_periph::devices::msp430f5529::MSP430F5529;
use msp430_periph::peripherals::{portb_1i1 as p1, portb_1 as p4, watchdog_timer_2 as wdt};
use msp430_periph::utils::Value;

#[entry]
fn main() -> ! {
    let mut p: MSP430F5529 = unsafe { core::mem::transmute(()) };

    // Disable watchdog
    p.watchdog_timer
        .ctl
        .write(unsafe { Value::from_raw(0x5a00) } | wdt::HOLD(true));

    // Set P1.0 and P4.7 as output
    p.port_1.out.modify(p1::OUT0(false));
    p.port_4.out.modify(p4::OUT7(true));
    p.port_1.dir.modify(p1::DIR0(true));
    p.port_4.dir.modify(p4::DIR7(true));

    loop {
        let mut i = 0u16;
        while i < 10_000u16 {
            i += 1;
            asm::nop();
        }

        // Toggle outputs
        p.port_1.out.toggle(p1::OUT::OUT0);
        p.port_4.out.toggle(p4::OUT::OUT7);
    }
}

#[no_mangle]
extern "C" fn abort() -> ! {
    panic!();
}
