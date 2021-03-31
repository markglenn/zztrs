extern crate nom;
extern crate num;

#[macro_use]
extern crate num_derive;

mod color;
mod elements;
mod render;
mod sidebar;
mod world;

use bracket_lib::prelude as Bracket;
use Bracket::{BTerm, Console};

use color::Color;
use elements::ElementType;

use world::{
    board::{BOARD_HEIGHT, BOARD_WIDTH},
    World, ZZTPoint,
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
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        let board = &self.world.boards[self.current_board];
        self.tick += 1;
        sidebar::draw(ctx, &self);

        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                let tile = &board.tiles[y][x];
                let element = &elements::ELEMENTS[tile.element_id as usize];

                let color = match element.element_type {
                    ElementType::Empty
                    | ElementType::TextBlue
                    | ElementType::TextGreen
                    | ElementType::TextCyan
                    | ElementType::TextRed
                    | ElementType::TextPurple
                    | ElementType::TextBrown
                    | ElementType::TextBlack => element.color,
                    _ => Color::new(tile.color),
                };

                let glyph = match element.glyph_func {
                    Some(func) => func(board, tile, ZZTPoint { x, y }, self.tick),
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

fn main() {
    let world = World::load_file("priv/TOWN.ZZT").expect("Cannot load file");

    let term = Bracket::BTermBuilder::vga(80, 25)
        .with_title("ZZTrs")
        .build();

    let gs = State::new(world);

    Bracket::main_loop(term, gs);
}
