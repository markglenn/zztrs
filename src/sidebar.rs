use crate::{
    elements::{ElementType, ELEMENTS},
    render::{render_ascii_string, render_glyph, render_string},
    State,
};
use bracket_lib::prelude::BTerm;

fn clear_line(ctx: &mut BTerm, y: i32) {
    render_ascii_string(ctx, 0x11, 60, y, b"\xB3                   ");
}

pub fn clear(ctx: &mut BTerm) {
    for y in 3..=24 {
        clear_line(ctx, y);
    }
}

pub fn render(ctx: &mut BTerm, state: &State) {
    clear(ctx);
    clear_line(ctx, 0);
    clear_line(ctx, 1);
    clear_line(ctx, 2);

    render_string(ctx, 0x1F, 61, 0, "    - - - - -      ");
    render_string(ctx, 0x70, 62, 1, "      ZZT      ");
    render_string(ctx, 0x1F, 61, 2, "    - - - - -      ");

    // if GameStateElement = E_PLAYER then begin
    render_in_game(ctx, state);
}

fn render_in_game(ctx: &mut BTerm, _state: &State) {
    render_string(ctx, 0x1E, 64, 7, " Health:");
    render_string(ctx, 0x1E, 64, 8, "   Ammo:");
    render_string(ctx, 0x1E, 64, 9, "Torches:");
    render_string(ctx, 0x1E, 64, 10, "   Gems:");
    render_string(ctx, 0x1E, 64, 11, "  Score:");
    render_string(ctx, 0x1E, 64, 12, "   Keys:");
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
    render_string(ctx, 0x70, 62, 14, " T ");
    render_string(ctx, 0x1F, 65, 14, " Torch");
    render_string(ctx, 0x30, 62, 15, " B ");
    render_string(ctx, 0x70, 62, 16, " H ");
    render_string(ctx, 0x1F, 65, 16, " Help");
    render_ascii_string(ctx, 0x30, 67, 18, b" \x18\x19\x1A\x1B ");
    render_string(ctx, 0x1F, 72, 18, " Move");
    render_ascii_string(ctx, 0x70, 61, 19, b" Shift \x18\x19\x1A\x1B ");
    render_string(ctx, 0x1F, 72, 19, " Shoot");
    render_string(ctx, 0x70, 62, 21, " S ");
    render_string(ctx, 0x1F, 65, 21, " Save game");
    render_string(ctx, 0x30, 62, 22, " P ");
    render_string(ctx, 0x1F, 65, 22, " Pause");
    render_string(ctx, 0x70, 62, 23, " Q ");
    render_string(ctx, 0x1F, 65, 23, " Quit");
    //if state.sound_enabled {
    render_string(ctx, 0x1F, 65, 15, " Be quiet");
    // } else {
    //     render_string(ctx, 0x1F, 65, 15, " Be noisy");
    // }
}
