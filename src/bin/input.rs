#![no_std]
#![no_main]

use cortex_m_rt::entry;
use stm32f1xx_hal::{flash::FlashExt, gpio::{GpioExt, Input, Pin, PullDown, PullUp}, pac, prelude::*};

use defmt_rtt as _;
use defmt::info;

#[allow(unused)]
use approach_f1::sys;

enum ButtonState {
    Pressed,
    Released,
}

struct Button<const P: char, const N: u8, Pull> {
    pin: Pin<P, N, Input<Pull>>,
}

impl<const P: char, const N: u8, Pull> Button<P, N, Pull> {
    pub fn from_input_pin(pin: Pin<P, N, Input<Pull>>) -> Self {
        Self { pin }
    }
}

impl<const P: char, const N: u8> Button<P, N, PullUp> {
    pub fn get_state(&self) -> ButtonState {
        match self.pin.is_low() {
            true => ButtonState::Pressed,
            false => ButtonState::Released,
        }
    }
}

impl<const P: char, const N: u8> Button<P, N, PullDown> {
    #[allow(dead_code)]
    pub fn get_state(&self) -> ButtonState {
        match self.pin.is_high() {
            true => ButtonState::Pressed,
            false => ButtonState::Released,
        }
    }
}




#[entry]
fn main() -> ! {
    let perip_core = cortex_m::Peripherals::take().unwrap();

    let perip_device = pac::Peripherals::take().unwrap();

    let mut flash = perip_device.FLASH.constrain();

    let rcc = perip_device.RCC.constrain();

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

    let mut gpioa = perip_device.GPIOA.split();
    let mut gpioc = perip_device.GPIOC.split();

    let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);
    let btn_pin = gpioc.pc13.into_pull_up_input(&mut gpioc.crh);
    let btn = Button::from_input_pin(btn_pin);

    let mut delay = perip_core.SYST.delay(&clocks);

    let mut btn_state_transition = (btn.get_state(), btn.get_state());

    loop {

        btn_state_transition.1 = btn.get_state();

        match btn_state_transition {
            (ButtonState::Pressed, ButtonState::Released) => {
                info!("CLicked!");
                led.set_high();
                delay.delay_ms(500u32);
                led.set_low();
            },
            _ => { }
        }

        btn_state_transition.0 = btn_state_transition.1;
    }

}