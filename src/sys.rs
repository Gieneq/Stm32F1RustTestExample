use cortex_m_semihosting::debug;

/// Terminates the application and makes a semihosting-capable debug tool exit
/// with status code 0.
pub fn exit() -> ! {
    loop {
        debug::exit(debug::EXIT_SUCCESS);
    }
}
