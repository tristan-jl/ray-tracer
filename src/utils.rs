use rand::Rng;
use std::f64::consts::PI;

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.
}

pub fn random_f64(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min, max)
}

pub const INFINITY: f64 = f64::MAX;
