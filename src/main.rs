#![no_std]
#![no_main]

use panic_halt as _;
use wio_clone as wio;

use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;
use wio::{entry, Pins, Sets};


#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut delay = Delay::new(core.SYST, &mut clocks);
    let mut sets: Sets = Pins::new(peripherals.PORT).split();
    let mut user_led = sets.user_led.into_push_pull_output(&mut sets.port);
    let mut bldc = sets.bldc.init(&mut sets.port);
    //let mut hin1 = sets.bldc.hin1.into_push_pull_output(&mut sets.port);

    loop {
        bldc.next();
        user_led.toggle();
        delay.delay_ms(10u16);
    }
}
