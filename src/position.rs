use std::ops::{AddAssign, DivAssign, Neg};
use na::Translation3;
use crate::traits::{IsVeryClose, Square};

#[derive(Default, Clone, Debug)]
pub(crate) struct Position{
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Position {

    pub(crate) fn midpoint(&self, other: &Position) -> Position{
        Position{x: (self.x + other.x)/2.0,
            y: (self.y + other.y)/2.0,
            z: (self.z + other.z)/2.0}
    }

    pub(crate) fn to_translation(&self) -> Translation3<f32> {
        Translation3::new(self.x, self.y, self.z)
    }

    pub(crate) fn distance_to(&self, other: &Self) -> f32{
        ((self.x - other.x).sq() + (self.y - other.y).sq() + (self.z - other.z).sq()).sqrt()
    }
}

impl AddAssign<&Self> for Position {
    fn add_assign(&mut self, rhs: &Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl DivAssign<f32> for Position {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Neg for Position {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Position{x: -self.x, y: -self.y, z: -self.z}
    }
}

impl IsVeryClose for Position {
    fn is_very_close_to(&self, other: &Self) -> bool {
        self.x.is_very_close_to(&other.x)
            && self.y.is_very_close_to(&other.y)
            && self.z.is_very_close_to(&other.z)
    }
}
