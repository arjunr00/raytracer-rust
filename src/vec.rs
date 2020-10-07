use rand::distributions::Distribution;
use std::{ ops::{self}, clone, cmp, convert, fmt };

use super::math;
use super::geom::{
    World,
    hit::Hittable,
};

/// A struct for a 3-dimensional floating-point vector
#[derive(Debug)]
pub struct Vec3 (f64, f64, f64);
// Useful aliases
pub use Vec3 as ColorRGB;
pub use Vec3 as Point3;

#[derive(Debug)]
pub struct Quaternion(f64, Vec3);

/// Describes a ray of the form r(t) = origin + t*dir
#[derive(Debug)]
pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3
}

#[derive(Debug)]
pub enum Color {
    R, G, B
}

#[derive(Clone, Copy, Debug)]
pub enum Coord {
    X, Y, Z
}

impl Ray {
    pub fn new(origin: &Point3, dir: &Vec3) -> Ray {
        Ray {
            origin: Point3::new(
                        origin[Coord::X], origin[Coord::Y], origin[Coord::Z]
                    ),
            dir: dir.unit()
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        &self.origin + &(t * &self.dir)
    }

    pub fn get_color(&self, world: &World, bg: &dyn Fn(f64) -> ColorRGB, depth: u32, rand: &mut math::Rand)
        -> ColorRGB
    {
        let mut color = colors::WHITE;
        let mut ray: Ray = self.clone();

        for _ in 0..depth {
            match world.is_hit(&ray, 0.001, f64::INFINITY) {
                None => {
                    let t = 0.5 * (1.0 - ray.dir[Coord::Y]);
                    color *= bg(t);
                    break;
                },
                Some(hit) => {
                    match hit.material.scatter(&ray, &hit, rand) {
                        None => {
                            color *= hit.material.attenuation() * hit.material.emit();
                            break;
                        },
                        Some(scattered) => {
                            ray = scattered;
                            color *= hit.material.attenuation();
                            color += hit.material.emit();
                        }
                    }
                }
            }
        }

        color
    }
}

impl Vec3 {
    pub const I: Vec3 = Vec3 ( 1.0, 0.0, 0.0 );
    pub const J: Vec3 = Vec3 ( 0.0, 1.0, 0.0 );
    pub const K: Vec3 = Vec3 ( 0.0, 0.0, 1.0 );
    pub const O: Vec3 = Vec3 ( 0.0, 0.0, 0.0 );

    pub fn new(e1: f64, e2: f64, e3: f64) -> Vec3 {
        Vec3(e1, e2, e3)
    }

    pub fn orthogonal(v: &Vec3, w: &Vec3) -> bool {
        math::f_eq(v.dot(w), 0.0)
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
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
        let norm = self.norm();
        if math::f_eq(norm, 0.0) {
            return Vec3::O;
        }

        (1.0/self.norm()) * self
    }

    pub fn projections(&self, onto: &Vec3, along: &Vec3) -> (Vec3, Vec3) {
        let onto_unit = onto.unit();
        let along_unit = along.unit();

        let self_parallel = (self.dot(&onto_unit)) * &onto_unit;
        let self_perp = self - &self_parallel;

        if Vec3::orthogonal(&onto_unit, &along_unit) {
            return (self_parallel, self_perp);
        }

        let cos_phi = onto_unit.dot(&along_unit);
        let tan_phi = cos_phi.acos().tan();

        (self_parallel - (self_perp.norm() / tan_phi) * onto_unit,
        ((self_perp.norm() / tan_phi).powi(2) + self_perp.norm().powi(2)).sqrt() * along_unit)
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        self - 2.0 * self.dot(normal) * normal
    }

    pub fn refract(&self, normal: &Vec3, index_i: f64, index_r: f64, rand: &mut math::Rand) -> Vec3 {
        let self_unit = self.unit();
        let sin_theta_i = (self_unit.cross(normal)).norm();
        let cos_theta_i = f64::abs(self_unit.dot(normal));
        // Total internal reflection or Fresnel reflection
        if index_i >= index_r && sin_theta_i > index_r/index_i
            || rand.dist.sample(&mut rand.rng) < math::schlick(cos_theta_i, index_i, index_r)
        {
            return self.reflect(normal);
        }

        let theta_r = (sin_theta_i * index_i / index_r).asin();
        let refracted_perp = theta_r.tan() * self.cross(&-normal).cross(&normal).unit();
        -normal + refracted_perp
    }

    pub fn random_unit(rand: &mut math::Rand) -> Vec3 {
        // Under the assumption that the input dist is 0 to 1
        let x = 2.0 * rand.dist.sample(&mut rand.rng) - 1.0;
        let y = 2.0 * rand.dist.sample(&mut rand.rng) - 1.0;
        let z = 2.0 * rand.dist.sample(&mut rand.rng) - 1.0;
        Vec3::new(x, y, z).unit()
    }

    pub fn random_in_unit_disc(rand: &mut math::Rand) -> Vec3 {
        // Under the assumption that the input dist is 0 to 1
        let x = 2.0 * rand.dist.sample(&mut rand.rng) - 1.0;
        let y = 2.0 * rand.dist.sample(&mut rand.rng) - 1.0;
        Vec3::new(x, y, 0.0).unit()
    }

    pub fn rotate(&self, angle: f64, axis: &Vec3) -> Vec3 {
        if math::f_eq(0.0, angle) { return self.clone(); }

        let rot = Quaternion((angle * 0.5).cos(), (angle * 0.5).sin() * axis.unit());
        let new_vec = &rot * Quaternion::from(self.clone()) * &rot.conj();
        new_vec.1
    }
}

impl Quaternion {
    pub fn new(angle: f64, axis: &Vec3) -> Quaternion {
        Quaternion(angle, axis.clone())
    }

    pub fn angle(&self) -> f64 {
        self.0
    }

    pub fn axis(&self) -> &Vec3 {
        &self.1
    }

    pub fn conj(&self) -> Quaternion {
        Quaternion(self.0, -&self.1)
    }
}

impl convert::From<Vec3> for Quaternion {
    fn from(vec: Vec3) -> Quaternion {
        Quaternion(0.0, vec)
    }
}

impl clone::Clone for Ray {
    fn clone(&self) -> Ray {
        Ray { origin: self.origin.clone(), dir: self.dir.clone() }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\u{27E8}{:.2}, {:.2}, {:.2}\u{27E9}", self[Coord::X], self[Coord::Y], self[Coord::Z])
    }
}

impl clone::Clone for Vec3 {
    fn clone(&self) -> Vec3 {
        Vec3::new(self.0, self.1, self.2)
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

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3::new(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl ops::AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, other: &Vec3) {
        *self = Vec3::new(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl ops::Mul<Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl ops::Mul<&Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Vec3 {
        Vec3::new(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl ops::Mul<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Vec3 {
        Vec3::new(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        *self = Vec3::new(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl ops::MulAssign<&Vec3> for Vec3 {
    fn mul_assign(&mut self, other: &Vec3) {
        *self = Vec3::new(self.0 * other.0, self.1 * other.1, self.2 * other.2)
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

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        self * -1.0
    }
}

impl ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        self * -1.0
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

impl fmt::Display for Quaternion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\u{27E8}{:.2}, {:.2}, {:.2}, {:.2}\u{27E9}",
            self.0, self.1[Coord::X], self.1[Coord::Y], self.1[Coord::Z])
    }
}

impl ops::Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, other: Quaternion) -> Quaternion {
        let a = (self.0, other.0);
        let b = ((self.1).0, (other.1).0);
        let c = ((self.1).1, (other.1).1);
        let d = ((self.1).2, (other.1).2);

        let w = a.0 * a.1 - b.0 * b.1 - c.0 * c.1 - d.0 * d.1;
        let i = a.0 * b.1 + b.0 * a.1 + c.0 * d.1 - d.0 * c.1;
        let j = a.0 * c.1 - b.0 * d.1 + c.0 * a.1 + d.0 * b.1;
        let k = a.0 * d.1 + b.0 * c.1 - c.0 * b.1 + d.0 * a.1;

        Quaternion(w, Vec3::new(i, j, k))
    }
}

impl ops::Mul<&Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, other: &Quaternion) -> Quaternion {
        let a = (self.0, other.0);
        let b = ((self.1).0, (other.1).0);
        let c = ((self.1).1, (other.1).1);
        let d = ((self.1).2, (other.1).2);

        let w = a.0 * a.1 - b.0 * b.1 - c.0 * c.1 - d.0 * d.1;
        let i = a.0 * b.1 + b.0 * a.1 + c.0 * d.1 - d.0 * c.1;
        let j = a.0 * c.1 - b.0 * d.1 + c.0 * a.1 + d.0 * b.1;
        let k = a.0 * d.1 + b.0 * c.1 - c.0 * b.1 + d.0 * a.1;

        Quaternion(w, Vec3::new(i, j, k))
    }
}

impl ops::Mul<Quaternion> for &Quaternion {
    type Output = Quaternion;

    fn mul(self, other: Quaternion) -> Quaternion {
        let a = (self.0, other.0);
        let b = ((self.1).0, (other.1).0);
        let c = ((self.1).1, (other.1).1);
        let d = ((self.1).2, (other.1).2);

        let w = a.0 * a.1 - b.0 * b.1 - c.0 * c.1 - d.0 * d.1;
        let i = a.0 * b.1 + b.0 * a.1 + c.0 * d.1 - d.0 * c.1;
        let j = a.0 * c.1 - b.0 * d.1 + c.0 * a.1 + d.0 * b.1;
        let k = a.0 * d.1 + b.0 * c.1 - c.0 * b.1 + d.0 * a.1;

        Quaternion(w, Vec3::new(i, j, k))
    }
}

impl ops::Mul<&Quaternion> for &Quaternion {
    type Output = Quaternion;

    fn mul(self, other: &Quaternion) -> Quaternion {
        let a = (self.0, other.0);
        let b = ((self.1).0, (other.1).0);
        let c = ((self.1).1, (other.1).1);
        let d = ((self.1).2, (other.1).2);

        let w = a.0 * a.1 - b.0 * b.1 - c.0 * c.1 - d.0 * d.1;
        let i = a.0 * b.1 + b.0 * a.1 + c.0 * d.1 - d.0 * c.1;
        let j = a.0 * c.1 - b.0 * d.1 + c.0 * a.1 + d.0 * b.1;
        let k = a.0 * d.1 + b.0 * c.1 - c.0 * b.1 + d.0 * a.1;

        Quaternion(w, Vec3::new(i, j, k))
    }
}

pub mod colors {
    use super::ColorRGB;

    pub const BLACK: ColorRGB = ColorRGB(0.0, 0.0, 0.0);
    pub const WHITE: ColorRGB = ColorRGB(1.0, 1.0, 1.0);
    pub const SKYBLUE: ColorRGB = ColorRGB(0.5, 0.7, 1.0);
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

    #[test]
    fn ray_at_t() {
        let ray = Ray::new(&Point3::new(0.0, 1.0, 2.0), &Vec3::new(3.0, 4.0, 0.0));
        assert_eq!(ray.at(5.0), Vec3::new(3.0, 5.0, 2.0));
    }

    #[test]
    fn project_vec() {
        let vector = Vec3::new(1.0, 1.0, 0.0);
        let onto = Vec3::I;
        let along = Vec3::J;

        assert_eq!(vector.projections(&onto, &along), (Vec3::I, Vec3::J));
    }

    #[test]
    fn reflect_vec() {
        let incident = Vec3::I - Vec3::J;
        let normal = Vec3::J;
        assert_eq!(incident.reflect(&normal), Vec3::I + Vec3::J);
    }

    #[test]
    fn reflect_normal() {
        let incident = Vec3::I;
        let normal = -Vec3::I;
        assert_eq!(incident.reflect(&normal), -Vec3::I);
    }

    #[test]
    fn refract_vec() {
        let constant = rand::distributions::Uniform::from(0.9..1.0);
        let rng = rand::thread_rng();
        let mut rand = math::Rand { dist: constant, rng };

        let normal = Vec3::J;
        let incident = Vec3::new(1.0, -1.0, 0.0);
        let refracted = incident.refract(&normal, 1.0, 1.5, &mut rand); 
        assert!(math::f_eq(refracted.unit().dot(&-normal).acos(),
            (std::f64::consts::SQRT_2 / 3.0).asin()));
    }

    #[test]
    fn refract_another_vec() {
        let constant = rand::distributions::Uniform::from(0.9..1.0);
        let rng = rand::thread_rng();
        let mut rand = math::Rand { dist: constant, rng };

        let normal = Vec3::new(0.5, 0.4, 0.3).unit();
        let incident = Vec3::new(0.7, 0.9, 1.0);
        let refracted = incident.refract(&normal, 1.0, 1.52, &mut rand); 
        let expected_angle = ((1.01 * (2.0_f64 / 2.3_f64).sqrt()).acos().sin() / 1.52).asin();

        assert!(math::f_eq(refracted.unit().dot(&-normal).acos(), expected_angle));
    }

    #[test]
    fn total_internal_reflection() {
        let constant = rand::distributions::Uniform::from(0.9..1.0);
        let rng = rand::thread_rng();
        let mut rand = math::Rand { dist: constant, rng };

        let normal = Vec3::J;
        let incident = Vec3::new(1.0, -1.0, 0.0);
        let refracted = incident.refract(&normal, 1.5, 1.0, &mut rand); 
        assert_eq!(refracted, incident.reflect(&normal));
    }

    #[test]
    fn schlick() {
        let constant = rand::distributions::Uniform::from(0.0..1.0);
        let rng = rand::thread_rng();
        let mut rand = math::Rand { dist: constant, rng };

        let normal = Vec3::J;
        let incident = Vec3::I;
        let refracted = incident.refract(&normal, 1.0, 1.5, &mut rand);
        assert_eq!(refracted, incident.reflect(&normal));
    }
}
