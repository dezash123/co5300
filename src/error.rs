#[derive(Debug, Copy, Clone)]
pub enum Error<SE, PE> {
    SpiError(SE),
    PinError(PE),
}
