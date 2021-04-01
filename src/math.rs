use std::ops::Add;

use nom::{number::complete::le_u8, sequence::tuple, IResult};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ZZTPoint<T> {
    pub x: T,
    pub y: T,
}

impl<T> ZZTPoint<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: From<u8>> ZZTPoint<T> {
    pub fn load(input: &[u8]) -> IResult<&[u8], ZZTPoint<T>> {
        let (input, (x, y)) = tuple((le_u8, le_u8))(input)?;

        Ok((
            input,
            ZZTPoint {
                x: T::from(x),
                y: T::from(y),
            },
        ))
    }
}

impl<T, R> Add<ZZTPoint<R>> for ZZTPoint<T>
where
    T: Add<Output = T>,
    R: Into<T>,
{
    type Output = Self;

    fn add(self, rhs: ZZTPoint<R>) -> Self::Output {
        let x = self.x + rhs.x.into();
        let y = self.y + rhs.y.into();

        Self::Output { x, y }
    }
}
