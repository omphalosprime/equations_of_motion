use std::fmt::Display;

use super::{displacement::Displacement, fv::FinalVelocity, iv::InitialVelocity, time::Time};

#[derive(Debug, Clone, Copy)]
pub struct Acceleration(pub f32);
impl From<f32> for Acceleration {
    fn from(v: f32) -> Self {
        Self(v)
    }
}
impl Display for Acceleration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Acceleration {
    pub fn from_initial_final_time(iv: InitialVelocity, fv: FinalVelocity, t: Time) -> Self {
        ((fv.0 - iv.0) / t.0).into()
    }
    pub fn from_displacement_initial_final(
        s: Displacement,
        iv: InitialVelocity,
        fv: FinalVelocity,
    ) -> Self {
        ((fv.0.powf(2.0) - iv.0.powf(2.0)) / (2.0 * s.0)).into()
    }
    pub fn from_displacement_initial_time(s: Displacement, iv: InitialVelocity, t: Time) -> Self {
        ((2.0 * (s.0 - iv.0 * t.0)) / t.0.powf(2.0)).into()
    }
    pub fn from_displacement_final_time(s: Displacement, fv: FinalVelocity, t: Time) -> Self {
        ((2.0 * (fv.0 * t.0 - s.0)) / t.0.powf(2.0)).into()
    }
}
