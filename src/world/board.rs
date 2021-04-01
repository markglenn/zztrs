use crate::world::{le_bool, prefixed_string};
use crate::{elements::status_element::StatusElement, math::ZZTPoint};
use nom::bytes::complete::take;
use nom::error::{make_error, ErrorKind};
use nom::multi::{count, length_data};
use nom::number::complete::{le_i16, le_u16, le_u8};
use nom::sequence::tuple;
use nom::Err::Failure;
use nom::IResult;

#[derive(Debug, Copy, Clone)]
pub struct Tile {
    pub element_id: usize,
    pub color: u8,
}

impl Tile {
    pub fn load(input: &[u8]) -> IResult<&[u8], Vec<Tile>> {
        let (input, (count, element_id, color)) = tuple((le_u8, le_u8, le_u8))(input)?;

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

            match tiles.len() {
                1500 => return Ok((input, tiles)),
                1501..=usize::MAX => return Err(Failure(make_error(input, ErrorKind::Count))),
                _ => (),
            }
        }
    }
}

pub const BOARD_WIDTH: usize = 60;
pub const BOARD_HEIGHT: usize = 25;

#[derive(Debug)]
pub struct Board {
    pub board_name: String,
    tiles: Vec<Vec<Tile>>,

    pub max_player_shots: u8,
    pub is_dark: bool,
    pub exit_north: usize,
    pub exit_south: usize,
    pub exit_west: usize,
    pub exit_east: usize,
    pub restart_on_zap: bool,
    pub message: String,
    pub player_enter_x: usize,
    pub player_enter_y: usize,
    pub timelimit: i16,
    pub status_elements: Vec<StatusElement>,
}

impl Board {
    pub fn status_at(&self, location: ZZTPoint<usize>) -> &StatusElement {
        let location = ZZTPoint {
            x: location.x + 1,
            y: location.y + 1,
        };

        self.status_elements
            .iter()
            .find(|&s| s.location == location)
            .unwrap()
    }

    pub fn tile_at(&self, location: ZZTPoint<usize>) -> Tile {
        self.tiles[location.y][location.x]
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
        let (input, (player_enter_x, player_enter_y)) = tuple((le_u8, le_u8))(input)?;
        let (input, timelimit) = le_i16(input)?;

        let (input, _unused) = take(16usize)(input)?;

        let (input, status_element_count) = le_i16(input)?;

        // Load the status elements (add 1 for player)
        let (input, status_elements) =
            count(StatusElement::load, 1 + status_element_count as usize)(input)?;

        let tiles = tiles.chunks(BOARD_WIDTH).map(|v| v.to_vec()).collect();

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
                player_enter_x: player_enter_x as usize,
                player_enter_y: player_enter_y as usize,
                timelimit,
                status_elements,
            },
        ))
    }
}
