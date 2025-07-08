use crate::{Co5300, param_command};
use embedded_graphics_core::{pixelcolor::{Gray8, Rgb565, Rgb666, Rgb888}, prelude::*};
use embedded_hal::digital::OutputPin;
use embedded_hal_async::{delay::DelayNs, digital::Wait, spi::SpiBus};
use crate::{consts::*, error::Error};
use embassy_time::Timer;

macro_rules! gray8_arr {
    ($color:expr) => {
        [$color.luma()]
    };
}

macro_rules! rgb565_arr {
    ($color:expr) => {
        [$color.r() << 3 | $color.g() >> 3, $color.g() << 5 | $color.b()]
    };
}

macro_rules! rgb666_arr {
    ($color:expr) => {
        [color.r() << 2, color.g() << 2, color.b() << 2]
    };
}

macro_rules! rgb888_arr {
    ($color:expr) => {
        [color.r(), color.g(), color.b()]
    };
}

impl<SPI, TE, RST> OriginDimensions for Co5300<SPI, TE, RST> {
    fn size(&self) -> Size {
        Size::new(466, 466)
    }
}

impl<SPI, TE, RST, SE, PE> Co5300<SPI, TE, RST> 
    where
    SPI: SpiBus<Error = SE>, 
    RST: OutputPin<Error = PE>,
    TE: Wait,
{
    type Color = Rgb888;
    type Error = Error<SE, PE>;
    

}
