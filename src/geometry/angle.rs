use std::f64::consts::PI;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Angle {
    rad: f64,
}

impl Angle {
    pub fn rad(rad: f64) -> Self {
        Angle { rad }
    }

    pub fn deg(deg: f64) -> Self {
        Angle {
            rad: deg / 180. * PI,
        }
    }

    pub fn as_rad(self) -> f64 {
        self.rad
    }

    pub fn as_deg(self) -> f64 {
        self.rad / PI * 180.
    }
}
