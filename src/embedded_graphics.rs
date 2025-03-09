use crate::{co5300::Co5300, param_command};
use embedded_graphics_core::{pixelcolor::Rgb888, prelude::*};
use embedded_hal::digital::OutputPin;
use embedded_hal_async::{delay::DelayNs, digital::Wait, spi::SpiBus};
use crate::{consts::*, error::Error};
use embassy_time::Timer;

impl<SPI, TE, RST, SE, PE> Co5300<SPI, TE, RST> 
    where
    SPI: SpiBus<Error = SE>, 
    RST: OutputPin<Error = PE>,
    TE: Wait,
{
    async fn write_rgb888(&mut self, color: Rgb888) -> Result<(), Error<SE, PE>> {
        self.qspi_write(param_command!(W_RAMWR, [color.r(), color.g(), color.b()])).await
    }
}

impl<SPI, TE, RST> OriginDimensions for Co5300<SPI, TE, RST> {
    fn size(&self) -> Size {
        Size::new(466, 466)
    }
}
