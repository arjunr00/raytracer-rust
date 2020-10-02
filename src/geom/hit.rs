use std::sync::Arc;

use crate::material::Material;
use crate::math;
use crate::vec::{ Coord, Point3, Vec3, Ray };

pub struct Hit {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub outer: bool,
    pub material: Arc<dyn Material>
}

impl Hit {
    pub fn new(point: Point3, normal: Vec3, t: f64, outer: bool, material: Arc<dyn Material>) -> Hit {
        Hit {
            point, t, outer, material,
            normal: if outer { normal.unit() } else { -normal.unit() }
        }
    }
}

#[derive(Debug)]
pub struct AxisAlignedBoundingBox {
    pub center: Point3,
    pub ftr_corner: Point3,
    pub bbl_corner: Point3
}

impl AxisAlignedBoundingBox {
    pub fn box_intersects(&self, other: AxisAlignedBoundingBox) -> bool {
        for &coord in [ Coord::X, Coord::Y, Coord::Z ].iter() {
            if !(math::f_leq(self.bbl_corner[coord], other.ftr_corner[coord])
                    && math::f_leq(other.bbl_corner[coord], self.ftr_corner[coord])) {
                return false;
            }
        }

        true
    }

    pub fn ray_intersects(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for &coord in [ Coord::X, Coord::Y, Coord::Z ].iter() {
            let dir_inverse = 1.0 / ray.dir[coord];
            let mut t0 = dir_inverse * (self.bbl_corner[coord] - ray.origin[coord]);
            let mut t1 = dir_inverse * (self.ftr_corner[coord] - ray.origin[coord]);

            if dir_inverse < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };

            if math::f_leq(t_max, t_min) {
                return false;
            }
        }

        true
    }
}

pub trait Bounded {
    fn bounding_box(&self) -> AxisAlignedBoundingBox;
}

pub trait Hittable {
    fn is_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
    fn surface_area(&self) -> f64;
}

pub trait BoundedHittable: Bounded + Hittable + Send + Sync + std::fmt::Debug {}

#[derive(Debug)]
pub struct HittableGroup {
    hittables: Vec<Arc<dyn BoundedHittable>>
}

impl HittableGroup {
    pub fn new(hittables: Vec<Arc<dyn BoundedHittable>>) -> HittableGroup {
        HittableGroup { hittables }
    }

    pub fn hittables(&self) -> &Vec<Arc<dyn BoundedHittable>> {
        &self.hittables
    }
}

impl Hittable for HittableGroup {
    fn is_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut hit: Option<Hit> = None;
        let mut closest_t = t_max;

        for obj in &self.hittables {
            if let Some(obj_hit) = obj.is_hit(ray, t_min, closest_t) {
                closest_t = obj_hit.t;
                hit = Some(obj_hit);
            }
        }

        hit
    }

    fn surface_area(&self) -> f64 {
        self.hittables.iter().fold(0.0, |acc, obj| acc + obj.surface_area())
    }
}

impl Bounded for HittableGroup {
    fn bounding_box(&self) -> AxisAlignedBoundingBox {
        self.hittables.bounding_box()
    }
}


impl Bounded for Vec<Arc<dyn BoundedHittable>> {
    fn bounding_box(&self) -> AxisAlignedBoundingBox {
        if self.len() == 0 {
            return AxisAlignedBoundingBox { ftr_corner: Point3::O, bbl_corner: Point3::O, center: Point3::O };
        }

        let mut ftr_corner = Point3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);
        let mut bbl_corner = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        for obj in self {
            let obj_box = obj.bounding_box();

            ftr_corner[Coord::X] = f64::max(ftr_corner[Coord::X], obj_box.ftr_corner[Coord::X]);
            ftr_corner[Coord::Y] = f64::max(ftr_corner[Coord::Y], obj_box.ftr_corner[Coord::Y]);
            ftr_corner[Coord::Z] = f64::max(ftr_corner[Coord::Z], obj_box.ftr_corner[Coord::Z]);

            bbl_corner[Coord::X] = f64::min(bbl_corner[Coord::X], obj_box.bbl_corner[Coord::X]);
            bbl_corner[Coord::Y] = f64::min(bbl_corner[Coord::Y], obj_box.bbl_corner[Coord::Y]);
            bbl_corner[Coord::Z] = f64::min(bbl_corner[Coord::Z], obj_box.bbl_corner[Coord::Z]);
        }

        let center = 0.5 * (&ftr_corner + &bbl_corner);
        AxisAlignedBoundingBox { ftr_corner, bbl_corner, center }
    }
}
