use std::fmt::Display;

use super::{
    accel::Acceleration, displacement::Displacement, fv::FinalVelocity, iv::InitialVelocity,
};

#[derive(Debug, Clone, Copy, PartialEq)]

pub struct Time(pub f32);
impl From<f32> for Time {
    fn from(v: f32) -> Self {
        Time(v)
    }
}
impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Time {
    pub fn from_initial_final_accel(
        iv: InitialVelocity,
        fv: FinalVelocity,
        a: Acceleration,
    ) -> Self {
        ((fv.0 - iv.0) / a.0).into()
    }

    pub fn from_initial_final_displacement(
        iv: InitialVelocity,
        fv: FinalVelocity,
        s: Displacement,
    ) -> Self {
        (s.0 / (0.5 * (iv.0 + fv.0))).into()
    }

    pub fn from_displacement_initial_accel_quad(
        s: Displacement,
        iv: InitialVelocity,
        a: Acceleration,
    ) -> (Self, Self) {
        (
            ((-iv.0 + (iv.0.powf(2.0) + 2.0 * a.0 * s.0).sqrt()) / a.0).into(),
            ((-iv.0 - (iv.0.powf(2.0) + 2.0 * a.0 * s.0).sqrt()) / a.0).into(),
        )
    }

    pub fn from_displacement_initial_accel(
        s: Displacement,
        iv: InitialVelocity,
        a: Acceleration,
    ) -> Self {
        let fv = FinalVelocity::from_displacement_initial_accel(s, iv, a);
        Self::from_initial_final_accel(iv, fv, a)
    }
    pub fn from_displacement_final_accel(
        s: Displacement,
        fv: FinalVelocity,
        a: Acceleration,
    ) -> Self {
        let iv = InitialVelocity::from_displacement_final_accel(s, fv, a);
        Self::from_initial_final_accel(iv, fv, a)
    }
}