use std::ops::{Add, Sub};

use specs::prelude::*;
use specs_derive::Component;

use crate::world::BOARD_WIDTH;

#[derive(Clone, Copy, Component, Debug, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    #[allow(dead_code)]
    pub fn to_index(&self) -> usize {
        (self.y as usize * BOARD_WIDTH) + self.x as usize
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
