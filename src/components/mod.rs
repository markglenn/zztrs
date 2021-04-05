pub(crate) mod board;
mod color;
mod position;

use specs::prelude::*;
use specs_derive::Component;

pub use board::Board;
pub use color::Color;

pub type Glyph = u8;

pub use position::Position;

#[derive(Component, Debug)]
pub struct Renderable {
    pub glyph: Glyph,
    pub color: Color,
}
