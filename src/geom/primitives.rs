use std::clone::Clone;
use std::f64::consts;
use std::sync::{ Arc };

use super::hit::{
    AxisAlignedBoundingBox,
    Bounded,
    BoundedHittable,
    Hit,
    Hittable
};

use crate::math;
use crate::material::Material;
use crate::vec::{ Point3, Ray, Vec3 };

#[derive(Debug)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Material>
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Sphere {
        Sphere { center, radius, material }
    }
}

impl BoundedHittable for Sphere {}

impl Hittable for Sphere {
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
            return Some(Hit::new(ray.at(t1), normal, t1, outer, self.material.clone()));
        } else if t2 < t_max && t2 > t_min {
            let normal = (ray.at(t2) - &self.center) * (1.0/self.radius);
            let outer = ray.dir.dot(&normal) < 0.0;
            return Some(Hit::new(ray.at(t2), normal, t2, outer, self.material.clone()));
        }

        None
    }

    fn surface_area(&self) -> f64 {
        4.0 * consts::PI * self.radius.powi(2)
    }
}

impl Bounded for Sphere {
    fn bounding_box(&self) -> AxisAlignedBoundingBox {
        let ftr_corner = &self.center + Point3::new(self.radius, self.radius, self.radius);
        let bbl_corner = &self.center - Point3::new(self.radius, self.radius, self.radius);
        AxisAlignedBoundingBox { ftr_corner, bbl_corner, center: self.center.clone() }
    }
}

#[derive(Debug)]
pub struct Plane {
    center: Point3,
    spanning_vecs: (Vec3, Vec3),
    material: Arc<dyn Material>
}

impl Plane {
    pub fn new(center: Point3, spanning_vecs: (Vec3, Vec3), material: Arc<dyn Material>) -> Plane {
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

impl BoundedHittable for Plane {}

impl Hittable for Plane {
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
            Some(Hit::new(ray.at(t), normal, t, outer, self.material.clone()))
        } else {
            None
        }
    }

    fn surface_area(&self) -> f64 {
        2.0 * self.spanning_vecs.0.norm() + 2.0 * self.spanning_vecs.1.norm()
    }
}

impl Bounded for Plane {
    fn bounding_box(&self) -> AxisAlignedBoundingBox {
        let ftr_corner = &self.center + &self.spanning_vecs.0 + &self.spanning_vecs.1;
        let bbl_corner = &self.center - &self.spanning_vecs.0 - &self.spanning_vecs.1;
        AxisAlignedBoundingBox { ftr_corner, bbl_corner, center: self.center.clone() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::material::DiffuseLambert;
    use super::super::super::vec::colors;

    #[test]
    fn sphere_hit() {
        let mat_dif_white = DiffuseLambert::new(colors::WHITE);
        let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Arc::new(mat_dif_white));
        let ray = Ray::new(&Vec3::O, &-Vec3::K);
        assert!(sphere.is_hit(&ray, 0.0, f64::INFINITY).is_some(),
            "Ray should have hit sphere but didn't.")
    }

    #[test]
    fn sphere_miss() {
        let mat_dif_white = DiffuseLambert::new(colors::WHITE);
        let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Arc::new(mat_dif_white));
        let ray = Ray::new(&Vec3::O, &Vec3::J);
        assert!(sphere.is_hit(&ray, 0.0, f64::INFINITY).is_none(),
            "Ray shouldn't have hit sphere but did.")
    }

    #[test]
    fn sphere_inside_hit() {
        let mat_dif_white = DiffuseLambert::new(colors::WHITE);
        let sphere = Sphere::new(Point3::new(0.0, 0.0, -0.3), 0.5, Arc::new(mat_dif_white));
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
            Arc::new(mat_dif_white)
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
            Arc::new(mat_dif_white)
        );
        let ray = Ray::new(&Vec3::O, &(Vec3::J + Vec3::I));
        assert!(plane.is_hit(&ray, 0.0, f64::INFINITY).is_none(),
            "Ray shouldn't have hit plane but did.")
    }
}
