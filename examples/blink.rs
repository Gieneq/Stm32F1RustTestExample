#![no_std]
#![no_main]

// use core::panic::PanicInfo;
use cortex_m_rt::entry;
use stm32f1xx_hal as _;

// defmt used for logging
use defmt_rtt as _;
use defmt::info;

use approach_f1::sys;

#[entry]
fn main() -> ! {

    info!("Helllllllo");

    defmt::println!("Hello, world!");

    sys::exit()
}
