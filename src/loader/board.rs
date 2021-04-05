use crate::{components::Position, elements::status_element::StatusElement};
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

#[derive(Debug, Copy, Clone)]
pub struct Tile {
    pub element_id: usize,
    pub color: u8,
}

impl Tile {
    pub fn load(input: &[u8]) -> IResult<&[u8], Vec<Tile>> {
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

    pub fn load_rle(input: &[u8]) -> IResult<&[u8], Vec<Tile>> {
        let mut tiles = Vec::<Tile>::new();
        let mut input = input;

        loop {
            let mut results = Tile::load(input)?;

            tiles.append(&mut results.1);
            input = results.0;

            if tiles.len() == BOARD_SIZE {
                return Ok((input, tiles));
            } else if tiles.len() > BOARD_SIZE {
                return Err(Failure(make_error(input, ErrorKind::Count)));
            }
        }
    }
}

#[derive(Debug)]
pub struct Board {
    pub board_name: String,
    tiles: Vec<Tile>,

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
    pub status_elements: Vec<StatusElement>,
}

impl Board {
    pub fn status_at(&self, location: Position) -> &StatusElement {
        self.status_elements
            .iter()
            .find(|&s| s.location == location)
            .unwrap()
    }

    pub fn tile_at(&self, location: Position) -> Tile {
        self.tiles[location.to_index()]
    }

    pub fn move_status(&mut self, status_id: usize, new_location: Position) {
        let StatusElement {
            location,
            under_color,
            under_id,
            ..
        } = self.status_elements[status_id];

        let old_tile = self.tiles[location.to_index()];
        let new_tile = self.tiles[new_location.to_index()];

        // Update the status to point to the new location
        {
            let status = &mut self.status_elements[status_id];

            status.under_id = new_tile.element_id;
            status.under_color = new_tile.color;
            status.location = new_location;
        }

        // Update the tiles
        self.tiles[location.to_index()] = Tile {
            color: under_color,
            element_id: under_id,
        };

        self.tiles[new_location.to_index()] = Tile {
            color: old_tile.color,
            element_id: old_tile.element_id,
        };
    }

    pub fn load(input: &[u8]) -> IResult<&[u8], Board> {
        // Load the board data as a blob
        let (input, data) = length_data(le_u16)(input)?;
        let (_, board) = Self::load_board(data)?;

        Ok((input, board))
    }

    fn load_board(input: &[u8]) -> IResult<&[u8], Board> {
        let (input, board_name) = prefixed_string(50)(input)?;
        let (input, tiles) = Tile::load_rle(input)?;

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
        let (input, status_elements) =
            count(StatusElement::load, 1 + status_element_count as usize)(input)?;

        Ok((
            input,
            Board {
                board_name,
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
                status_elements,
            },
        ))
    }
}
