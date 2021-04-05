use crate::{
    components::{board::Tile, Board},
    elements::status_element::StatusElement,
};
use nom::{
    bytes::complete::take,
    error::{make_error, ErrorKind},
    multi::{count, length_data},
    number::complete::{le_i16, le_u16, le_u8},
    sequence::tuple,
    Err::Failure,
    IResult,
};

use super::{le_bool, prefixed_string, u8_position, BOARD_SIZE};

fn load_tile(input: &[u8]) -> IResult<&[u8], Vec<Tile>> {
    let mut load_tile = tuple((le_u8, le_u8, le_u8));
    let (input, (count, element_id, color)) = load_tile(input)?;

    Ok((
        input,
        std::iter::repeat(Tile {
            element_id: element_id as usize,
            color,
        })
        .take(count as usize)
        .collect(),
    ))
}

fn load_tile_rle(input: &[u8]) -> IResult<&[u8], Vec<Tile>> {
    let mut tiles = Vec::<Tile>::new();
    let mut input = input;

    loop {
        let mut results = load_tile(input)?;

        tiles.append(&mut results.1);
        input = results.0;

        if tiles.len() == BOARD_SIZE {
            return Ok((input, tiles));
        } else if tiles.len() > BOARD_SIZE {
            return Err(Failure(make_error(input, ErrorKind::Count)));
        }
    }
}

pub fn load_board(input: &[u8]) -> IResult<&[u8], Board> {
    // Load the board data as a blob
    let (input, data) = length_data(le_u16)(input)?;
    let (_, board) = load_board_impl(data)?;

    Ok((input, board))
}

fn load_board_impl(input: &[u8]) -> IResult<&[u8], Board> {
    let (input, name) = prefixed_string(50)(input)?;
    let (input, tiles) = load_tile_rle(input)?;

    let (input, max_player_shots) = le_u8(input)?;
    let (input, is_dark) = le_bool(input)?;

    let (input, (exit_north, exit_south, exit_west, exit_east)) =
        tuple((le_u8, le_u8, le_u8, le_u8))(input)?;

    let (input, restart_on_zap) = le_bool(input)?;
    let (input, message) = prefixed_string(58)(input)?;
    let (input, player_enter) = u8_position(input)?;
    let (input, timelimit) = le_i16(input)?;

    let (input, _unused) = take(16usize)(input)?;

    let (input, status_element_count) = le_i16(input)?;

    // Load the status elements (add 1 for player)
    let (input, _status_elements) =
        count(StatusElement::load, 1 + status_element_count as usize)(input)?;

    Ok((
        input,
        Board {
            name,
            tiles,
            max_player_shots,
            is_dark,
            exit_north: exit_north as usize,
            exit_south: exit_south as usize,
            exit_west: exit_west as usize,
            exit_east: exit_east as usize,
            restart_on_zap,
            message,
            player_enter,
            timelimit,
            //status_elements,
        },
    ))
}
