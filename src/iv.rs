use std::fmt::Display;

use super::{accel::Acceleration, displacement::Displacement, fv::FinalVelocity, time::Time};

#[derive(Debug, Clone, Copy)]

pub struct InitialVelocity(pub f32);

impl From<f32> for InitialVelocity {
    fn from(v: f32) -> Self {
        Self(v)
    }
}
impl Display for InitialVelocity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl InitialVelocity {
    pub fn from_displacement_accel_time(s: Displacement, a: Acceleration, t: Time) -> Self {
        ((s.0 - (0.5 * a.0 * t.0.powf(2.0))) / t.0).into()
    }
    pub fn from_displacement_final_time(s: Displacement, fv: FinalVelocity, t: Time) -> Self {
        (2.0 * s.0 / t.0 - fv.0).into()
    }
    pub fn from_displacement_final_accel(
        s: Displacement,
        fv: FinalVelocity,
        a: Acceleration,
    ) -> Self {
        ((fv.0.powf(2.0) - 2.0 * a.0 * s.0).sqrt()).into()
    }
    pub fn from_final_accel_time(fv: FinalVelocity, a: Acceleration, t: Time) -> Self {
        (fv.0 - a.0 * t.0).into()
    }
}
