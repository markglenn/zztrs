use crate::{
    components::{Position, Renderable},
    sidebar,
};
use bracket_lib::prelude::*;
use specs::{Join, World, WorldExt};

pub struct State {
    pub ecs: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        sidebar::render(ctx, self);

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(
                pos.x,
                pos.y,
                render.color.foreground,
                render.color.background,
                render.glyph,
            );
        }
    }
}
