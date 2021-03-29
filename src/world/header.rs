use super::{le_bool, prefixed_string};
use nom::error::make_error;
use nom::error::ErrorKind;
use nom::multi::count;
use nom::number::complete::{le_i16, le_u8};
use nom::Err::Failure;
use nom::IResult;
use num::FromPrimitive;

pub const NUM_KEYS: usize = 7;
pub const NUM_FLAGS: usize = 10;

#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive)]
pub enum Keys {
    Blue,
    Green,
    Cyan,
    Red,
    Purple,
    Yellow,
    White,
}

#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive)]
pub enum WorldType {
    ZZT = -1,      // *.ZZT
    SuperZZT = -2, // *.SZT
}

impl WorldType {
    pub fn load(input: &[u8]) -> IResult<&[u8], WorldType> {
        let (input, result) = le_i16(input)?;

        match FromPrimitive::from_i16(result) {
            Some(world_type) => Ok((input, world_type)),
            None => Err(Failure(make_error(input, ErrorKind::IsA))),
        }
    }
}

#[derive(Debug)]
pub struct Header {
    pub world_type: WorldType,
    pub num_boards: i16,
    pub player_ammo: i16,
    pub player_gems: i16,
    pub keys: Vec<bool>,
    pub player_health: i16,
    pub player_board: i16,

    // Extended ZZT header
    pub player_torches: i16,
    pub torch_cycles: i16,
    pub energy_cycles: i16,
    pub player_score: i16,
    pub world_name: String,
    pub flags: Vec<String>,

    pub time_passed: i16,
    pub time_passed_ticks: i16,
    pub locked: bool,
}

impl Header {
    pub fn load(input: &[u8]) -> IResult<&[u8], Header> {
        let mut fs20 = prefixed_string(20);

        let (input, world_type) = WorldType::load(input)?;
        let (input, num_boards) = le_i16(input)?;
        let (input, player_ammo) = le_i16(input)?;
        let (input, player_gems) = le_i16(input)?;
        let (input, keys) = count(le_bool, NUM_KEYS as usize)(input)?;
        let (input, player_health) = le_i16(input)?;
        let (input, player_board) = le_i16(input)?;
        let (input, player_torches) = le_i16(input)?;
        let (input, torch_cycles) = le_i16(input)?;
        let (input, energy_cycles) = le_i16(input)?;
        let (input, _unused) = le_i16(input)?;
        let (input, player_score) = le_i16(input)?;
        let (input, world_name) = fs20(input)?;

        let (input, flags) = count(fs20, NUM_FLAGS)(input)?;

        let (input, time_passed) = le_i16(input)?;
        let (input, time_passed_ticks) = le_i16(input)?;
        let (input, locked) = le_bool(input)?;
        let (input, _) = count(le_u8, 14)(input)?;

        Ok((
            input,
            Header {
                world_type,
                num_boards,
                player_ammo,
                player_gems,
                keys,
                player_health,
                player_board,
                player_torches,
                torch_cycles,
                energy_cycles,
                player_score,
                world_name,
                flags,
                time_passed,
                time_passed_ticks,
                locked,
            },
        ))
    }
}
