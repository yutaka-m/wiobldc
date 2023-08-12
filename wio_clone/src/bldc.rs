
//use atsamd_hal::gpio::v2::PushPull;
use atsamd_hal::gpio::{Floating, Output, Pd11, Pd9, Pb15, Pd12, Pd10, Pd8, Pc23, Pc22, Pa18, Pa10, Pb24, Port};
//use atsamd_hal::gpio::v1::Input;
use atsamd_hal::gpio::PushPull;
use atsamd_hal::gpio::Input;
use atsamd_hal::hal::digital::v2::StatefulOutputPin;
use atsamd_hal::prelude::_atsamd_hal_embedded_hal_digital_v2_OutputPin;


/*
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
*/
pub struct Bldc {
    pub hin1: Pd12<Input<Floating>>,
    pub hin2: Pd10<Input<Floating>>,
    pub hin3: Pd8<Input<Floating>>,
    pub lin1: Pd11<Input<Floating>>,
    pub lin2: Pd9<Input<Floating>>,
    pub lin3: Pb15<Input<Floating>>,
    pub mux_1: Pc23<Input<Floating>>,
    pub mux_2: Pc22<Input<Floating>>,
    pub mux_3: Pa18<Input<Floating>>,
    pub com_all: Pb24<Input<Floating>>,
}

impl Bldc {
    pub fn init(self, port: &mut Port) -> BldcController {
        BldcController {
            step: 0,
            steptable: [
                [1, 0, 0, 0, 0, 1],
                [0, 0, 0, 1, 1, 0],
                [0, 1, 1, 0, 0, 0],
                [0, 0, 1, 1, 0, 0],
                [1, 1, 0, 0, 0, 0],
                [0, 0, 0, 0, 1, 1],
            ],
            hin1: self.hin1.into_push_pull_output(port),
            hin2: self.hin2.into_push_pull_output(port),
            hin3: self.hin3.into_push_pull_output(port),
            lin1: self.lin1.into_push_pull_output(port),
            lin2: self.lin2.into_push_pull_output(port),
            lin3: self.lin3.into_push_pull_output(port),
            mux_1: self.mux_1.into_floating_input(port),
            mux_2: self.mux_2.into_floating_input(port),
            mux_3: self.mux_3.into_floating_input(port),
            com_all: self.com_all.into_floating_input(port),
        }
    }
}

pub struct BldcController {
    step: usize,
    steptable: [[u8; 6]; 6],
    pub hin1: Pd12<Output<PushPull>>,
    pub hin2: Pd10<Output<PushPull>>,
    pub hin3: Pd8<Output<PushPull>>,
    pub lin1: Pd11<Output<PushPull>>,
    pub lin2: Pd9<Output<PushPull>>,
    pub lin3: Pb15<Output<PushPull>>,
    pub mux_1: Pc23<Input<Floating>>,
    pub mux_2: Pc22<Input<Floating>>,
    pub mux_3: Pa18<Input<Floating>>,
    pub com_all: Pb24<Input<Floating>>,
}

impl BldcController {
    pub fn next(&mut self) {
        if self.steptable[0][self.step] == 1 {
            self.hin1.set_high();
        } else {
            self.hin1.set_low();
        }

        if self.steptable[1][self.step] == 1 {
            self.hin2.set_high();
        } else {
            self.hin2.set_low();
        }

        if self.steptable[2][self.step] == 1 {
            self.hin3.set_high();
        } else {
            self.hin3.set_low();
        }

        if self.steptable[3][self.step] == 1 {
            self.lin1.set_high();
        } else {
            self.lin1.set_low();
        }

        if self.steptable[4][self.step] == 1 {
            self.lin2.set_high();
        } else {
            self.lin2.set_low();
        }

        if self.steptable[5][self.step] == 1 {
            self.lin3.set_high();
        } else {
            self.lin3.set_low();
        }

        self.step += 1;
        if self.step > 5 {
            self.step = 0;
        }
    }
}
