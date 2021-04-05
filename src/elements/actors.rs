use rand::Rng;

use crate::{components::Position, loader::board::Board};

use super::{status_element::StatusElement, ELEMENTS};

pub fn lion_tick(board: &mut Board, status_id: usize) {
    let mut rng = rand::thread_rng();
    let StatusElement { p1, location, .. } = board.status_elements[status_id];
    let location = Position {
        x: location.x - 1,
        y: location.y - 1,
    };

    let delta = if p1 < rng.gen_range(0..10) {
        random_direction()
    } else {
        seek_direction(board, location)
    };

    let location = location + delta;

    let tile = board.tile_at(location);

    if ELEMENTS[tile.element_id].walkable {
        board.move_status(status_id, location);
    } else {
    }
}

pub fn ruffian_tick(board: &mut Board, status_id: usize) {
    lion_tick(board, status_id);
}

pub fn tiger_tick(board: &mut Board, status_id: usize) {
    lion_tick(board, status_id);
}

fn signum(first: i32, second: i32) -> i32 {
    if first == second {
        0
    } else if first > second {
        1
    } else {
        -1
    }
}

pub fn random_direction() -> Position {
    let mut rng = rand::thread_rng();

    match rng.gen_range(-1..=1) {
        0 => Position {
            x: 0,
            y: rng.gen_range(-1..=1),
        },
        dx => Position { x: dx, y: 0 },
    }
}

fn seek_direction(board: &Board, location: Position) -> Position {
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

    Position {
        x: delta_x,
        y: delta_y,
    }
}
