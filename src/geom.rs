use std::clone::Clone;
use std::f64::consts;

use super::math;
use super::material::Material;
use super::vec::{ Coord, Point3, Ray, Vec3 };

pub struct Hit<'a> {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub outer: bool,
    pub material: &'a dyn Material
}

impl Hit<'_> {
    pub fn new(point: Point3, normal: Vec3, t: f64, outer: bool, material: &dyn Material) -> Hit {
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

pub trait BoundedHittable: Bounded + Hittable + std::fmt::Debug {}

pub struct HittableGroup<'a> {
    hittables: Vec<&'a dyn BoundedHittable>
}

impl HittableGroup<'_> {
    pub fn new(hittables: Vec<&dyn BoundedHittable>) -> HittableGroup {
        HittableGroup { hittables }
    }
}

impl Hittable for HittableGroup<'_> {
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

impl Bounded for Vec<&dyn BoundedHittable> {
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

impl Bounded for HittableGroup<'_> {
    fn bounding_box(&self) -> AxisAlignedBoundingBox {
        self.hittables.bounding_box()
    }
}

#[derive(Debug)]
pub struct Sphere<'a> {
    center: Point3,
    radius: f64,
    material: &'a dyn Material
}

impl Sphere<'_> {
    pub fn new(center: Point3, radius: f64, material: &dyn Material) -> Sphere {
        Sphere { center, radius, material }
    }
}

impl BoundedHittable for Sphere<'_> {}

impl Hittable for Sphere<'_> {
    fn is_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let vec_to_center = &ray.origin - &self.center;
        let center_dot_self = ray.dir.dot(&vec_to_center);
        let discriminant =
            center_dot_self.powi(2)
            - (vec_to_center.norm().powi(2) - self.radius.powi(2));

        if discriminant < 0.0 { return None; }

        let root = discriminant.sqrt();
        let t1 = -center_dot_self - root;
        let t2 = -center_dot_self + root;

        if t1 < t_max && t1 > t_min {
            let normal = (ray.at(t1) - &self.center) * (1.0/self.radius);
            let outer = ray.dir.dot(&normal) < 0.0;
            return Some(Hit::new(ray.at(t1), normal, t1, outer, self.material));
        } else if t2 < t_max && t2 > t_min {
            let normal = (ray.at(t2) - &self.center) * (1.0/self.radius);
            let outer = ray.dir.dot(&normal) < 0.0;
            return Some(Hit::new(ray.at(t2), normal, t2, outer, self.material));
        }

        None
    }

    fn surface_area(&self) -> f64 {
        4.0 * consts::PI * self.radius.powi(2)
    }
}

impl Bounded for Sphere<'_> {
    fn bounding_box(&self) -> AxisAlignedBoundingBox {
        let ftr_corner = &self.center + Point3::new(self.radius, self.radius, self.radius);
        let bbl_corner = &self.center - Point3::new(self.radius, self.radius, self.radius);
        AxisAlignedBoundingBox { ftr_corner, bbl_corner, center: self.center.clone() }
    }
}

#[derive(Debug)]
pub struct Plane<'a> {
    center: Point3,
    spanning_vecs: (Vec3, Vec3),
    material: &'a dyn Material
}

impl Plane<'_> {
    pub fn new(center: Point3, spanning_vecs: (Vec3, Vec3), material: &dyn Material) -> Plane {
        let plane_i = spanning_vecs.0;
        let mut plane_j = spanning_vecs.1;

        if !Vec3::orthogonal(&plane_i, &plane_j) {
            let plane_k = plane_i.cross(&plane_j);
            let new_plane_j = plane_j.projections(&plane_i, &plane_k.cross(&plane_i)).1;
            eprintln!(
                "Warning: Plane centered at {} is not spanned by orthogonal vectors. \
                Second spanning vector {} has been projected to {}.",
                center, plane_j, new_plane_j
            );
            plane_j = new_plane_j;
        }

        Plane { center, spanning_vecs: (plane_i, plane_j), material }
    }
}

impl BoundedHittable for Plane<'_> {}

impl Hittable for Plane<'_> {
    fn is_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let plane_i = &self.spanning_vecs.0;
        let plane_j = &self.spanning_vecs.1;
        let normal = (plane_i.cross(plane_j)).unit();
        if Vec3::orthogonal(&ray.dir, &normal) { return None; }

        let t = ((&self.center - &ray.origin).dot(&normal)) / (ray.dir.dot(&normal));
        let center_to_point = ray.at(t) - &self.center;
        let ctp_components = center_to_point.projections(&plane_i, &plane_j);

        if t < t_max && t > t_min
            && math::f_leq(ctp_components.0.norm(), plane_i.norm())
            && math::f_leq(ctp_components.1.norm(), plane_j.norm())
        {
            let outer = ray.dir.dot(&normal) < 0.0;
            Some(Hit::new(ray.at(t), normal, t, outer, self.material))
        } else {
            None
        }
    }

    fn surface_area(&self) -> f64 {
        2.0 * self.spanning_vecs.0.norm() + 2.0 * self.spanning_vecs.1.norm()
    }
}

impl Bounded for Plane<'_> {
    fn bounding_box(&self) -> AxisAlignedBoundingBox {
        let ftr_corner = &self.center + &self.spanning_vecs.0 + &self.spanning_vecs.1;
        let bbl_corner = &self.center - &self.spanning_vecs.0 - &self.spanning_vecs.1;
        AxisAlignedBoundingBox { ftr_corner, bbl_corner, center: self.center.clone() }
    }
}

#[derive(Debug)]
pub struct Prism<'a> {
    center: Point3,
    spanning_vecs: (Vec3, Vec3, Vec3),
    planes: [Plane<'a> ; 6]
}

impl Prism<'_> {
    pub fn new(center: Point3, spanning_vecs: (Vec3, Vec3, Vec3), material: &dyn Material)
        -> Prism
    {
        let prism_i = &spanning_vecs.0;
        let prism_j = &spanning_vecs.1;
        let prism_k = &spanning_vecs.2;

        let front_face = Plane::new(
            &center - prism_k, (prism_j.clone(), prism_i.clone()), material
        );
        let back_face = Plane::new(
            &center + prism_k, (prism_i.clone(), prism_j.clone()), material
        );
        let top_face = Plane::new(
            &center + prism_j, (prism_k.clone(), prism_i.clone()), material
        );
        let bottom_face = Plane::new(
            &center - prism_j, (prism_i.clone(), prism_k.clone()), material
        );
        let left_face = Plane::new(
            &center - prism_i, (prism_k.clone(), prism_j.clone()), material
        );
        let right_face = Plane::new(
            &center + prism_i, (prism_j.clone(), prism_k.clone()), material
        );

        Prism {
            center, spanning_vecs,
            planes: [ front_face, top_face, left_face, bottom_face, right_face, back_face ]
        }
    }
}

impl BoundedHittable for Prism<'_> {}

impl Hittable for Prism<'_> {
    fn is_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut hit: Option<Hit> = None;
        let mut closest_t = t_max;

        for obj in self.planes.iter() {
            if let Some(obj_hit) = obj.is_hit(ray, t_min, closest_t) {
                closest_t = obj_hit.t;
                hit = Some(obj_hit);
            }
        }

        hit
    }

    fn surface_area(&self) -> f64 {
        self.planes.iter().fold(0.0, |acc, plane| acc + plane.surface_area())
    }
}

impl Bounded for Prism<'_> {
    fn bounding_box(&self) -> AxisAlignedBoundingBox {
        let ftr_corner = &self.center + &self.spanning_vecs.0 + &self.spanning_vecs.1 + &self.spanning_vecs.2;
        let bbl_corner = &self.center - &self.spanning_vecs.0 - &self.spanning_vecs.1 - &self.spanning_vecs.2;
        AxisAlignedBoundingBox { ftr_corner, bbl_corner, center: self.center.clone() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::material::DiffuseLambert;
    use super::super::vec::colors;

    #[test]
    fn sphere_hit() {
        let mat_dif_white = DiffuseLambert::new(colors::WHITE);
        let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, &mat_dif_white);
        let ray = Ray::new(&Vec3::O, &-Vec3::K);
        assert!(sphere.is_hit(&ray, 0.0, f64::INFINITY).is_some(),
            "Ray should have hit sphere but didn't.")
    }

    #[test]
    fn sphere_miss() {
        let mat_dif_white = DiffuseLambert::new(colors::WHITE);
        let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, &mat_dif_white);
        let ray = Ray::new(&Vec3::O, &Vec3::J);
        assert!(sphere.is_hit(&ray, 0.0, f64::INFINITY).is_none(),
            "Ray shouldn't have hit sphere but did.")
    }

    #[test]
    fn sphere_inside_hit() {
        let mat_dif_white = DiffuseLambert::new(colors::WHITE);
        let sphere = Sphere::new(Point3::new(0.0, 0.0, -0.3), 0.5, &mat_dif_white);
        let ray = Ray::new(&Vec3::O, &-Vec3::K);
        assert!(sphere.is_hit(&ray, 0.0, f64::INFINITY).is_some(),
            "Ray should have hit sphere but didn't.")
    }

    #[test]
    fn plane_hit() {
        let mat_dif_white = DiffuseLambert::new(colors::WHITE);
        let plane = Plane::new(
            Point3::new(0.0, 1.0, 0.0),
            (Vec3::I, -Vec3::K),
            &mat_dif_white
        );
        let ray = Ray::new(&Vec3::O, &Vec3::J);
        assert!(plane.is_hit(&ray, 0.0, f64::INFINITY).is_some(),
            "Ray should have hit plane but didn't.")
    }

    #[test]
    fn plane_miss() {
        let mat_dif_white = DiffuseLambert::new(colors::WHITE);
        let plane = Plane::new(
            Point3::new(0.0, 1.0, 0.0),
            (0.5 * Vec3::I, -0.5 * Vec3::K),
            &mat_dif_white
        );
        let ray = Ray::new(&Vec3::O, &(Vec3::J + Vec3::I));
        assert!(plane.is_hit(&ray, 0.0, f64::INFINITY).is_none(),
            "Ray shouldn't have hit plane but did.")
    }
}
