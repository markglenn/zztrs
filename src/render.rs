use bracket_lib::prelude::{BTerm, Console};

use crate::components::{Color, Glyph};

pub fn render_ascii_string(ctx: &mut BTerm, color: u8, x: i32, y: i32, text: &[u8]) {
    let Color {
        foreground: fg,
        background: bg,
        ..
    } = Color::from(color);

    for (i, glyph) in text.iter().enumerate() {
        ctx.set(x + i as i32, y as i32, fg, bg, *glyph);
    }
}

pub fn render_string(ctx: &mut BTerm, color: u8, x: i32, y: i32, text: &str) {
    render_ascii_string(ctx, color, x, y, text.as_bytes())
}

pub fn render_glyph(ctx: &mut BTerm, color: u8, x: i32, y: i32, glyph: Glyph) {
    let col = Color::from(color);

    ctx.set(x as i32, y as i32, col.foreground, col.background, glyph);
}
