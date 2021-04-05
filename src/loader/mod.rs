pub mod board;
mod header;
mod world;

pub use board::Board;
pub use header::Header;
pub use world::World;

use nom::{
    bytes::complete::take,
    combinator::{map, map_res},
    number::complete::le_u8,
    sequence::tuple,
    IResult,
};

use crate::components::Position;

pub const BOARD_WIDTH: usize = 60;
pub const BOARD_HEIGHT: usize = 25;
pub const BOARD_SIZE: usize = BOARD_WIDTH * BOARD_HEIGHT;

pub fn prefixed_string(max_length: usize) -> impl FnMut(&[u8]) -> IResult<&[u8], String> {
    let check_max = move |v: u8| {
        if v as usize > max_length {
            Err("Invalid string length")
        } else {
            Ok(v)
        }
    };

    move |input: &[u8]| {
        let (input, length) = map_res(le_u8, check_max)(input)?;
        let (input, s) = map_res(take(max_length), |v: &[u8]| {
            let slice = &v[..length as usize];
            std::str::from_utf8(slice)
        })(input)?;

        Ok((input, String::from(s)))
    }
}

pub fn le_bool(input: &[u8]) -> IResult<&[u8], bool> {
    map(le_u8, |v| v != 0)(input)
}

pub fn u8_position(input: &[u8]) -> IResult<&[u8], Position> {
    let (input, (x, y)) = tuple((le_u8, le_u8))(input)?;

    Ok((
        input,
        Position {
            x: x as i32,
            y: y as i32,
        },
    ))
}

pub fn load_u8_position_offset_1(input: &[u8]) -> IResult<&[u8], Position> {
    let (input, (x, y)) = tuple((le_u8, le_u8))(input)?;

    Ok((
        input,
        Position {
            x: x as i32 - 1,
            y: y as i32 - 1,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::{le_bool, prefixed_string};
    const EMPTY: [u8; 0] = [];

    #[test]
    fn reads_string() {
        let v = "\x05hello               ";

        assert_eq!(
            prefixed_string(v.len() - 1)(v.as_bytes()).unwrap(),
            (&EMPTY[..], String::from("hello"))
        );
    }

    #[test]
    fn reads_bool() {
        let falsy = vec![0 as u8; 1];
        let truthy = vec![1 as u8; 1];
        assert_eq!(le_bool(&falsy[..]).unwrap(), (&EMPTY[..], false));
        assert_eq!(le_bool(&truthy[..]).unwrap(), (&EMPTY[..], true));
    }
}
