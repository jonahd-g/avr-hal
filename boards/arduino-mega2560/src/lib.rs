#![no_std]

// Expose hal & pac crates
pub use atmega2560_hal as hal;
pub use crate::hal::pac;

/// See [`avr_device::entry`](https://docs.rs/avr-device/latest/avr_device/attr.entry.html).
#[cfg(feature = "rt")]
pub use crate::hal::entry;

pub use crate::pac::Peripherals;

mod pins;
pub use crate::pins::*;

pub mod prelude {
    pub use crate::hal::prelude::*;
    pub use crate::hal::usart::BaudrateExt as _;
}

pub use crate::hal::spi;
pub use crate::hal::adc;

pub type Delay = crate::hal::delay::Delay<hal::clock::MHz16>;
pub type Serial<IMODE> = crate::usart::Usart0<IMODE>;

pub mod usart {
    pub use avr_hal_generic::usart::*;

    pub type Usart0<IMODE> = crate::hal::usart::Usart0<crate::hal::clock::MHz16, IMODE>;
    pub type Usart1<IMODE> = crate::hal::usart::Usart1<crate::hal::clock::MHz16, IMODE>;
    pub type Usart2<IMODE> = crate::hal::usart::Usart2<crate::hal::clock::MHz16, IMODE>;
    pub type Usart3<IMODE> = crate::hal::usart::Usart3<crate::hal::clock::MHz16, IMODE>;
}

pub type I2c<M> = crate::hal::i2c::I2c<hal::clock::MHz16, M>;

/// Support for PWM pins
///
/// The 6 timers of ATmega2560 can be used for PWM on certain pins.
/// The PWM methods are from `embedded_hal::PwmPin`.
///
/// # Example
/// For a full example, see [`examples/mega2560-pwm.rs`][ex-pwm].  In short:
/// ```
/// let mut pins = arduino_mega2560::Pins::new(
///     dp.PORTA, dp.PORTB, dp.PORTC, dp.PORTD,
///     dp.PORTE, dp.PORTF, dp.PORTG, dp.PORTH,
///     dp.PORTJ, dp.PORTK, dp.PORTL,
/// );
///
/// let mut timer0 = arduino_mega2560::pwm::Timer0Pwm::new(
///     dp.TC0,
///     arduino_mega2560::pwm::Prescaler::Prescale64,
/// );
///
/// let mut pin = pins.d13.into_output(&mut pins.ddr).into_pwm(&mut timer0);
///
/// pin.set_duty(128);
/// pin.enable();
/// ```
///
/// Here is an overview of pins and which timer they work with:
///
/// | Pin | Conversion Method | Alternate Conversion Method |
/// | --- | --- | --- |
/// `pins.d2` | `.into_pwm(&mut timer3)` | |
/// `pins.d3` | `.into_pwm(&mut timer3)` | |
/// `pins.d4` | `.into_pwm(&mut timer0)` | |
/// `pins.d5` | `.into_pwm(&mut timer3)` | |
/// `pins.d6` | `.into_pwm(&mut timer4)` | |
/// `pins.d7` | `.into_pwm(&mut timer4)` | |
/// `pins.d8` | `.into_pwm(&mut timer4)` | |
/// `pins.d9` | `.into_pwm(&mut timer2)` | |
/// `pins.d10` | `.into_pwm(&mut timer2)` | |
/// `pins.d11` | `.into_pwm(&mut timer1)` | |
/// `pins.d12` | `.into_pwm(&mut timer1)` | |
/// `pins.d13` | `.into_pwm(&mut timer0)` | `.into_pwm1(&mut timer1)` |
///
/// [ex-pwm]: https://github.com/sepotvin/avr-hal/blob/master/boards/arduino-mega2560/examples/mega2560-pwm.rs
pub mod pwm {
    pub use crate::hal::pwm::*;
}
