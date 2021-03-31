use bracket_lib::prelude::{BTerm, Console};

use crate::color::Color;
use crate::elements::Glyph;

pub fn render_string(ctx: &mut BTerm, color: u8, x: usize, y: usize, text: &[u8]) {
    let col = Color::new(color);

    for (i, glyph) in text.iter().enumerate() {
        ctx.set(
            (x + i) as i32,
            y as i32,
            col.foreground,
            col.background,
            *glyph,
        );
    }
}

pub fn render_glyph(ctx: &mut BTerm, color: u8, x: usize, y: usize, glyph: Glyph) {
    let col = Color::new(color);

    ctx.set(x as i32, y as i32, col.foreground, col.background, glyph);
}
