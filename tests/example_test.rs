#![no_std]
#![no_main]
   
#[cfg(test)]
#[embedded_test::tests]
mod tests {
    use defmt_rtt as _;
    use stm32f1xx_hal::pac::Peripherals;
 
    #[test]
    fn log() {
        let _p = Peripherals::take().unwrap();
        assert!(true);
    }
}