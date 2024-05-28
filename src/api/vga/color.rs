/// The standard color palette in VGA text mode
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    DarkBlack     = 0x0,
    DarkBlue      = 0x1,
    DarkGreen     = 0x2,
    DarkCyan      = 0x3,
    DarkRed       = 0x4,
    DarkMagenta   = 0x5,
    DarkYellow    = 0x6,
    DarkWhite     = 0x7,

    BrightBlack   = 0x8,
    BrightBlue    = 0x9,
    BrightGreen   = 0xA,
    BrightCyan    = 0xB,
    BrightRed     = 0xC,
    BrightMagenta = 0xD,
    BrightYellow  = 0xE,
    BrightWhite   = 0xF,
}

const COLORS: [Color; 16] = [
    Color::DarkBlack,
    Color::DarkBlue,
    Color::DarkGreen,
    Color::DarkCyan,
    Color::DarkRed,
    Color::DarkMagenta,
    Color::DarkYellow,
    Color::DarkWhite,
    Color::BrightBlack,
    Color::BrightBlue,
    Color::BrightGreen,
    Color::BrightCyan,
    Color::BrightRed,
    Color::BrightMagenta,
    Color::BrightYellow,
    Color::BrightWhite,
];

pub fn colors() -> [Color; 16] {
    COLORS
}

pub fn from_index(index: usize) -> Color {
    COLORS[index]
}

pub fn from_ansi(code: u8) -> Color {
    match code {
        30 => Color::DarkBlack,
        31 => Color::DarkRed,
        32 => Color::DarkGreen,
        33 => Color::DarkYellow,
        34 => Color::DarkBlue,
        35 => Color::DarkMagenta,
        36 => Color::DarkCyan,
        37 => Color::DarkWhite,
        90 => Color::BrightBlack,
        91 => Color::BrightRed,
        92 => Color::BrightGreen,
        93 => Color::BrightYellow,
        94 => Color::BrightBlue,
        95 => Color::BrightMagenta,
        96 => Color::BrightCyan,
        97 => Color::BrightWhite,
        _  => Color::DarkBlack, // TODO: Error
    }
}

impl Color {
    pub fn to_vga_reg(&self) -> u8 {
        match self {
            Color::DarkBlack     => 0x00,
            Color::DarkBlue      => 0x01,
            Color::DarkGreen     => 0x02,
            Color::DarkCyan      => 0x03,
            Color::DarkRed       => 0x04,
            Color::DarkMagenta   => 0x05,
            Color::DarkYellow    => 0x14,
            Color::DarkWhite     => 0x07,
            Color::BrightBlack   => 0x38,
            Color::BrightBlue    => 0x39,
            Color::BrightGreen   => 0x3A,
            Color::BrightCyan    => 0x3B,
            Color::BrightRed     => 0x3C,
            Color::BrightMagenta => 0x3D,
            Color::BrightYellow  => 0x3E,
            Color::BrightWhite   => 0x3F,
        }
    }
}
