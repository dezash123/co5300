// use anyhow::Result;
use embedded_graphics_core::{geometry::{OriginDimensions, Point, Size}, pixelcolor::{Rgb565, Rgb888}};
use embedded_graphics_framebuf::FrameBuf;
use embedded_hal::digital::OutputPin;
use embedded_hal_async::{delay::DelayNs, digital::Wait, spi::SpiBus};
use heapless::Vec;
use embedded_graphics_core::pixelcolor::RgbColor;
use crate::{consts::*, error::Co5300Error};
use embassy_time::Timer;

pub trait SupportedColor {}
impl SupportedColor for Rgb888 {}
impl SupportedColor for Rgb565 {}

pub const X_OFFS: u16 = 6;
pub const Y_OFFS: u16 = 0;

pub struct Co5300<SPI, TE, RST, RGB> {
    spi: SPI,
    sync: TE,
    reset: RST,
    colormode: RGB,
    brightness: u8,
    asleep: bool,
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

impl<SPI, TE, RST, RGB, SE, PE> Co5300<SPI, TE, RST, RGB> 
    where
    SPI: SpiBus<Error = SE>, 
    RST: OutputPin<Error = PE>,
    TE: Wait,
    RGB: SupportedColor,
{
    pub type Error = Co5300Error<SE, PE>;
    pub async fn new(spi: SPI, sync: TE, reset: RST, colormode: RGB) -> Result<Self, Self::Error> {
        Self { spi, sync, reset, colormode, brightness: 0, asleep: false }.init().await
    }

    async fn init(mut self) -> Result<Self, Self::Error> {
        self.wake().await?;
        self.set_4wire().await?;

        self.send_param_command(SET_CMD_PAGE, [0]).await?;

        // self.send_param_command(WC_TEARON, [0x00]).await?;
        
        self.send_param_command(W_SPIMODECTL, [1 << 7]).await?;

        // self.send_param_command(W_MADCTL, MADCTL_COLOR_ORDER).await?; // RGB/BGR

        // self.send_param_command(W_PIXFMT, [0x55]).await?; // Interface Pixel Format 16bit/pixel
        // self.send_param_command(W_PIXFMT, [0x66]).await?; // Interface Pixel Format 18bit/pixel
        self.send_param_command(W_PIXFMT, [0x77]).await?; // Interface Pixel Format 24bit/pixel

        self.send_param_command(W_WCTRLD1, [1 << 5]).await?; // en/disable brightness control
        self.send_param_command(W_WDBRIGHTNESSVALHBM, [0xFF]).await?;

        self.send_param_command(W_CASET, [0x00, 0x06, 0x01, 0xD7]).await?; // 6 to 471 incl (466 px)
        self.send_param_command(W_PASET, [0x00, 0x00, 0x01, 0xD1]).await?; // 0 to 465 incl (466 px)

        self.send_command(C_DISPON).await?;

        self.send_param_command(W_WCE, [Contrast::ContrastOff as u8]).await?;

        Ok(self)
    }

    pub async fn wake(&mut self) -> Result<(), Co5300Error<SE, PE>> {
        self.reset().await?;
        self.send_command(C_SLPOUT).await?;
        Timer::after_millis(RST_TIME_MS).await;
        self.asleep = false;
        Ok(())
    }
    
    pub async fn sleep(&mut self) -> Result<(), Self::Error> {
        self.send_command(C_SLPIN).await?;
        Timer::after_millis(SLPIN_TO_RST_MS).await;
        self.reset.set_low().map_err(Co5300Error::PinError)?;
        self.asleep = true;
        Ok(())
    }
    
    async fn set_1wire(&mut self) -> Result<(), Self::Error> {
        self.spi.write(&[SET_SINGLE_SPI; 4]).await.map_err(Co5300Error::SpiError)?;
        Ok(())
    }
    
    async fn set_4wire(&mut self) -> Result<(), Self::Error> {
        self.set_1wire().await?;
        self.spi.write(&[SET_QUAD_SPI]).await.map_err(Co5300Error::SpiError)?;
        Ok(())
    }

    pub async fn all_pixels_on(&mut self) -> Result<(), Self::Error> {
        self.send_command(C_ALLPON).await?;
        Ok(())
    }

    pub async fn all_pixels_off(&mut self) -> Result<(), Self::Error> {
        self.send_command(C_ALLPOFF).await?;
        Ok(())
    }

    pub async fn set_brightness(&mut self, brightness: u8) -> Result<(), Self::Error> {
        self.send_param_command(W_WDBRIGHTNESSVALNOR, [brightness]).await?;
        Ok(())
    }

    pub async fn reset(&mut self) -> Result<(), Self::Error> {
        self.reset.set_low().map_err(Co5300Error::PinError)?;
        Timer::after_micros(RST_DOWN_US).await;
        self.reset.set_high().map_err(Co5300Error::PinError)?;
        Timer::after_millis(RST_TIME_MS).await;
        Ok(())
    }
    
    const fn pixel_setup(pixel: Point) -> ([u8; 2], [u8; 2]) {
        let x: [u8; 2] = (pixel.x as u16 + X_OFFS).to_be_bytes();
        let y: [u8; 2] = (pixel.y as u16 + Y_OFFS).to_be_bytes();
        (x, y)
    }
    async fn set_pixel_location(&mut self, pixel: Point) -> Result<(), Self::Error> {
        let pixels_out = Self::pixel_setup(pixel);
        self.send_param_command(W_CASET, pixels_out.0).await?;
        self.send_param_command(W_PASET, pixels_out.1).await
    }
    // async fn first_color_write(&mut self, color: Rgb888) -> Result<()> {
    //     self.send_param_command(RAMWR_START, [color.r(), color.g(), color.b()]).await
    // }

    #[inline]
    async fn send_command(&mut self, command: u8) -> Result<(), Self::Error> {
        self.spi.write(&[0x02u8.to_be(), 0x00, command.to_be(), 0x00]).await.map_err(Co5300Error::SpiError)?;
        Ok(())
    }
    #[inline]
    async fn send_param_command<const N: usize>(&mut self, command: u8, parameters: [u8; N]) -> Result<(), Self::Error>
    where
        [u8; N + 4]:,
    {
        let mut data: Vec<u8, { N + 4 }> = Vec::from_slice(&[0x02u8.to_be(), 0x00, command.to_be(), 0x00]).unwrap();
        data.extend_from_slice(&parameters).unwrap();
        self.spi.write(&data.into_array::<{N + 4}>().unwrap()).await.map_err(Co5300Error::SpiError)?;
        Ok(())
    }
}

impl<SPI: SpiBus, TE: Wait, RST: OutputPin> Co5300<SPI, TE, RST, Rgb888> {
    
}

impl<SPI, TE, RST, RGB> OriginDimensions for Co5300<SPI, TE, RST, RGB> {
    fn size(&self) -> Size {
        Size::new(466, 466)
    }
}



