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
use crate::vec::{ Coord, Point3, Ray, Vec3 };

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
        AxisAlignedBoundingBox::new(ftr_corner, bbl_corner)
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
        let spans = (
             &self.spanning_vecs.0 + &self.spanning_vecs.1,
             &self.spanning_vecs.0 - &self.spanning_vecs.1,
            -&self.spanning_vecs.0 + &self.spanning_vecs.1,
            -&self.spanning_vecs.0 - &self.spanning_vecs.1
        );

        let mut ftr_corner = &self.center + Point3::new(
            math::f_max_all(vec![
                spans.0[Coord::X], spans.1[Coord::X],
                spans.2[Coord::X], spans.3[Coord::X]
            ]),
            math::f_max_all(vec![
                spans.0[Coord::Y], spans.1[Coord::Y],
                spans.2[Coord::Y], spans.3[Coord::Y]
            ]),
            math::f_max_all(vec![
                spans.0[Coord::Z], spans.1[Coord::Z],
                spans.2[Coord::Z], spans.3[Coord::Z]
            ]),
        );

        let mut bbl_corner = &self.center + Point3::new(
            math::f_min_all(vec![
                spans.0[Coord::X], spans.1[Coord::X],
                spans.2[Coord::X], spans.3[Coord::X]
            ]),
            math::f_min_all(vec![
                spans.0[Coord::Y], spans.1[Coord::Y],
                spans.2[Coord::Y], spans.3[Coord::Y]
            ]),
            math::f_min_all(vec![
                spans.0[Coord::Z], spans.1[Coord::Z],
                spans.2[Coord::Z], spans.3[Coord::Z]
            ]),
        );

        // Add a little padding so ray intersection doesn't devolve
        for &coord in [ Coord::X, Coord::Y, Coord::Z ].iter() {
            if math::f_eq(ftr_corner[coord], bbl_corner[coord]) {
                ftr_corner[coord] += 0.01;
                bbl_corner[coord] -= 0.01;
            }
        }

        let bound = AxisAlignedBoundingBox::new(ftr_corner, bbl_corner);
        bound
    }
}

#[derive(Debug)]
pub struct Triangle {
    corners: (Point3, Point3, Point3),
    material: Arc<dyn Material>
}

impl Triangle {
    pub fn new(corners: (Point3, Point3, Point3), material: Arc<dyn Material>) -> Triangle {
        Triangle { corners, material }
    }

    pub fn a(&self) -> &Point3 {
        &self.corners.0
    }

    pub fn b(&self) -> &Point3 {
        &self.corners.1
    }

    pub fn c(&self) -> &Point3 {
        &self.corners.2
    }
}

impl BoundedHittable for Triangle {}

impl Hittable for Triangle {
    fn is_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        // label corners 0, 1, 2 as A, B, C going counter-clockwise
        let a = &self.corners.0;
        let b = &self.corners.1;
        let c = &self.corners.2;

        let ab = b - a;
        let ac = c - a;
        let bc = c - b;

        let normal = ab.cross(&ac).unit();

        if Vec3::orthogonal(&ray.dir, &normal) { return None; }

        let t = ((&self.corners.0 - &ray.origin).dot(&normal)) / (ray.dir.dot(&normal));

        if ab.cross(&(ray.at(t) - a)).dot(&normal) < 0.0
            || bc.cross(&(ray.at(t) - b)).dot(&normal) < 0.0
            || (-ac).cross(&(ray.at(t) - c)).dot(&normal) < 0.0
        {
            return None;
        }

        if t < t_max && t > t_min {
            let outer = ray.dir.dot(&normal) < 0.0;
            Some(Hit::new(ray.at(t), normal, t, outer, self.material.clone()))
        } else {
            None
        }
    }

    fn surface_area(&self) -> f64 {
        let a = &self.corners.0;
        let b = &self.corners.1;
        let c = &self.corners.2;

        let ab = b - a;
        let ac = c - a;

        0.5 * ab.cross(&ac).norm()
    }
}

impl Bounded for Triangle {
    fn bounding_box(&self) -> AxisAlignedBoundingBox {
        let a = &self.corners.0;
        let b = &self.corners.1;
        let c = &self.corners.2;

        let ftr_corner = Point3::new(
            math::f_max_all(vec![a[Coord::X], b[Coord::X], c[Coord::X]]),
            math::f_max_all(vec![a[Coord::Y], b[Coord::Y], c[Coord::Y]]),
            math::f_max_all(vec![a[Coord::Z], b[Coord::Z], c[Coord::Z]])
        );
        let bbl_corner = Point3::new(
            math::f_min_all(vec![a[Coord::X], b[Coord::X], c[Coord::X]]),
            math::f_min_all(vec![a[Coord::Y], b[Coord::Y], c[Coord::Y]]),
            math::f_min_all(vec![a[Coord::Z], b[Coord::Z], c[Coord::Z]])
        );

        AxisAlignedBoundingBox::new(
            ftr_corner, bbl_corner
        )
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

    #[test]
    fn tri_hit() {
        let mat_dif_white = DiffuseLambert::new(colors::WHITE);
        let tri = Triangle::new(
            (Point3::new(-0.5, -0.5, -0.5),
             Point3::new( 0.5, -0.5, -0.5),
             Point3::new( 0. ,  0.0, -0.5)),
            Arc::new(mat_dif_white)
        );
        let ray = Ray::new(&Vec3::O, &-Vec3::K);
        assert!(tri.is_hit(&ray, 0.0, f64::INFINITY).is_some(),
            "Ray should have hit triangle but didn't.")
    }

    #[test]
    fn tri_miss() {
        let mat_dif_white = DiffuseLambert::new(colors::WHITE);
        let tri = Triangle::new(
            (Point3::new(-0.5, -0.5, -0.5),
             Point3::new( 0.5, -0.5, -0.5),
             Point3::new( 0. ,  0.0, -0.5)),
            Arc::new(mat_dif_white)
        );
        let ray = Ray::new(&Vec3::O, &(Vec3::J - Vec3::K));
        assert!(tri.is_hit(&ray, 0.0, f64::INFINITY).is_none(),
            "Ray shouldn't have hit triangle but did.")
    }
}
