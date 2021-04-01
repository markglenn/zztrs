use nom::bytes::complete::take;
use nom::combinator::map_res;
use nom::multi::count;
use nom::number::complete::le_i16;
use nom::number::complete::le_i32;
use nom::number::complete::le_u8;
use nom::sequence::tuple;
use nom::IResult;

use crate::math::ZZTPoint;

#[derive(Debug)]
pub struct StatusElement {
    pub location: ZZTPoint<usize>,
    pub step_x: i16,
    pub step_y: i16,
    pub cycle: i16,
    pub p1: u8,
    pub p2: u8,
    pub p3: u8,
    pub follower: i16,
    pub leader: i16,
    pub under_id: u8,
    pub under_color: u8,
    pub current_instruction: i16,
    pub length: i16,
    pub code: Option<String>,
}

impl StatusElement {
    pub fn load(input: &[u8]) -> IResult<&[u8], StatusElement> {
        let (input, location) = ZZTPoint::<usize>::load(input)?;
        let (input, (step_x, step_y)) = tuple((le_i16, le_i16))(input)?;

        let (input, cycle) = le_i16(input)?;
        let (input, (p1, p2, p3)) = tuple((le_u8, le_u8, le_u8))(input)?;
        let (input, (follower, leader)) = tuple((le_i16, le_i16))(input)?;
        let (input, (under_id, under_color)) = tuple((le_u8, le_u8))(input)?;
        let (input, _pointer) = le_i32(input)?;
        let (input, current_instruction) = le_i16(input)?;
        let (input, length) = le_i16(input)?;

        // Ignored padding
        let (input, _padding) = count(le_u8, 8)(input)?;

        let (input, code) = if length > 0 {
            let (input, s) =
                map_res(take(length as usize), |v: &[u8]| std::str::from_utf8(v))(input)?;

            (input, Some(String::from(s)))
        } else {
            (input, None)
        };

        Ok((
            input,
            StatusElement {
                location,
                step_x,
                step_y,
                cycle,
                p1,
                p2,
                p3,
                follower,
                leader,
                under_id,
                under_color,
                current_instruction,
                length,
                code,
            },
        ))
    }
}
