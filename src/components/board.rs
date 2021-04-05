use bracket_lib::prelude::{BTerm, BaseMap, Console};
use specs::World;

use crate::{
    elements::{ElementType, ELEMENTS},
    loader::BOARD_WIDTH,
};

use super::{Color, Position};

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub element_id: usize,
    pub color: u8,
}

#[derive(Debug)]
pub struct Board {
    pub name: String,
    pub tiles: Vec<Tile>,
    pub max_player_shots: u8,
    pub is_dark: bool,
    pub exit_north: usize,
    pub exit_south: usize,
    pub exit_west: usize,
    pub exit_east: usize,
    pub restart_on_zap: bool,
    pub message: String,
    pub player_enter: Position,
    pub timelimit: i16,
}

pub fn draw_board(ecs: &World, ctx: &mut BTerm) {
    let board = ecs.fetch::<Board>();

    for (idx, tile) in board.tiles.iter().enumerate() {
        let element = &ELEMENTS[tile.element_id];

        let color = match element.element_type {
            ElementType::Empty
            | ElementType::TextBlue
            | ElementType::TextGreen
            | ElementType::TextCyan
            | ElementType::TextRed
            | ElementType::TextPurple
            | ElementType::TextBrown
            | ElementType::TextBlack => element.color,
            _ => Color::from(tile.color),
        };

        let x = (idx % BOARD_WIDTH) as i32;
        let y = (idx / BOARD_WIDTH) as i32;

        let glyph = match element.glyph_func {
            // Some(func) => func(board, tile, point, self.tick),
            _ => element.glyph,
        };

        ctx.set(x, y, color.foreground, color.background, glyph);
    }
}

impl BaseMap for Board {}
