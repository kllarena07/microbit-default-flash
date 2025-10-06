#![no_std]
#![no_main]

use core::ptr::{read_volatile, write_volatile};

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    const GPIO0_PINCNF0_SPEAKER_ADDR: *mut u32 = 0x5000_0700 as *mut u32;
    const GPIO0_PINCNF14_BTN_A_ADDR: *mut u32 = 0x5000_0738 as *mut u32;
    const GPIO0_PINCNF23_BTN_B_ADDR: *mut u32 = 0x5000_075C as *mut u32;
    const DIR_OUTPUT_POS: u32 = 0;

    unsafe {
        write_volatile(GPIO0_PINCNF0_SPEAKER_ADDR, 1 << DIR_OUTPUT_POS);
        // configure BTN_B (Port 0, Pin 14)
        write_volatile(GPIO0_PINCNF14_BTN_A_ADDR, 0 << DIR_OUTPUT_POS);
        // configure BTN_B (Port 0, Pin 23)
        write_volatile(GPIO0_PINCNF23_BTN_B_ADDR, 0 << DIR_OUTPUT_POS);
    }

    rtt_init_print!();
    rprintln!("Starting...");

    const GPIO0_OUT_ADDR: *mut u32 = 0x5000_0504 as *mut u32;
    const GPIO0_IN_ADDR: *mut u32 = 0x5000_0510 as *mut u32;
    const GPIO0_IN_BTN_A_POS: u32 = 14;
    const GPIO0_IN_BTN_B_POS: u32 = 23;
    const GPIO0_OUT_SPEAKER_POS: u32 = 0;

    fn wind_up() {
        let mut delay = 500;
        unsafe {
            while delay > 0 {
                write_volatile(GPIO0_OUT_ADDR, 1 << GPIO0_OUT_SPEAKER_POS);
                for _ in 0..delay {
                    nop();
                }
                write_volatile(GPIO0_OUT_ADDR, 0 << GPIO0_OUT_SPEAKER_POS);
                for _ in 0..delay {
                    nop();
                }
                delay -= 1;
            }
        }
    }

    fn wind_down() {
        let mut delay = 0;
        unsafe {
            while delay < 600 {
                write_volatile(GPIO0_OUT_ADDR, 1 << GPIO0_OUT_SPEAKER_POS);
                for _ in 0..delay {
                    nop();
                }
                write_volatile(GPIO0_OUT_ADDR, 0 << GPIO0_OUT_SPEAKER_POS);
                for _ in 0..delay {
                    nop();
                }
                delay += 1;
            }
        }
    }

    loop {
        unsafe {
            let input_port_val: u32 = read_volatile(GPIO0_IN_ADDR);
            let gpio0_in_btn_a_val: u32 = (input_port_val >> GPIO0_IN_BTN_A_POS) & 1;
            let gpio0_in_btn_b_val: u32 = (input_port_val >> GPIO0_IN_BTN_B_POS) & 1;

            if gpio0_in_btn_a_val == 0 {
                wind_up();
            }

            if gpio0_in_btn_b_val == 0 {
                wind_down();
            }
        }
    }
}
