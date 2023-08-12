use atsamd_hal::clock::GenericClockController;
use atsamd_hal::common::eic;
use atsamd_hal::common::eic::pin::*;
use atsamd_hal::gpio::*;

//pub extern crate embedded_hal as hal;
//pub use atsamd51p as target_device;
//use atsamd_hal::{define_pins, target_device};
use atsamd_hal::target_device::*;

use cortex_m::peripheral::NVIC;

/// pushbuttons and joystick
pub struct ButtonPins {
    /// BUTTONS
    pub button1: Pc26<Input<Floating>>,
    pub button2: Pc27<Input<Floating>>,
    pub button3: Pc28<Input<Floating>>,
    pub button4: Pc24<Input<Floating>>,
    pub button5: Pc25<Input<Floating>>,
}


impl ButtonPins {
    pub fn init(
        self,
        eic: EIC,
        clocks: &mut GenericClockController,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> ButtonController {
        let clk = clocks.gclk1();
        let mut eic = eic::init_with_ulp32k(mclk, clocks.eic(&clk).unwrap(), eic);
        eic.button_debounce_pins(&[
            self.button1.id(),
            self.button2.id(),
            self.button3.id(),
            self.button4.id(),
            self.button5.id(),
        ]);

        // Unfortunately, the pin assigned to B1 shares the same
        // ExtInt line as up on the joystick. As such, we don't
        // support B1.

        // let mut b1 = self.button1.into_ei(port);
        let mut b2 = self.button2.into_ei(port);
        let mut b3 = self.button3.into_ei(port);
        let mut b4 = self.button4.into_ei(port);
        let mut b5 = self.button5.into_ei(port);

        // b1.sense(&mut eic, Sense::BOTH);
        b2.sense(&mut eic, Sense::BOTH);
        b3.sense(&mut eic, Sense::BOTH);
        b4.sense(&mut eic, Sense::BOTH);
        b5.sense(&mut eic, Sense::BOTH);
        

        // b1.enable_interrupt(&mut eic);
        b2.enable_interrupt(&mut eic);
        b3.enable_interrupt(&mut eic);
        b4.enable_interrupt(&mut eic);
        b5.enable_interrupt(&mut eic);

        ButtonController {
            _eic: eic.finalize(),
            // b1,
            b2,
            b3,
            b4,
            b5,
        }
    }
}


#[derive(Debug, PartialEq, Eq)]
pub enum Button {
    TopLeft,
    TopMiddle,
    // TopRight,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ButtonEvent {
    pub button: Button,
    pub down: bool,
}

pub struct ButtonController {
    _eic: eic::EIC,
    // b1: ExtInt10<Pc26<PfA>>,
    b2: ExtInt11<Pc27<PfA>>,
    b3: ExtInt12<Pc28<PfA>>,
    b4: ExtInt8<Pc24<PfA>>,
    b5: ExtInt9<Pc25<PfA>>,

}

macro_rules! isr {
    ($Handler:ident, $($Event:expr, $Button:ident),+) => {
        pub fn $Handler(&mut self) -> Option<ButtonEvent> {
            $(
                {
                    let b = &mut self.$Button;
                    if b.is_interrupt() {
                        b.clear_interrupt();
                        return Some(ButtonEvent {
                            button: $Event,
                            down: !b.state(),
                        })
                    }
                }
            )+

            None
        }
    };
}

impl ButtonController {
    pub fn enable(&self, nvic: &mut NVIC) {
        unsafe {
            nvic.set_priority(interrupt::EIC_EXTINT_10, 1);
            NVIC::unmask(interrupt::EIC_EXTINT_10);
            nvic.set_priority(interrupt::EIC_EXTINT_11, 1);
            NVIC::unmask(interrupt::EIC_EXTINT_11);
            nvic.set_priority(interrupt::EIC_EXTINT_12, 1);
            NVIC::unmask(interrupt::EIC_EXTINT_12);
            nvic.set_priority(interrupt::EIC_EXTINT_8, 1);
            NVIC::unmask(interrupt::EIC_EXTINT_8);
            nvic.set_priority(interrupt::EIC_EXTINT_9, 1);
            NVIC::unmask(interrupt::EIC_EXTINT_9);
        }
    }
    
    // isr!(interrupt_extint10, Button::TopRight, b1);
    isr!(interrupt_extint11, Button::TopMiddle, b2);
    isr!(interrupt_extint12, Button::TopLeft, b3);
    isr!(interrupt_extint13, Button::TopMiddle, b4);
    isr!(interrupt_extint14, Button::TopLeft, b5);
}

#[macro_export]
macro_rules! button_interrupt {
    ($controller:ident, unsafe fn $func_name:ident ($cs:ident: $cstype:ty, $event:ident: ButtonEvent ) $code:block) => {
        unsafe fn $func_name($cs: $cstype, $event: ButtonEvent) {
            $code
        }

        macro_rules! _button_interrupt_handler {
            ($Interrupt:ident, $Handler:ident) => {
                #[interrupt]
                fn $Interrupt() {
                    disable_interrupts(|cs| unsafe {
                        $controller.as_mut().map(|ctrlr| {
                            if let Some(event) = ctrlr.$Handler() {
                                $func_name(cs, event);
                            }
                        });s
                    });
                }
            };
        }
        
        _button_interrupt_handler!(EIC_EXTINT_10, interrupt_extint10);
        _button_interrupt_handler!(EIC_EXTINT_11, interrupt_extint11);
        _button_interrupt_handler!(EIC_EXTINT_12, interrupt_extint12);
        _button_interrupt_handler!(EIC_EXTINT_8, interrupt_extint8);
        _button_interrupt_handler!(EIC_EXTINT_9, interrupt_extint9);
    };
}
