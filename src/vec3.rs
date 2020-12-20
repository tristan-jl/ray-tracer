use crate::utils::random_f64;
use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub},
};

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    pub fn new() -> Self {
        Self { e: [0., 0., 0.] }
    }
    pub fn from(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.squared().sqrt()
    }
    pub fn squared(&self) -> f64 {
        dot(*self, *self)
    }
    pub fn unit_vector(&self) -> Self {
        unit_vector(*self)
    }

    pub fn random(min: f64, max: f64) -> Self {
        Self::from(
            random_f64(min, max),
            random_f64(min, max),
            random_f64(min, max),
        )
    }
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random(-1., 1.);
            if p.squared() < 1. {
                return p;
            };
        }
    }
    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }
    pub fn random_in_hemisphere(normal: Self) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if dot(in_unit_sphere, normal) > 0. {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }
    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Self::from(random_f64(-1., 1.), random_f64(-1., 1.), 0.);
            if p.squared() < 1. {
                return p;
            };
        }
    }

    pub fn near_zero(&self) -> bool {
        let s: f64 = 1e-8;
        (self.e[0].abs() < s) && (self.e[1].abs() < s) && (self.e[2].abs() < s)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e: [
                self.e[0] + other[0],
                self.e[1] + other[1],
                self.e[2] + other[2],
            ],
        };
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        Self {
            e: [
                self.e[0] * (1. / other),
                self.e[1] * (1. / other),
                self.e[2] * (1. / other),
            ],
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self *= 1. / other
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        &self.e[i]
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            e: [self.e[0] * other, self.e[1] * other, self.e[2] * other],
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {
            e: [self.e[0] * other, self.e[1] * other, self.e[2] * other],
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(), |acc, v| {
            Self::from(acc.x() + v[0], acc.y() + v[1], acc.z() + v[2])
        })
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::from(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * (2. * dot(v, n))
}
pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(-uv, n).min(1.);
    let r_out_perp = (n * cos_theta + uv) * etai_over_etat;
    let r_out_parallel = n * -((1.0 - r_out_perp.squared()).abs()).sqrt();

    r_out_perp + r_out_parallel
}

pub type Point3 = Vec3;
pub type Colour = Vec3;
