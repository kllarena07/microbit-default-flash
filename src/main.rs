#![no_std]
#![no_main]

use core::ptr::write_volatile;

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    const DIR_OUTPUT_POS: u32 = 0;

    const ROWS: [*mut u32; 5] = [
        0x5000_0754 as *mut u32, // ROW 1
        0x5000_0758 as *mut u32, // ROW 2
        0x5000_073C as *mut u32, // ROW 3
        0x5000_0760 as *mut u32, // ROW 4
        0x5000_074C as *mut u32, // ROW 5
    ];

    const ROW_PIN_NUMBS: [u32; 5] = [
        21, // ROW 1
        22, // ROW 2
        15, // ROW 3
        24, // ROW 4
        19, // ROW 5
    ];

    const COLUMNS: [*mut u32; 5] = [
        0x5000_0770 as *mut u32, // COL 1
        0x5000_072C as *mut u32, // COL 2
        0x5000_077C as *mut u32, // COL 3
        0x5000_0A14 as *mut u32, // COL 4, P1 (0x50000300); PIN 5 (0x714); 0x50000300 + 0x714
        0x5000_0778 as *mut u32, // COL 5
    ];

    unsafe {
        write_volatile(ROWS[1], 1 << DIR_OUTPUT_POS); // row 2
        write_volatile(ROWS[3], 1 << DIR_OUTPUT_POS); // row 4
        write_volatile(ROWS[4], 1 << DIR_OUTPUT_POS); // row 5

        // this is setting the VALUES at these memory addresses to '1'
    }
    const GPIO0_OUT_ADDR: *mut u32 = 0x5000_0504 as *mut u32;

    rtt_init_print!();
    rprintln!("Starting...");

    const DELAY: u32 = 1_000;

    let is_smiling: bool = false;

    fn smile() {
        unsafe {
            write_volatile(GPIO0_OUT_ADDR, 1 << ROW_PIN_NUMBS[1]);
            write_volatile(COLUMNS[0], 0 << DIR_OUTPUT_POS);
            write_volatile(COLUMNS[1], 1 << DIR_OUTPUT_POS);
            write_volatile(COLUMNS[2], 0 << DIR_OUTPUT_POS);
            write_volatile(COLUMNS[3], 1 << DIR_OUTPUT_POS);
            write_volatile(COLUMNS[4], 0 << DIR_OUTPUT_POS);

            for _ in 0..DELAY {
                nop();
            }

            // Drawing the "cheeks"
            write_volatile(GPIO0_OUT_ADDR, 1 << ROW_PIN_NUMBS[3]);
            write_volatile(COLUMNS[0], 1 << DIR_OUTPUT_POS);
            write_volatile(COLUMNS[1], 0 << DIR_OUTPUT_POS);
            write_volatile(COLUMNS[2], 0 << DIR_OUTPUT_POS);
            write_volatile(COLUMNS[3], 0 << DIR_OUTPUT_POS);
            write_volatile(COLUMNS[4], 1 << DIR_OUTPUT_POS);

            for _ in 0..DELAY {
                nop();
            }

            // Drawing row 5
            write_volatile(GPIO0_OUT_ADDR, 1 << ROW_PIN_NUMBS[4]);
            write_volatile(COLUMNS[0], 0 << DIR_OUTPUT_POS);
            write_volatile(COLUMNS[1], 1 << DIR_OUTPUT_POS);
            write_volatile(COLUMNS[2], 1 << DIR_OUTPUT_POS);
            write_volatile(COLUMNS[3], 1 << DIR_OUTPUT_POS);
            write_volatile(COLUMNS[4], 0 << DIR_OUTPUT_POS);

            for _ in 0..DELAY {
                nop();
            }
        }
    }

    fn frown() {
        unsafe {
            write_volatile(GPIO0_OUT_ADDR, 1 << ROW_PIN_NUMBS[1]);
            write_volatile(COLUMNS[0], 0 << DIR_OUTPUT_POS);
            write_volatile(COLUMNS[1], 1 << DIR_OUTPUT_POS);
            write_volatile(COLUMNS[2], 0 << DIR_OUTPUT_POS);
            write_volatile(COLUMNS[3], 1 << DIR_OUTPUT_POS);
            write_volatile(COLUMNS[4], 0 << DIR_OUTPUT_POS);

            for _ in 0..DELAY {
                nop();
            }

            // Drawing the "cheeks"
            write_volatile(GPIO0_OUT_ADDR, 1 << ROW_PIN_NUMBS[3]);
            write_volatile(COLUMNS[0], 0 << DIR_OUTPUT_POS);
            write_volatile(COLUMNS[1], 1 << DIR_OUTPUT_POS);
            write_volatile(COLUMNS[2], 1 << DIR_OUTPUT_POS);
            write_volatile(COLUMNS[3], 1 << DIR_OUTPUT_POS);
            write_volatile(COLUMNS[4], 0 << DIR_OUTPUT_POS);

            for _ in 0..DELAY {
                nop();
            }

            // Drawing row 5
            write_volatile(GPIO0_OUT_ADDR, 1 << ROW_PIN_NUMBS[4]);
            write_volatile(COLUMNS[0], 1 << DIR_OUTPUT_POS);
            write_volatile(COLUMNS[1], 0 << DIR_OUTPUT_POS);
            write_volatile(COLUMNS[2], 0 << DIR_OUTPUT_POS);
            write_volatile(COLUMNS[3], 0 << DIR_OUTPUT_POS);
            write_volatile(COLUMNS[4], 1 << DIR_OUTPUT_POS);

            for _ in 0..DELAY {
                nop();
            }
        }
    }

    loop {
        if is_smiling {
            smile();
        } else {
            frown();
        }
    }
}
