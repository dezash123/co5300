
// my code; switched to values from arduino driver

pub const SET_CMD_PAGE: u8 = 0xFE; // set CMD page (CMD Page Switch)
/*
pub const SET_SPI_MODE: u8 = 0xC4; //SPI mode select (SetSPIMode)
pub const COLMOD: u8 = 0x3A; // color mode select
pub const TEON: u8 = 0x35; // 0 = vsync, 1 = v+hsync
pub const WRCTRLD: u8 = 0x53; // en/disable brightness control
pub const WRDISBV: u8 = 0x51; // set brightness
pub const WRHBMDISBV: u8 = 0x51; // set brightness in HBM mode (?)
pub const CASET: u8 = 0x2A; // set column start address
pub const RASET: u8 = 0x2B; // set row start address
pub const SLPOUT: u8 = 0x11; // exit sleep mode
pub const DISPON: u8 = 0x29; //turn on display
pub const RAMWR_START: u8 = 0x2C;
pub const RAMWR_CONTINUOUS: u8 = 0x3C;
pub const ALLPON: u8 = 0x23;
pub const SPA_ID: u8 = 0x15;
pub const LPA_ID: u8 = 0x29;
*/

// instructions
pub const SET_SINGLE_SPI: u8 = 0xFF;
pub const SET_DUAL_SPI: u8 = 0x3B;
pub const SET_QUAD_SPI: u8 = 0x38;
pub const PIXEL_WRITE_4_WIRE_6_ADR: u8 = 0x12;


// stolen from arduino driver
pub const MAXWIDTH: u16 = 480;  ///< CO5300 max TFT width
pub const MAXHEIGHT: u16 = 480; ///< CO5300 max TFT width
/// all times are double minima
pub const RST_DOWN_US: u64 = 20; 
// pub const RST_TIME_SLPIN_MS: u64 = 10;
// pub const RST_TIME_SLPOUT_MS: u64 = 200; // min is 120
pub const RST_TIME_MS: u64 = 200; // ion trust u china
// pub const RST_TO_SLPOUT_MS: u64 = 30; // could be 10 but need to also wait 10 after vdd
pub const SLPIN_TO_RST_MS: u64 = 166;
///
pub const C_NOP: u8 = 0x00;          // nop
pub const C_SWRESET: u8 = 0x01;      // Software Reset
pub const R_RDID: u8 = 0x04;         // Read Display Identification Information ID/1/2/3
pub const R_RDNERRORSDSI: u8 = 0x05; // Read Number of Errors on DSI
pub const R_RDPOWERMODE: u8 = 0x0A;  // Read Display Power Mode
pub const R_RDMADCTL: u8 = 0x0B;     // Read Display MADCTL
pub const R_RDPIXFMT: u8 = 0x0C;     // Read Display Pixel Format
pub const R_RDIMGFMT: u8 = 0x0D;     // Read Display Image Mode
pub const R_RDSIGMODE: u8 = 0x0E;    // Read Display Signal Mode
pub const R_RDSELFDIAG: u8 = 0x0F;   // Read Display Self-Diagnostic Result
// 
pub const C_SLPIN: u8 = 0x10;  // Sleep In
pub const C_SLPOUT: u8 = 0x11; // Sleep Out
pub const C_PTLON: u8 = 0x12;  // Partial Display On
pub const C_NORON: u8 = 0x13;  // Normal Display mode on
//
pub const C_INVOFF: u8 = 0x20;  // Inversion Off
pub const C_INVON: u8 = 0x21;   // Inversion On
pub const C_ALLPOFF: u8 = 0x22; // All pixels off
pub const C_ALLPON: u8 = 0x23;  // All pixels on
pub const C_DISPOFF: u8 = 0x28; // Display off
pub const C_DISPON: u8 = 0x29;  // Display on
pub const W_CASET: u8 = 0x2A;   // Column Address Set
pub const W_PASET: u8 = 0x2B;   // Page Address Set
pub const W_RAMWR: u8 = 0x2C;   // Memory Write Start
//
pub const W_PTLAR: u8 = 0x30;   // Partial Area Row Set
pub const W_PTLAC: u8 = 0x31;   // Partial Area Column Set
pub const C_TEAROFF: u8 = 0x34; // Tearing effect off
pub const WC_TEARON: u8 = 0x35; // Tearing effect on
pub const W_MADCTL: u8 = 0x36;  // Memory data access control
pub const C_IDLEOFF: u8 = 0x38; // Idle Mode Off
pub const C_IDLEON: u8 = 0x39;  // Idle Mode On
pub const W_PIXFMT: u8 = 0x3A;  // Write Display Pixel Format
pub const W_WRMC: u8 = 0x3C;    // Memory Write Continue
//
pub const W_SETTSL: u8 = 0x44;             // Write Tearing Effect Scan Line
pub const R_GETSL: u8 = 0x45;              // Read Scan Line Number
pub const C_SPIROFF: u8 = 0x46;            // SPI read Off
pub const C_SPIRON: u8 = 0x47;             // SPI read On
pub const C_AODMOFF: u8 = 0x48;            // AOD Mode Off
pub const C_AODMON: u8 = 0x49;             // AOD Mode On
pub const W_WDBRIGHTNESSVALAOD: u8 = 0x4A; // Write Display Brightness Value in AOD Mode
pub const R_RDBRIGHTNESSVALAOD: u8 = 0x4B; // Read Display Brightness Value in AOD Mode
pub const W_DEEPSTMODE: u8 = 0x4F;         // Deep Standby Mode On
//
pub const W_WDBRIGHTNESSVALNOR: u8 = 0x51; // Write Display Brightness Value in Normal Mode
pub const R_RDBRIGHTNESSVALNOR: u8 = 0x52; // Read display brightness value in Normal Mode
pub const W_WCTRLD1: u8 = 0x53;            // Write CTRL Display1
pub const R_RCTRLD1: u8 = 0x54;            // Read CTRL Display1
pub const W_WCTRLD2: u8 = 0x55;            // Write CTRL Display2
pub const R_RCTRLD2: u8 = 0x56;            // Read CTRL Display2
pub const W_WCE: u8 = 0x58;                // Write CE
pub const R_RCE: u8 = 0x59;                // Read CE
//
pub const W_WDBRIGHTNESSVALHBM: u8 = 0x63; // Write Display Brightness Value in HBM Mode
pub const R_WDBRIGHTNESSVALHBM: u8 = 0x64; // Read Display Brightness Value in HBM Mode
pub const W_WHBMCTL: u8 = 0x66;            // Write HBM Control
//
pub const W_COLORSET0: u8 = 0x70;  // Color Set 0
pub const W_COLORSET1: u8 = 0x71;  // Color Set 1
pub const W_COLORSET2: u8 = 0x72;  // Color Set 2
pub const W_COLORSET3: u8 = 0x73;  // Color Set 3
pub const W_COLORSET4: u8 = 0x74;  // Color Set 4
pub const W_COLORSET5: u8 = 0x75;  // Color Set 5
pub const W_COLORSET6: u8 = 0x76;  // Color Set 6
pub const W_COLORSET7: u8 = 0x77;  // Color Set 7
pub const W_COLORSET8: u8 = 0x78;  // Color Set 8
pub const W_COLORSET9: u8 = 0x79;  // Color Set 9
pub const W_COLORSET10: u8 = 0x7A; // Color Set 10
pub const W_COLORSET11: u8 = 0x7B; // Color Set 11
pub const W_COLORSET12: u8 = 0x7C; // Color Set 12
pub const W_COLORSET13: u8 = 0x7D; // Color Set 13
pub const W_COLORSET14: u8 = 0x7E; // Color Set 14
pub const W_COLORSET15: u8 = 0x7F; // Color Set 15
//
pub const W_COLOROPTION: u8 = 0x80; // Color Option
//
pub const R_RDDBSTART: u8 = 0xA1;         // Read DDB start
pub const R_DDBCONTINUE: u8 = 0xA8;       // Read DDB Continue
pub const R_RFIRCHECKSUN: u8 = 0xAA;      // Read First Checksum
pub const R_RCONTINUECHECKSUN: u8 = 0xAF; // Read Continue Checksum
//
pub const W_SPIMODECTL: u8 = 0xC4; // SPI mode control
//
pub const R_RDID1: u8 = 0xDA; // Read ID1
pub const R_RDID2: u8 = 0xDB; // Read ID2
pub const R_RDID3: u8 = 0xDC; // Read ID3
//
pub const MADCTL_X_AXIS_FLIP: u8 = 0x02; // Flip Horizontal
pub const MADCTL_Y_AXIS_FLIP: u8 = 0x05; // Flip Vertical

pub const MADCTL_RGB: u8 = 0x00;                      // Red-Green-Blue pixel order
pub const MADCTL_BGR: u8 = 0x08;                      // Blue-Green-Red pixel order
pub const MADCTL_COLOR_ORDER: u8 = MADCTL_RGB; // RGB

#[repr(u8)]
pub enum Contrast {
    ContrastOff = 0,
    LowContrast,
    MediumContrast,
    HighContrast,
}

