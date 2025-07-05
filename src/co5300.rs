use embedded_graphics_core::{geometry::{OriginDimensions, Point, Size}, pixelcolor::{Rgb565, Rgb888}};
use embedded_hal::digital::OutputPin;
use embedded_hal_async::{delay::DelayNs, digital::Wait, spi::SpiBus};
use crate::{consts::*, error::Error};
use embassy_time::Timer;

pub const X_OFFS: u16 = 6;
pub const Y_OFFS: u16 = 0;

pub struct Co5300<SPI, TE, RST> {
    spi: SPI,
    sync: TE,
    reset: RST,
    asleep: bool,
    // framebuf: [u8; 512],
}

#[macro_export]
macro_rules! param_command {
    ($cmd:expr, [$($param:expr),* $(,)?]) => {
        &[0x02u8.to_be(), 0x00, $cmd, 0x00 $(, $param)*]
    };
}

const fn convert_4wire_to_1wire(four_wire: u8) -> [u8; 4] {
        let four_wire_u32 = four_wire as u32;
        let mut output: u32 = 0;
        let mut bit: u8 = 8;
        while bit != 0 {
            bit -= 1;
            output <<= 3;
            output |= four_wire_u32 & (1 << bit);
        }
        output.to_be_bytes()
}

impl<SPI, TE, RST, SE, PE> Co5300<SPI, TE, RST> 
    where
    SPI: SpiBus<Error = SE>, 
    RST: OutputPin<Error = PE>,
    TE: Wait,
{
    pub async fn new(spi: SPI, sync: TE, reset: RST) -> Result<Self, Error<SE, PE>> {
        Self { spi, sync, reset, asleep: false }.init().await
    }

    async fn init(mut self) -> Result<Self, Error<SE, PE>> {
        self.wake().await?;
        self.set_4wire().await?;

        self.qspi_write(param_command!(SET_CMD_PAGE, [0])).await?;

        // self.send_param_command(WC_TEARON, [0x00]).await?;
        
        self.qspi_write(param_command!(W_SPIMODECTL, [1 << 7])).await?;
        
        // self.send_param_command(W_MADCTL, MADCTL_COLOR_ORDER).await?; // RGB/BGR

        // self.send_param_command(W_PIXFMT, [0x55]).await?; // Interface Pixel Format 16bit/pixel (rgb565)
        // self.send_param_command(W_PIXFMT, [0x66]).await?; // Interface Pixel Format 18bit/pixel (rgb666)
        self.qspi_write(param_command!(W_PIXFMT, [0x77])).await?; // Interface Pixel Format 24bit/pixel (rgb888)
        
        self.qspi_write(param_command!(W_WCTRLD1, [1 << 5])).await?; // en/disable brightness control
        self.qspi_write(param_command!(W_WDBRIGHTNESSVALHBM, [0xFF])).await?;

        self.qspi_write(param_command!(W_CASET, [0x00, 0x06, 0x01, 0xD7])).await?;
        self.qspi_write(param_command!(W_PASET, [0x00, 0x00, 0x01, 0xD1])).await?;

        self.send_command(C_DISPON).await?;

        self.qspi_write(param_command!(W_WCE, [Contrast::ContrastOff as u8])).await?;

        Ok(self)
    }

    pub async fn wake(&mut self) -> Result<(), Error<SE, PE>> {
        self.reset().await?;
        self.send_command(C_SLPOUT).await?;
        Timer::after_millis(RST_TIME_MS).await;
        self.asleep = false;
        Ok(())
    }
    
    pub async fn sleep(&mut self) -> Result<(), Error<SE, PE>> {
        self.send_command(C_SLPIN).await?;
        Timer::after_millis(SLPIN_TO_RST_MS).await;
        self.reset.set_low().map_err(Error::PinError)?;
        self.asleep = true;
        Ok(())
    }
    
    async fn set_1wire(&mut self) -> Result<(), Error<SE, PE>> {
        self.qspi_write(&[SET_SINGLE_SPI; 4]).await 
    }
    
    async fn set_4wire(&mut self) -> Result<(), Error<SE, PE>> {
        self.set_1wire().await?;
        self.qspi_write(&[SET_QUAD_SPI]).await 
    }

    pub async fn all_pixels_on(&mut self) -> Result<(), Error<SE, PE>> {
        self.send_command(C_ALLPON).await
    }

    pub async fn all_pixels_off(&mut self) -> Result<(), Error<SE, PE>> {
        self.send_command(C_ALLPOFF).await
    }

    pub async fn set_brightness(&mut self, brightness: u8) -> Result<(), Error<SE, PE>> {
        self.qspi_write(param_command!(W_WDBRIGHTNESSVALNOR, [brightness])).await
    }

    pub async fn reset(&mut self) -> Result<(), Error<SE, PE>> {
        self.reset.set_low().map_err(Error::PinError)?;
        Timer::after_micros(RST_DOWN_US).await;
        self.reset.set_high().map_err(Error::PinError)?;
        Timer::after_millis(RST_TIME_MS).await;
        Ok(())
    }
    
    const fn pixel_setup(pixel: Point) -> ([u8; 2], [u8; 2]) {
        ((pixel.x as u16 + X_OFFS).to_be_bytes(),
        (pixel.y as u16 + Y_OFFS).to_be_bytes())
    }

    pub async fn set_pixel_location(&mut self, pixel: Point) -> Result<(), Error<SE, PE>> {
        let (x, y) = Self::pixel_setup(pixel);
        self.qspi_write(param_command!(W_CASET, [x[0], x[1]])).await?;
        self.qspi_write(param_command!(W_PASET, [y[0], y[1]])).await
    }

    #[inline]
    async fn send_command(&mut self, command: u8) -> Result<(), Error<SE, PE>> {
        self.qspi_write(&[0x02u8.to_be(), 0x00, command.to_be(), 0x00]).await
    }

    #[inline]
    async fn qspi_write(&mut self, data: &[u8]) -> Result<(), Error<SE, PE>> {
        self.spi.write(data).await.map_err(Error::SpiError)
    }
}
