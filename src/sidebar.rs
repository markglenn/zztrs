use crate::{
    elements::{ElementType, ELEMENTS},
    render::{render_glyph, render_string},
    State,
};
use bracket_lib::prelude::BTerm;

fn clear_line(ctx: &mut BTerm, y: usize) {
    render_string(ctx, 0x11, 60, y, &b"\xB3                   "[..]);
}

pub fn clear(ctx: &mut BTerm) {
    for y in 3..=24 {
        clear_line(ctx, y);
    }
}

pub fn draw(ctx: &mut BTerm, state: &State) {
    clear(ctx);
    clear_line(ctx, 0);
    clear_line(ctx, 1);
    clear_line(ctx, 2);

    render_string(ctx, 0x1F, 61, 0, &b"    - - - - -      "[..]);
    render_string(ctx, 0x70, 62, 1, &b"      ZZT      "[..]);
    render_string(ctx, 0x1F, 61, 2, &b"    - - - - -      "[..]);

    // if GameStateElement = E_PLAYER then begin
    render_in_game(ctx, state);
}

fn render_in_game(ctx: &mut BTerm, state: &State) {
    render_string(ctx, 0x1E, 64, 7, &b" Health:"[..]);
    render_string(ctx, 0x1E, 64, 8, &b"   Ammo:"[..]);
    render_string(ctx, 0x1E, 64, 9, &b"Torches:"[..]);
    render_string(ctx, 0x1E, 64, 10, &b"   Gems:"[..]);
    render_string(ctx, 0x1E, 64, 11, &b"  Score:"[..]);
    render_string(ctx, 0x1E, 64, 12, &b"   Keys:"[..]);
    render_glyph(
        ctx,
        0x1F,
        62,
        7,
        ELEMENTS[ElementType::Player as usize].glyph,
    );
    render_glyph(ctx, 0x1B, 62, 8, ELEMENTS[ElementType::Ammo as usize].glyph);
    render_glyph(
        ctx,
        0x16,
        62,
        9,
        ELEMENTS[ElementType::Torch as usize].glyph,
    );
    render_glyph(ctx, 0x1B, 62, 10, ELEMENTS[ElementType::Gem as usize].glyph);
    render_glyph(ctx, 0x1F, 62, 12, ELEMENTS[ElementType::Key as usize].glyph);
    render_string(ctx, 0x70, 62, 14, &b" T "[..]);
    render_string(ctx, 0x1F, 65, 14, &b" Torch"[..]);
    render_string(ctx, 0x30, 62, 15, &b" B "[..]);
    render_string(ctx, 0x70, 62, 16, &b" H "[..]);
    render_string(ctx, 0x1F, 65, 16, &b" Help"[..]);
    render_string(ctx, 0x30, 67, 18, &b" \x18\x19\x1A\x1B "[..]);
    render_string(ctx, 0x1F, 72, 18, &b" Move"[..]);
    render_string(ctx, 0x70, 61, 19, &b" Shift \x18\x19\x1A\x1B "[..]);
    render_string(ctx, 0x1F, 72, 19, &b" Shoot"[..]);
    render_string(ctx, 0x70, 62, 21, &b" S "[..]);
    render_string(ctx, 0x1F, 65, 21, &b" Save game"[..]);
    render_string(ctx, 0x30, 62, 22, &b" P "[..]);
    render_string(ctx, 0x1F, 65, 22, &b" Pause"[..]);
    render_string(ctx, 0x70, 62, 23, &b" Q "[..]);
    render_string(ctx, 0x1F, 65, 23, &b" Quit"[..]);
    if state.sound_enabled {
        render_string(ctx, 0x1F, 65, 15, &b" Be quiet"[..]);
    } else {
        render_string(ctx, 0x1F, 65, 15, &b" Be noisy"[..]);
    }
}
