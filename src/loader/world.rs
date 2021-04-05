use std::fs::OpenOptions;
use std::io::{self, Read};

use nom::{bytes::complete::take, IResult};

use crate::components::Board;

use super::{board::load_board, Header};

#[derive(Debug)]
pub struct World {
    pub header: Header,
    pub boards: Vec<Board>,
}

impl World {
    pub fn load(input: &[u8]) -> IResult<&[u8], World> {
        let (input, header) = Header::load(input)?;

        let (input, _skip) = take(233 as usize)(input)?;
        let (input, boards) = nom::multi::count(load_board, header.num_boards as usize)(input)?;

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
