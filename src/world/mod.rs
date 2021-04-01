pub mod board;
mod header;

use board::Board;
use header::Header;
use nom::multi::count;

use nom::bytes::complete::take;
use nom::combinator::{map, map_res};
use nom::number::complete::le_u8;
use nom::IResult;

use std::io;
use std::io::Read;
use std::{fs::OpenOptions, usize};

#[derive(Debug)]
pub struct World {
    pub header: Header,
    pub boards: Vec<Board>,
}

impl World {
    pub fn load(input: &[u8]) -> IResult<&[u8], World> {
        let (input, header) = Header::load(input)?;
        let (input, _skip) = take(233 as usize)(input)?;

        let (input, boards) = count(Board::load, header.num_boards as usize)(input)?;

        Ok((input, World { header, boards }))
    }

    pub fn load_file(filename: &str) -> Result<World, io::Error> {
        let mut file = OpenOptions::new().read(true).open(filename)?;

        let mut contents: Vec<u8> = Vec::new();
        file.read_to_end(&mut contents)?;

        match World::load(&contents) {
            Ok((_, world)) => Ok(world),
            Err(_) => Err(io::Error::from(io::ErrorKind::InvalidData)),
        }
    }
}

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
