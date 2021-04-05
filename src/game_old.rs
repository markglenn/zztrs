use bracket_lib::prelude::{self as Bracket, Console};

use crate::{
    color::Color,
    elements::{self, status_element::StatusElement, Element, ElementType},
    math::ZZTPoint,
    sidebar,
    world::{
        board::{BOARD_HEIGHT, BOARD_WIDTH},
        World,
    },
};

pub struct State {
    pub world: World,
    pub current_board: usize,
    pub tick: usize,
    pub sound_enabled: bool,
}

impl State {
    pub fn new<'a>(world: World) -> Self {
        State {
            world,
            current_board: 0,
            tick: 0,
            sound_enabled: true,
        }
    }
}

impl Bracket::GameState for State {
    fn tick(&mut self, ctx: &mut Bracket::BTerm) {
        ctx.cls();
        sidebar::draw(ctx, &self);

        self.tick += 1;
        let board = &mut self.world.boards[self.current_board];

        // Tick status elements
        for status_id in 0..board.status_elements.len() {
            let StatusElement { location, .. } = board.status_elements[status_id];
            let element_id = board
                .tile_at(ZZTPoint {
                    x: location.x - 1,
                    y: location.y - 1,
                })
                .element_id;
            let Element { tick_func, .. } = elements::ELEMENTS[element_id];

            if let Some(func) = tick_func {
                func(board, status_id);
            }
        }

        // Redraw play area
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                let point = ZZTPoint { x, y };
                let tile = board.tile_at(point);
                let element = &elements::ELEMENTS[tile.element_id];

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

                let glyph = match element.glyph_func {
                    Some(func) => func(board, tile, point, self.tick),
                    _ => element.glyph,
                };

                ctx.set(
                    x as i32,
                    y as i32,
                    color.foreground,
                    color.background,
                    glyph,
                );
            }
        }
    }
}
