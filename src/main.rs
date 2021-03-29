extern crate nom;
extern crate num;

#[macro_use]
extern crate num_derive;

mod element;
mod world;

use bracket_lib::prelude as Bracket;
use Bracket::{BTerm, Console};

use element::color::Color;
use element::ElementType;

use world::World;

struct State {
    pub elements: Vec<element::Element>,
}

impl Bracket::GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let element = &self.elements[ElementType::Player as usize];
        let foreground = element.color.foreground.unwrap_or(Color::WHITE);
        let background = element.color.background.unwrap_or(Color::BLACK);

        ctx.cls();

        ctx.set(2, 2, foreground, background, element.character);
    }
}

fn main() {
    let _world = World::load_file("priv/TOWN.ZZT").expect("Cannot load file");

    let term = Bracket::BTermBuilder::vga(80, 25)
        .with_title("ZZTrs")
        .build();

    let gs = State {
        elements: element::ELEMENTS,
    };

    Bracket::main_loop(term, gs);
}
