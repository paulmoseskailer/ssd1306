//! Draw a 1 bit per pixel black and white image.
//!
//! Image was created with ImageMagick:
//!
//! ```bash
//! convert rust.png -depth 1 gray:rust.raw
//! ```

#![no_std]

extern crate cortex_m;
extern crate embedded_graphics;
extern crate embedded_hal as hal;
extern crate ssd1306;
extern crate stm32f103xx_hal as blue_pill;

use blue_pill::i2c::{DutyCycle, I2c, Mode};
use blue_pill::prelude::*;
use embedded_graphics::Drawing;
use ssd1306::Builder;
use embedded_graphics::image::Image1BPP;

fn main() {
    let dp = blue_pill::stm32f103xx::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let i2c = I2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000,
            duty_cycle: DutyCycle::Ratio1to1,
        },
        clocks,
        &mut rcc.apb1,
    );

    let mut disp = Builder::new().connect_i2c(i2c);
    disp.init();
    disp.flush();

    let im = Image1BPP {
        width: 64,
        height: 64,
        imagedata: include_bytes!("./rust.raw"),
    };

    disp.draw(im.into_iter());

    disp.flush();
}
