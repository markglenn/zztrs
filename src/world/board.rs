use crate::world::prefixed_string;
use nom::error::make_error;
use nom::error::ErrorKind;
use nom::number::complete::le_i16;
use nom::number::complete::le_u8;
use nom::sequence::tuple;
use nom::Err::Failure;
use nom::IResult;

#[derive(Debug, Copy, Clone)]
pub struct Tile {
    pub element_id: u8,
    pub color: u8,
}

impl Tile {
    pub fn load(input: &[u8]) -> IResult<&[u8], Vec<Tile>> {
        let (input, (count, element_id, color)) = tuple((le_u8, le_u8, le_u8))(input)?;

        Ok((
            input,
            std::iter::repeat(Tile { element_id, color })
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

#[derive(Debug)]
pub struct Board {
    pub board_size: i16,
    pub board_name: String,
    pub tiles: Vec<Tile>,
}

impl Board {
    pub fn load(input: &[u8]) -> IResult<&[u8], Board> {
        let (input, board_size) = le_i16(input)?;
        let (input, board_name) = prefixed_string(50)(input)?;
        let (input, tiles) = Tile::load_rle(input)?;

        Ok((
            input,
            Board {
                board_size,
                board_name,
                tiles,
            },
        ))
    }
}
