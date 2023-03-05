pub trait Square {
    fn sq(&self) -> Self;
}

pub trait IsVeryClose{
    fn is_very_close_to(&self, other: &Self) -> bool;
}

pub trait Minimum{
    fn min<'a>(&'a self, other: &'a Self) -> &'a Self;
}
