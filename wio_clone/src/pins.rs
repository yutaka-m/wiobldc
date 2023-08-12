use atsamd_hal::gpio::{self, *};
use atsamd_hal::{define_pins, target_device};

use super::buttons::ButtonPins;
use super::display::Display;
use super::sensors::{Accelerometer, LightSensor};
use super::serial::{UART, USB};
use super::sound::{Buzzer, Microphone};
use super::storage::{QSPIFlash, SDCard};
use super::wifi::WifiPins;
use super::bldc::Bldc;



define_pins!(
    /// Map the desired pin names to their physical pins
    struct Pins,
    target_device: target_device,

    // NOTE:
    // The following pin names were adapted from the labels in the schematic.
    // They will likely evolve over time.
    // They're not in any particular order.

    /// USER_LED
    pin user_led = a15,

    /// UART
    pin txd = b26,
    pin rxd = b27,

    /// USB
    pin usb_dm = a24,
    pin usb_dp = a25,
    pin usb_host_en = a27,

    /// YUTAKA PINS
    /// BUTTONS
    pin button1 = c26,
    pin button2 = c27,
    pin button3 = c28,
    pin button4 = c24,
    pin button5 = c25,

    /// BLDC1
    pin lin1 = d11,
    pin lin2 = d9,
    pin lin3 = b15,
    pin hin1 = d12,
    pin hin2 = d10,
    pin hin3 = d8,
    pin mux_1 = c23,
    pin mux_2 = c22,
    pin mux_3 = a18,
    pin com_all = b24,
    
    /// BLDC2
    pin lin1_b = a4,
    pin lin2_b = b8,
    pin lin3_b = b6,
    pin hin1_b = a5,
    pin hin2_b = b9,
    pin hin3_b = b7,
    pin mux_1_b = c4,
    pin mux_2_b = a7,
    pin mux_3_b = a6,
    pin com_all_b = c1,
    
    /// BLDC3
    pin lin1_c = b5,
    pin lin2_c = a2,
    pin lin3_c = c2,
    pin hin1_c = d0,
    pin hin2_c = b4,
    pin hin3_c = c3,
    pin mux_1_c = b29,
    pin mux_2_c = b28,
    pin mux_3_c = b25,
    pin com_all_c = b17,
    
    /// GPCLK
    pin gpclk0 = b14,
    pin gpclk1 = b12,
    pin gpclk2 = b13,

    /// SWD
    pin swdclk = a30,
    pin swdio = a31,

    /// XIN/XOUT
    pin xin = b22,
    pin xout = b23,

    /// MISCELLANEOUS
    pin swo = b30,
    pin ir_ctl = b31,
    pin output_ctr_5v = c14,
    pin output_ctr_3v3 = c15,
);


pub struct Sets {
    /// GPIO port
    pub port: Port,

    /// UART (external pinout) pins
    pub uart: UART,

    /// USB pins
    pub usb: USB,

    /// LED pin
    pub user_led: Pa15<Input<Floating>>,

    pub buttons: ButtonPins,

    pub bldc: Bldc,
}

impl Pins {
    /// Split the device pins into subsets
    pub fn split(self) -> Sets {
        let port = self.port;

        let uart = UART {
            rx: self.rxd,
            tx: self.txd,
        };
        
        let usb = USB {
            dm: self.usb_dm,
            dp: self.usb_dp,
        };

        let user_led  = self.user_led;

        let buttons = ButtonPins {
            button1: self.button1,
            button2: self.button2,
            button3: self.button3,
            button4: self.button4,
            button5: self.button5,
        };

        let bldc = Bldc {
            lin1: self.lin1,
            lin2: self.lin2,
            lin3: self.lin3,
            hin1: self.hin1,
            hin2: self.hin2,
            hin3: self.hin3,
            mux_1: self.mux_1,
            mux_2: self.mux_2,
            mux_3: self.mux_3,
            com_all: self.com_all,
        };

        Sets {
            port,
            uart,
            usb,
            user_led,
            buttons,
            bldc,
        }
    }
}
