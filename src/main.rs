extern crate nom;
extern crate num;

#[macro_use]
extern crate num_derive;

mod color;
mod elements;
mod game;
mod math;
mod render;
mod sidebar;
mod world;

use bracket_lib::prelude as Bracket;

use color::Color;

use game::State;
use world::World;

fn main() {
    let world = World::load_file("priv/TOWN.ZZT").expect("Cannot load file");

    let term = Bracket::BTermBuilder::vga(80, 25)
        .with_title("ZZTrs")
        .build();

    let gs = State::new(world);

    Bracket::main_loop(term, gs);
}
