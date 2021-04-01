use bracket_lib::prelude::RGB;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub foreground: RGB,
    pub background: RGB,
    pub is_blinking: bool,
}

pub const BLACK: RGB = RGB {
    r: 0.0,
    g: 0.0,
    b: 0.0,
};

pub const BLUE: RGB = RGB {
    r: 0.0,
    g: 0.0,
    b: 0.667,
};

pub const GREEN: RGB = RGB {
    r: 0.0,
    g: 0.7,
    b: 0.0,
};

pub const CYAN: RGB = RGB {
    r: 0.0,
    g: 0.667,
    b: 0.667,
};

pub const RED: RGB = RGB {
    r: 0.667,
    g: 0.0,
    b: 0.0,
};

pub const MAGENTA: RGB = RGB {
    r: 0.667,
    g: 0.0,
    b: 0.667,
};

pub const BROWN: RGB = RGB {
    r: 0.667,
    g: 0.333,
    b: 0.0,
};

pub const GRAY: RGB = RGB {
    r: 0.667,
    g: 0.667,
    b: 0.667,
};

pub const CHARCOAL: RGB = RGB {
    r: 0.333,
    g: 0.333,
    b: 0.333,
};

pub const BRIGHT_BLUE: RGB = RGB {
    r: 0.333,
    g: 0.333,
    b: 1.0,
};

pub const BRIGHT_GREEN: RGB = RGB {
    r: 0.333,
    g: 1.0,
    b: 0.333,
};

pub const BRIGHT_CYAN: RGB = RGB {
    r: 0.333,
    g: 1.0,
    b: 1.0,
};

pub const BRIGHT_RED: RGB = RGB {
    r: 1.0,
    g: 0.333,
    b: 0.333,
};

pub const PINK: RGB = RGB {
    r: 1.0,
    g: 0.333,
    b: 1.0,
};

pub const YELLOW: RGB = RGB {
    r: 1.0,
    g: 1.0,
    b: 0.333,
};

pub const WHITE: RGB = RGB {
    r: 1.0,
    g: 1.0,
    b: 1.0,
};

pub const PALETTE: [RGB; 16] = [
    BLACK,
    BLUE,
    GREEN,
    CYAN,
    RED,
    MAGENTA,
    BROWN,
    GRAY,
    CHARCOAL,
    BRIGHT_BLUE,
    BRIGHT_GREEN,
    BRIGHT_CYAN,
    BRIGHT_RED,
    PINK,
    YELLOW,
    WHITE,
];

impl Color {
    pub const fn new(color: u8) -> Self {
        let color = color as usize;
        Color {
            foreground: PALETTE[color & 0xF],
            background: PALETTE[(color >> 4) & 0x7],
            is_blinking: color & 0x80 != 0,
        }
    }
}

impl From<u8> for Color {
    fn from(c: u8) -> Self {
        Color::new(c)
    }
}
