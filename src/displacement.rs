use std::fmt::Display;

use super::{accel::Acceleration, fv::FinalVelocity, iv::InitialVelocity, time::Time};
#[derive(Debug, Clone, Copy)]
pub struct Displacement(pub f32);
impl From<f32> for Displacement {
    fn from(v: f32) -> Self {
        Self(v)
    }
}
impl Display for Displacement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Displacement {
    pub fn from_initial_accel_time(iv: InitialVelocity, a: Acceleration, t: Time) -> Self {
        (iv.0 * t.0 + 0.5 * a.0 * t.0.powf(2.0)).into()
    }
    pub fn from_initial_final_time(iv: InitialVelocity, fv: FinalVelocity, t: Time) -> Self {
        (0.5 * (iv.0 + fv.0) * t.0).into()
    }
    pub fn from_initial_final_accel(
        iv: InitialVelocity,
        fv: FinalVelocity,
        a: Acceleration,
    ) -> Self {
        ((fv.0.powf(2.0) - iv.0.powf(2.0)) / (2.0 * a.0)).into()
    }
    pub fn from_final_accel_time(fv: FinalVelocity, a: Acceleration, t: Time) -> Self {
        let iv = InitialVelocity::from_final_accel_time(fv, a, t);
        Self::from_initial_accel_time(iv, a, t)
    }
}

impl std::ops::Add for Displacement {
    type Output = Displacement;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
