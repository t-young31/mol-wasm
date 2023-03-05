use crate::traits::{IsVeryClose, Square, Minimum};


impl Square for f32 {
    fn sq(&self) -> f32 {
        self * self
    }
}

impl IsVeryClose for f32 {
    fn is_very_close_to(&self, other: &f32) -> bool{
        (*self - *other).abs() < 1E-16
    }
}

impl Minimum for f32 {
    fn min<'a>(&'a self, other: &'a Self) -> &'a Self {
        if self < other{
            self
        } else {
            other
        }
    }
}
