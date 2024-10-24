use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug)]
pub struct Coordinates(u64, u64);

impl Add for Coordinates {
    type Output = (u64, u64);

    fn add(self, rhs: Self) -> Self::Output {
        (self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Coordinates {
    type Output = (u64, u64);

    fn sub(self, rhs: Self) -> Self::Output {
        (self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul for Coordinates {
    type Output = (u64, u64);

    fn mul(self, rhs: Self) -> Self::Output {
        (self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl Div for Coordinates {
    type Output = (u64, u64);

    fn div(self, rhs: Self) -> Self::Output {
        (self.0 / rhs.0, self.1 / rhs.1)
    }
}
