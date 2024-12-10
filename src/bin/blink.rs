#![no_std]
#![no_main]

use cortex_m_rt::entry;
use stm32f1xx_hal::{prelude::*, flash::FlashExt, gpio::GpioExt, pac, rcc::RccExt, time::Hertz, timer::Timer, timer};

// defmt used for logging
use defmt_rtt as _;
use defmt::{info, error};

#[allow(unused)]
use approach_f1::sys;


#[entry]
fn main() -> ! {
    info!("Starting!");

    // Core peripherals
    let pc = cortex_m::Peripherals::take().unwrap();

    // Device specific peripherals
    let pd = pac::Peripherals::take().unwrap();

    let mut flash = pd.FLASH.constrain();

    let rcc = pd.RCC.constrain();

    // Adjust clocks
    // HCLK is SYSCLK * presc
    // Cortex Systick is HCLK / 8
    let clocks = rcc
        .cfgr
        .use_hse(8.MHz())
        .sysclk(72.MHz())
        .freeze(&mut flash.acr);

    defmt::println!("Clocks[kHz]: hclk={}, pclk1={}, pclk2={}, sysclk={}, adcclk={}", 
        clocks.hclk().to_kHz(),
        clocks.pclk1().to_kHz(),
        clocks.pclk2().to_kHz(),
        clocks.sysclk().to_kHz(),
        clocks.adcclk().to_kHz()
    );

    defmt::println!("APB1 Timers: TIM2-7, TIM12-14 = {} kHz", clocks.pclk1_tim().to_kHz());
    defmt::println!("APB2 Timers: TIM1,   TIMM8-11 = {} kHz", clocks.pclk2_tim().to_kHz());

    let mut gpioa = pd.GPIOA.split();

    // The `crh` register is for pins 15..8. crl is for pins 0-7.
    let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);

    let mut timer = Timer::syst_external(pc.SYST, &clocks).counter_hz();
    match timer.start(Hertz::Hz(1)) {
        Ok(()) => {},
        Err(timer::Error::WrongAutoReload) => {
            error!("Error::WrongAutoReload");
        },
        Err(_) => {
            error!("Unknown error");
        },
    };

    let mut ticks_counter = Timer::new(pd.TIM2, &clocks).counter_hz();
    ticks_counter.start(1.kHz()).unwrap();
    
    let mut last_high_log_millis = 0u32;
    let mut last_low_log_millis = 0u32;

    loop {
        ticks_counter.reset();
        led.set_high();
        nb::block!(timer.wait()).unwrap();
        let high_state_millis = ticks_counter.now().ticks();

        ticks_counter.reset();
        info!("High took {} ms, last log took {} ms", high_state_millis, last_high_log_millis);
        last_high_log_millis = ticks_counter.now().ticks();

        ticks_counter.reset();
        led.set_low();
        nb::block!(timer.wait()).unwrap();
        let low_state_millis = ticks_counter.now().ticks();

        ticks_counter.reset();
        info!("Low took {} ms, last log took {} ms", low_state_millis, last_low_log_millis);
        last_low_log_millis = ticks_counter.now().ticks();
    }
}
