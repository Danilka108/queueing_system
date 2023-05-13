use std::ops;

#[derive(Clone, Copy, PartialEq, PartialOrd, Default, Debug)]
pub struct Time(f32);

impl From<Time> for f32 {
    fn from(value: Time) -> Self {
        value.0
    }
}

impl Time {
    pub const ZERO: Time = Time(0.0);
}

impl From<f32> for Time {
    fn from(value: f32) -> Self {
        if value < 0.0 {
            panic!("time can't be negative");
        }

        Self(value)
    }
}

impl ops::Sub<Time> for Time {
    type Output = Self;

    fn sub(self, rhs: Time) -> Self::Output {
        if self.0 < rhs.0 {
            panic!("time can't be negative");
        }

        Self(self.0 - rhs.0)
    }
}

impl ops::SubAssign<Time> for Time {
    fn sub_assign(&mut self, rhs: Time) {
        self.0 -= rhs.0;
    }
}

impl ops::Add<Time> for Time {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl ops::AddAssign<Time> for Time {
    fn add_assign(&mut self, rhs: Time) {
        self.0 += rhs.0;
    }
}
