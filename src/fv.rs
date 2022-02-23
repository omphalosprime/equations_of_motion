use std::fmt::Display;

use super::{accel::Acceleration, iv::InitialVelocity, time::Time, displacement::Displacement};
#[derive(Debug, Clone, Copy)]

pub struct FinalVelocity(pub f32);
impl From<f32> for FinalVelocity {
    fn from(v: f32) -> Self {
        Self(v)
    }
}
impl Display for FinalVelocity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl FinalVelocity {
    pub fn from_initial_accel_time(iv: InitialVelocity, a: Acceleration, t: Time) -> Self {
        (iv.0 + a.0 * t.0).into()
    }
    pub fn from_displacement_initial_accel(
        s: Displacement,
        iv: InitialVelocity,
        a: Acceleration,
    ) -> Self {
        ((iv.0.powf(2.0) + 2.0 * a.0 * s.0).sqrt()).into()
    }
    pub fn from_displacement_initial_time(s: Displacement, iv: InitialVelocity, t: Time) -> Self {
        ((2.0 * s.0) / t.0 - iv.0).into()
    }
    pub fn from_displacement_accel_time(s: Displacement, a: Acceleration, t: Time) -> Self {
        ((s.0 + 0.5 * a.0 * t.0.powf(2.0)) / t.0).into()
    }
}
