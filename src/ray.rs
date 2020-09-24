use super::math;
use super::vec::{ Vec3, Point3, ColorRGB, Coord };

/// Describes a ray of the form r(t) = origin + t*dir
#[derive(Debug)]
pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3
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

    pub fn get_color(&self) -> ColorRGB {
        let t: f64 = 0.5 * (1.0 - self.dir.unit()[Coord::Y]);
        let white = ColorRGB::new(1.0, 1.0, 1.0);
        let skyblue = ColorRGB::new(0.5, 0.7, 1.0);
        math::lerp(skyblue, white, t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_at_t() {
        let ray = Ray::new(&Point3::new(0.0, 1.0, 2.0), &Vec3::new(3.0, 4.0, 0.0));
        assert_eq!(ray.at(5.0), Vec3::new(3.0, 5.0, 2.0));
    }
}
