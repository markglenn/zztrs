use bracket_lib::prelude::RGB;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub foreground: Option<RGB>,
    pub background: Option<RGB>,
    pub is_blinking: bool,
}

impl Color {
    pub const TRANSPARENT: Color = Color {
        foreground: None,
        background: None,
        is_blinking: false,
    };

    pub const BLACK: RGB = RGB {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };

    pub const BLUE: RGB = RGB {
        r: 0.0,
        g: 0.0,
        b: 0.5,
    };

    pub const GREEN: RGB = RGB {
        r: 0.0,
        g: 0.7,
        b: 0.0,
    };

    pub const CYAN: RGB = RGB {
        r: 0.0,
        g: 0.5,
        b: 0.5,
    };

    pub const RED: RGB = RGB {
        r: 0.5,
        g: 0.0,
        b: 0.0,
    };

    pub const MAGENTA: RGB = RGB {
        r: 0.5,
        g: 0.0,
        b: 0.5,
    };

    pub const BROWN: RGB = RGB {
        r: 0.5,
        g: 0.5,
        b: 0.0,
    };

    pub const GRAY: RGB = RGB {
        r: 0.75,
        g: 0.75,
        b: 0.75,
    };

    pub const CHARCOAL: RGB = RGB {
        r: 0.5,
        g: 0.5,
        b: 0.5,
    };

    pub const BRIGHT_BLUE: RGB = RGB {
        r: 0.0,
        g: 0.0,
        b: 1.0,
    };

    pub const BRIGHT_GREEN: RGB = RGB {
        r: 0.0,
        g: 1.0,
        b: 0.0,
    };

    pub const BRIGHT_CYAN: RGB = RGB {
        r: 0.0,
        g: 1.0,
        b: 1.0,
    };

    pub const BRIGHT_RED: RGB = RGB {
        r: 1.0,
        g: 0.0,
        b: 0.0,
    };

    pub const PINK: RGB = RGB {
        r: 1.0,
        g: 0.0,
        b: 1.0,
    };

    pub const YELLOW: RGB = RGB {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };

    pub const WHITE: RGB = RGB {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };

    pub const PALETTE: [RGB; 16] = [
        Self::BLACK,
        Self::BLUE,
        Self::GREEN,
        Self::CYAN,
        Self::RED,
        Self::MAGENTA,
        Self::BROWN,
        Self::GRAY,
        Self::CHARCOAL,
        Self::BRIGHT_BLUE,
        Self::BRIGHT_GREEN,
        Self::BRIGHT_CYAN,
        Self::BRIGHT_RED,
        Self::PINK,
        Self::YELLOW,
        Self::WHITE,
    ];
}

impl Color {
    pub const fn new(base_color: u8) -> Self {
        let color = base_color as usize;
        match base_color {
            0xFF => Color::TRANSPARENT,
            0xFE => Color {
                foreground: Some(Self::PALETTE[0xE]),
                background: None,
                is_blinking: false,
            },
            _ => Color {
                foreground: Some(Self::PALETTE[color & 0xF]),
                background: Some(Self::PALETTE[(color >> 4) & 0x7]),
                is_blinking: color & 0x80 != 0,
            },
        }
    }
}
