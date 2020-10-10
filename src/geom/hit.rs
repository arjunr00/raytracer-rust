use std::sync::Arc;

use crate::accel::bvh::BVH;
use crate::material::Material;
use crate::math;
use crate::vec::{ Coord, Point3, Vec3, Ray };

pub type HittableRefs = Vec<Arc<dyn BoundedHittable>>;

#[derive(Debug)]
pub struct Hit {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub outer: bool,
    pub material: Arc<dyn Material>
}

impl Hit {
    pub const FP_OFFSET: f64 = 0.001;

    pub fn new(point: Point3, normal: Vec3, t: f64, outer: bool, material: Arc<dyn Material>) -> Hit {
        Hit {
            point, t, outer, material,
            normal: if outer { normal.unit() } else { -normal.unit() }
        }
    }
}

#[derive(Clone, Debug)]
pub struct AxisAlignedBoundingBox {
    center: Point3,
    ftr_corner: Point3,
    bbl_corner: Point3
}

impl AxisAlignedBoundingBox {
    pub fn new(ftr_corner: Point3, bbl_corner: Point3) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox {
            center: 0.5 * &bbl_corner + 0.5 * &ftr_corner,
            ftr_corner, bbl_corner
        }
    }

    pub fn empty() -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox::new(
            Point3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY),
            Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY)
        )
    }

    pub fn center(&self) -> &Point3 {
        &self.center
    }

    pub fn surface_area(&self) -> f64 {
        let size = &self.ftr_corner - &self.bbl_corner;
        2.0 * size[Coord::X] * size[Coord::Y]
        + 2.0 * size[Coord::Y] * size[Coord::Z]
        + 2.0 * size[Coord::X] * size[Coord::Z]
    }

    pub fn volume(&self) -> f64 {
        let size = &self.ftr_corner - &self.bbl_corner;
        size[Coord::X] * size[Coord::Y] * size[Coord::Z]
    }

    pub fn point_offset(&self, point: &Point3) -> Vec3 {
        let mut offset = point - &self.bbl_corner;
        for &coord in [ Coord::X, Coord::Y, Coord::Z ].iter() {
            if self.ftr_corner[coord] > self.bbl_corner[coord] {
                offset[coord] /= self.ftr_corner[coord] - self.bbl_corner[coord];
            }
        }

        offset
    }

    pub fn box_intersects(&self, other: AxisAlignedBoundingBox) -> bool {
        for &coord in [ Coord::X, Coord::Y, Coord::Z ].iter() {
            if !(math::f_leq(self.bbl_corner[coord], other.ftr_corner[coord])
                    && math::f_leq(other.bbl_corner[coord], self.ftr_corner[coord])) {
                return false;
            }
        }

        true
    }

    pub fn ray_intersects(&self, ray: &Ray, dir_inverse: &Vec3, mut t_min: f64, mut t_max: f64) -> Option<(f64, f64)> {
        let t0_vec = dir_inverse * (&self.bbl_corner - &ray.origin);
        let t1_vec = dir_inverse * (&self.ftr_corner - &ray.origin);
        for &coord in [ Coord::X, Coord::Y, Coord::Z ].iter() {
            let mut t0 = t0_vec[coord];
            let mut t1 = t1_vec[coord];

            if dir_inverse[coord] < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };

            if math::f_leq(t_max, t_min) {
                return None;
            }
        }

        Some((t_min, t_max))
    }

    pub fn union(boxes: Vec<&Self>) -> AxisAlignedBoundingBox {
        if boxes.len() == 0 {
            return AxisAlignedBoundingBox::new(Point3::O, Point3::O);
        }

        let mut ftr_corner = Point3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);
        let mut bbl_corner = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        for obj_box in boxes {
            ftr_corner[Coord::X] = f64::max(ftr_corner[Coord::X], obj_box.ftr_corner[Coord::X]);
            ftr_corner[Coord::Y] = f64::max(ftr_corner[Coord::Y], obj_box.ftr_corner[Coord::Y]);
            ftr_corner[Coord::Z] = f64::max(ftr_corner[Coord::Z], obj_box.ftr_corner[Coord::Z]);

            bbl_corner[Coord::X] = f64::min(bbl_corner[Coord::X], obj_box.bbl_corner[Coord::X]);
            bbl_corner[Coord::Y] = f64::min(bbl_corner[Coord::Y], obj_box.bbl_corner[Coord::Y]);
            bbl_corner[Coord::Z] = f64::min(bbl_corner[Coord::Z], obj_box.bbl_corner[Coord::Z]);
        }

        AxisAlignedBoundingBox::new(ftr_corner, bbl_corner)
    }

    pub fn union_from_objs(objs: &HittableRefs) -> AxisAlignedBoundingBox {
        let bounding_boxes: Vec<_> = objs.iter().map(|obj| obj.bounding_box()).collect();
        AxisAlignedBoundingBox::union(bounding_boxes.iter().collect())
    }

    pub fn union_from_points(points: &Vec<Point3>) -> AxisAlignedBoundingBox {
        let points_boxes: Vec<_> = points.iter()
            .map(|point| AxisAlignedBoundingBox::new(point.clone(), point.clone())).collect();
        AxisAlignedBoundingBox::union(points_boxes.iter().collect())
    }

    pub fn largest_extent_axis(&self) -> Coord {
        let extent = &self.ftr_corner - &self.bbl_corner;

        if math::f_geq(extent[Coord::X], f64::max(extent[Coord::Y], extent[Coord::Z])) {
            Coord::X
        } else if math::f_geq(extent[Coord::Y], extent[Coord::Z]) {
            Coord::Y
        } else {
            Coord::Z
        }
    }
}

pub trait Bounded {
    fn bounding_box(&self) -> AxisAlignedBoundingBox;
    fn centroid(&self) -> Point3 {
        self.bounding_box().center().clone()
    }
}

pub trait Hittable {
    fn is_hit(&self, ray: &Ray, t_min: f64, t_max: f64, rand: &mut math::Rand) -> Option<Hit>;
    fn surface_area(&self) -> f64 { 0.0 }
}

pub trait BoundedHittable: Bounded + Hittable + Send + Sync + std::fmt::Debug {}

#[derive(Debug)]
pub struct HittableGroup {
    accel: BVH
}

impl HittableGroup {
    pub fn new(hittables: HittableRefs) -> HittableGroup {
        let accel = BVH::new(hittables);
        HittableGroup { accel }
    }

    pub fn hittables(&self) -> &HittableRefs {
        self.accel.objects()
    }
}

impl Hittable for HittableGroup {
    fn is_hit(&self, ray: &Ray, t_min: f64, t_max: f64, rand: &mut math::Rand) -> Option<Hit> {
        self.accel.is_hit(ray, t_min, t_max, rand)
    }

    fn surface_area(&self) -> f64 {
        self.accel.objects().iter().fold(0.0, |acc, obj| acc + obj.surface_area())
    }
}

impl Bounded for HittableGroup {
    fn bounding_box(&self) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox::union_from_objs(self.hittables())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::f_eq;

    #[test]
    fn box_surface_area() {
        let bound = AxisAlignedBoundingBox::new(
            Point3::O, Point3::new(-1.0, -2.3, -4.2)
        );
        assert!(f_eq(bound.surface_area(), 32.32));
    }

    #[test]
    fn box_volume() {
        let bound = AxisAlignedBoundingBox::new(
            Point3::O, Point3::new(-1.0, -2.3, -4.2)
        );
        assert!(f_eq(bound.volume(), 9.66));
    }

    #[test]
    fn boxes_overlap() {
        let bounds = (
            AxisAlignedBoundingBox::new(
                Point3::O, Point3::new(-1.0, -2.3, -4.2)
            ),
            AxisAlignedBoundingBox::new(
                Point3::new(0.5, 1.0, 0.2), Point3::new(-0.5, -3.1, -1.0)
            )
        );

        assert!(bounds.0.box_intersects(bounds.1));
    }

    #[test]
    fn boxes_no_overlap() {
        let bounds = (
            AxisAlignedBoundingBox::new(
                Point3::O, Point3::new(-1.0, -2.3, -4.2)
            ),
            AxisAlignedBoundingBox::new(
                Point3::new(1.0, 2.3, 4.2), Point3::new(0.1, 1.0, 1.0)
            )
        );

        assert!(!bounds.0.box_intersects(bounds.1));
    }

    #[test]
    fn ray_intersects_box() {
        let bound = AxisAlignedBoundingBox::new(
            Point3::O, Point3::new(-1.0, -2.3, -4.2)
        );
        let ray = Ray::new(&Point3::new(1.0, 2.0, 2.0), &Vec3::new(-1.0, -2.3, -4.2));
        let dir_inverse = Vec3::new(
            1.0 / ray.dir[Coord::X],
            1.0 / ray.dir[Coord::Y],
            1.0 / ray.dir[Coord::Z]
        );

        assert!(bound.ray_intersects(&ray, &dir_inverse, 0.01, f64::INFINITY).is_some());
    }

    #[test]
    fn ray_no_intersects_box() {
        let bound = AxisAlignedBoundingBox::new(
            Point3::O, Point3::new(-1.0, -2.3, -4.2)
        );
        let ray = Ray::new(&Point3::new(3.0, 2.0, 4.0), &Vec3::new(0.8, 3.0, 4.0));
        let dir_inverse = Vec3::new(
            1.0 / ray.dir[Coord::X],
            1.0 / ray.dir[Coord::Y],
            1.0 / ray.dir[Coord::Z]
        );

        assert!(bound.ray_intersects(&ray, &dir_inverse, 0.01, f64::INFINITY).is_none());
    }
}
