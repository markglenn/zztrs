extern crate nom;
extern crate num;

#[macro_use]
extern crate num_derive;

mod components;
mod elements;
mod game;
mod loader;
mod render;
mod sidebar;
mod systems;

use bracket_lib::prelude as Bracket;

use components::{Color, Position, Renderable};
use game::State;
use specs::{Builder, World, WorldExt};

fn main() {
    let _world = loader::World::load_file("priv/TOWN.ZZT").expect("Cannot load file");

    let term = Bracket::BTermBuilder::vga(80, 25)
        .with_title("ZZTrs")
        .build();

    let mut gs = State { ecs: World::new() };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();

    let player_entity = gs
        .ecs
        .create_entity()
        .with(Position { x: 1, y: 1 })
        .with(Renderable {
            glyph: 0x01,
            color: Color::new(0x0F),
        })
        .build();

    gs.ecs.insert(player_entity);

    Bracket::main_loop(term, gs);
}
