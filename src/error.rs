use core::{error::Error, fmt::Debug};

use embedded_hal::digital::OutputPin;
use embedded_hal_async::spi::SpiDevice;

#[derive(Debug)]
pub enum Co5300Error<SE = (), PE = ()>
{
    SpiError(SE),
    PinError(PE),
}

// impl<SE, PE> From<SE> for Co5300Error<SE, PE> {
//     fn from(value: SE) -> Self {
//         Self::SpiError(value)
//     }
// }
// 
// impl<SE, PE> From<PE> for Co5300Error<SE, PE> {
//     fn from(value: PE) -> Self {
//         Self::PinError(value)
//     }
// }
