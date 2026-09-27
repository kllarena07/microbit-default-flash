#![no_std]
#![no_main]

use core::ptr::{write_volatile};

use cortex_m_rt::{entry};
use panic_halt as _;

// COL5 CONFIG
const ROW1_CNF: *mut u32 = 0x50000754 as *mut u32;
const GPIO_OUTSET: *mut u32 = 0x50000508 as *mut u32;
const PIN_21_OUT: u32 = 21;

// TIMER0 = 0x40008000
// EVENTS_COMPARE[0] = 0x140
// SO:
const EVENTS_COMPARE0: u32 = 0x40008140;
const TIMER0_CC0: *mut u32 = 0x40008540 as * mut u32;
const TIMER0_TASKS_START: *mut u32 = 0x40008000 as *mut u32;
const TIMER0_TASKS_CLEAR: u32 = 0x4000800C;
const TIMER0_BITMODE: *mut u32 = 0x4000_8508 as *mut u32;

// GPIO
const GPIOTE_CONFIG0: *mut u32 = 0x4000_6510 as *mut u32;

const GPIOTE_MODE_TASK: u32 = 0b11;
const GPIOTE_POLARITY_TOGGLE: u32 = 0b11;
const COL5_PIN: u32 = 30;

// bits  0–1: MODE       0b11 = Task mode
// bits  8–12: PSEL      30 = P0.30
// bits 16–17: POLARITY  0b11 = toggle for every TASKS_OUT[0]
const GPIOTE_CONFIG0_TOGGLE_COL5: u32 =
    GPIOTE_MODE_TASK
    | (COL5_PIN << 8)
    | (GPIOTE_POLARITY_TOGGLE << 16);
const GPIOTE_TASKS_OUT0: u32 = 0x40006000;

// PPI = 0x4001F000
const PPI_CHENSET: *mut u32 = 0x4001F504 as *mut u32;
// CH[0].EEP = 0x510
// CH[0].TEP = 0x514
const CH0_EEP: *mut u32 = 0x4001F510 as *mut u32;
const CH0_TEP: *mut u32 = 0x4001F514 as *mut u32;

// CH[1].EEP = 0x518
// CH[1].TEP = 0x51C
const CH1_EEP: *mut u32 = 0x4001F518 as *mut u32;
const CH1_TEP: *mut u32 = 0x4001F51C as *mut u32;

#[entry]
fn main() -> ! {
    unsafe {
        // drive ROW1 HIGH
        write_volatile(ROW1_CNF, 1);
        write_volatile(GPIO_OUTSET, 1 << PIN_21_OUT);

        // configure GPIOTE.CONFIG[0] 
        write_volatile(GPIOTE_CONFIG0, GPIOTE_CONFIG0_TOGGLE_COL5);

        // configure TIMER0 to work with 32 bits
        write_volatile(TIMER0_BITMODE, 3);
        
        // set CC[0] to be 16000000
        write_volatile(TIMER0_CC0, 1_000_000);

        // configure PPI CH0
        write_volatile(CH0_EEP, EVENTS_COMPARE0);
        write_volatile(CH0_TEP, TIMER0_TASKS_CLEAR);

        // configure PPI CH1
        write_volatile(CH1_EEP, EVENTS_COMPARE0);
        write_volatile(CH1_TEP, GPIOTE_TASKS_OUT0);

        // enable PPI
        write_volatile(PPI_CHENSET, 0b11);

        // start TIMER0
        write_volatile(TIMER0_TASKS_START, 1);
    }

    loop {}
}
