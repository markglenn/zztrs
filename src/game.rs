use bracket_lib::prelude::{self as Bracket, Console};
use rand::Rng;

use crate::{
    color::Color,
    elements::{self, ElementType},
    math::ZZTPoint,
    sidebar,
    world::{
        board::{Board, BOARD_HEIGHT, BOARD_WIDTH},
        World,
    },
};

fn signum(first: usize, second: usize) -> i32 {
    if first == second {
        0
    } else if first > second {
        1
    } else {
        -1
    }
}

pub fn random_direction() -> ZZTPoint<i32> {
    let mut rng = rand::thread_rng();

    match rng.gen_range(-1..=1) {
        0 => ZZTPoint::new(0, rng.gen_range(-1..=1)),
        dx => ZZTPoint::new(dx, 0),
    }
}

pub fn seek_direction(board: &Board, location: ZZTPoint<usize>) -> ZZTPoint<i32> {
    let mut rng = rand::thread_rng();

    let mut delta_x = 0;
    let mut delta_y = 0;

    let player = &board.status_elements[0];

    if rng.gen_range(0..=1) == 0 || player.location.y == location.y {
        delta_x = signum(player.location.x, location.x);
    }

    if delta_x == 0 {
        delta_y = signum(player.location.y, location.y);
    }

    // if World.Info.EnergizerTicks > 0 then begin
    //     deltaX := -deltaX;
    //     deltaY := -deltaY;
    // end;

    ZZTPoint::new(delta_x, delta_y)
}

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
