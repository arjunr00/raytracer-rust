use std::{ ops::{self}, cmp };
use super::math;

/// A struct for a 3-dimensional floating-point vector
#[derive(Debug)]
pub struct Vec3 (f64, f64, f64);
// Useful aliases
pub use Vec3 as ColorRGB;
pub use Vec3 as Point3;

pub enum Color {
    R, G, B
}

pub enum Coord {
    X, Y, Z
}

impl Vec3 {
    pub fn new(e1: f64, e2: f64, e3: f64) -> Vec3 {
        Vec3(e1, e2, e3)
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0
        )
    }

    pub fn norm(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }

    pub fn unit(&self) -> Vec3 {
        (1.0/self.norm()) * self
    }
}

impl cmp::PartialEq for Vec3 {
    // Explicitly define equality to account for floating-point imprecision
    fn eq(&self, other: &Vec3) -> bool {
        math::f_eq(self.0, other.0)
        && math::f_eq(self.1, other.1)
        && math::f_eq(self.2, other.2)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        Vec3::new(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        Vec3::new(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3::new(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3::new(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3::new(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(other.0 * self, other.1 * self, other.2 * self)
    }
}

impl ops::Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Vec3 {
        Vec3::new(other.0 * self, other.1 * self, other.2 * self)
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Vec3::new(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        self + (-1.0 * other)
    }
}

impl ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        self + (-1.0 * other)
    }
}

impl ops::Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Vec3 {
        self + (-1.0 * other)
    }
}

impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Vec3 {
        self + &(-1.0 * other)
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self += -1.0 * &other;
    }
}

impl ops::Index<Color> for Vec3 {
    type Output = f64;

    fn index(&self, color: Color) -> &f64 {
        match color {
            Color::R => &self.0,
            Color::G => &self.1,
            Color::B => &self.2
        }
    }
}

impl ops::IndexMut<Color> for Vec3 {
    fn index_mut(&mut self, color: Color) -> &mut f64 {
        match color {
            Color::R => &mut self.0,
            Color::G => &mut self.1,
            Color::B => &mut self.2
        }
    }
}

impl ops::Index<Coord> for Vec3 {
    type Output = f64;

    fn index(&self, coord: Coord) -> &f64 {
        match coord {
            Coord::X => &self.0,
            Coord::Y => &self.1,
            Coord::Z => &self.2
        }
    }
}

impl ops::IndexMut<Coord> for Vec3 {
    fn index_mut(&mut self, coord: Coord) -> &mut f64 {
        match coord {
            Coord::X => &mut self.0,
            Coord::Y => &mut self.1,
            Coord::Z => &mut self.2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(3.0, 2.0, 1.0);
        let v3 = &v1 + &v2;
        assert_eq!(v3, Vec3::new(4.0, 4.0, 4.0));
    }

    #[test]
    fn add_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(3.0, 2.0, 1.0);
        v1 += v2;
        assert_eq!(v1, Vec3::new(4.0, 4.0, 4.0));
    }

    #[test]
    fn mul() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let k = 4.0;
        let v3 = &v1 * k;
        assert_eq!(v3, Vec3::new(4.0, 8.0, 12.0));
    }

    #[test]
    fn mul_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let k = 4.0;
        v1 *= k;
        assert_eq!(v1, Vec3::new(4.0, 8.0, 12.0));
    }

    #[test]
    fn sub() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(3.0, 2.0, 1.0);
        let v3 = &v1 - &v2;
        assert_eq!(v3, Vec3::new(-2.0, 0.0, 2.0));
    }

    #[test]
    fn sub_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(3.0, 2.0, 1.0);
        v1 -= v2;
        assert_eq!(v1, Vec3::new(-2.0, 0.0, 2.0));
    }

    #[test]
    fn norm() {
        let v = Vec3::new(1.0 / 3.0_f64.sqrt(), 1.0 / 3.0_f64.sqrt(), 1.0 / 3.0_f64.sqrt());
        let norm = v.norm();
        assert_eq!(norm, 1.0);
    }

    #[test]
    fn unit() {
        let v = Vec3::new(1.0, 1.0, 1.0);
        let unit = v.unit();
        assert_eq!(unit, Vec3::new(1.0 / 3.0_f64.sqrt(), 1.0 / 3.0_f64.sqrt(), 1.0 / 3.0_f64.sqrt()));
    }

    #[test]
    fn index_as_color() {
        let mut col = ColorRGB::new(1.0, 0.0, 2.0);
        assert_eq!(col[Color::R], 1.0);
        assert_eq!(col[Color::G], 0.0);
        assert_eq!(col[Color::B], 2.0);
        col[Color::G] = 3.0;
        assert_eq!(col[Color::G], 3.0);
    }

    #[test]
    fn index_as_point() {
        let mut pt = Point3::new(1.0, 0.0, 2.0);
        assert_eq!(pt[Coord::X], 1.0);
        assert_eq!(pt[Coord::Y], 0.0);
        assert_eq!(pt[Coord::Z], 2.0);
        pt[Coord::Y] = 3.0;
        assert_eq!(pt[Coord::Y], 3.0);
    }
}