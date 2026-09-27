#![no_std]
#![no_main]

use core::ptr::{write_volatile};

use cortex_m::asm::nop;
use cortex_m_rt::{entry};
use panic_halt as _;
// use rtt_target::{rprintln, rtt_init_print};

// const ROW1: *mut u32 = (GPIO_P0_BASE + PIN_21) as *mut u32;
// const COL5: *mut u32 = (GPIO_P0_BASE + PIN_30) as *mut u32;

// GPIO_P0_BASE= 0x50000000
// PIN_21= 0x754 -> ROW1
// PIN_30= 0x778 -> COL5
const ROW1_CNF: *mut u32 = 0x50000754 as *mut u32;
const COL5_CNF: *mut u32 = 0x50000778 as *mut u32;

const GPIO_OUTSET: *mut u32 = 0x50000508 as *mut u32;
const GPIO_OUTCLR: *mut u32 = 0x5000050C as *mut u32;
const PIN_21_OUT: u32 = 21;
const PIN_30_OUT: u32 = 30;


#[entry]
fn main() -> ! {
    unsafe {
        // configure pins as outputs (think arduino)
        write_volatile(ROW1_CNF, 1);
        write_volatile(COL5_CNF, 1);

        write_volatile(GPIO_OUTCLR, 1 << PIN_30_OUT);
    }

    let mut is_on: bool = true;
    loop {
        unsafe {
            if (is_on) {
                write_volatile(GPIO_OUTSET, 1 << PIN_21_OUT);
            } else {

                write_volatile(GPIO_OUTCLR, 1 << PIN_21_OUT);
            }
        }

        for _ in 0..400_000 {
            nop();
        }
        is_on = !is_on;
    }
}
