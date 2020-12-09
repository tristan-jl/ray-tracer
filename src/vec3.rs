use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub};

#[derive(Copy, Clone)]
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

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            e: [
                self.e[0] * (1. / rhs),
                self.e[1] * (1. / rhs),
                self.e[2] * (1. / rhs),
            ],
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1. / rhs
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

    fn mul(self, rhs: f64) -> Self {
        Self {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
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

pub type Point3 = Vec3;
pub type Colour = Vec3;
