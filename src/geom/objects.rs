use std::sync::Arc;

use super::hit::{
    AxisAlignedBoundingBox,
    Bounded,
    Hit,
    Hittable,
    BoundedHittable,
    HittableGroup,
};
use super::primitives::Plane;

use crate::material::Material;
use crate::vec::{ Point3, Ray, Vec3 };

#[derive(Debug)]
pub struct Prism {
    center: Point3,
    spanning_vecs: (Vec3, Vec3, Vec3),
    primitives: HittableGroup
}

impl Prism {
    pub fn new(center: Point3, spanning_vecs: (Vec3, Vec3, Vec3), material: Arc<dyn Material>)
        -> Prism
    {
        let prism_i = &spanning_vecs.0;
        let prism_j = &spanning_vecs.1;
        let prism_k = &spanning_vecs.2;

        let front_face = Arc::new(Plane::new(
            &center - prism_k, (prism_j.clone(), prism_i.clone()), material.clone()
        ));
        let back_face = Arc::new(Plane::new(
            &center + prism_k, (prism_i.clone(), prism_j.clone()), material.clone()
        ));
        let top_face = Arc::new(Plane::new(
            &center + prism_j, (prism_k.clone(), prism_i.clone()), material.clone()
        ));
        let bottom_face = Arc::new(Plane::new(
            &center - prism_j, (prism_i.clone(), prism_k.clone()), material.clone()
        ));
        let left_face = Arc::new(Plane::new(
            &center - prism_i, (prism_k.clone(), prism_j.clone()), material.clone()
        ));
        let right_face = Arc::new(Plane::new(
            &center + prism_i, (prism_j.clone(), prism_k.clone()), material.clone()
        ));

        let primitives = HittableGroup::new(vec![
            front_face, top_face, left_face, bottom_face, right_face, back_face 
        ]);

        Prism { center, spanning_vecs, primitives }
    }
}

impl BoundedHittable for Prism {}

impl Hittable for Prism {
    fn is_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut hit: Option<Hit> = None;
        let mut closest_t = t_max;

        for obj in self.primitives.hittables() {
            if let Some(obj_hit) = obj.is_hit(ray, t_min, closest_t) {
                closest_t = obj_hit.t;
                hit = Some(obj_hit);
            }
        }

        hit
    }

    fn surface_area(&self) -> f64 {
        self.primitives.hittables().iter().fold(0.0, |acc, plane| acc + plane.surface_area())
    }
}

impl Bounded for Prism {
    fn bounding_box(&self) -> AxisAlignedBoundingBox {
        let ftr_corner = &self.center + &self.spanning_vecs.0 + &self.spanning_vecs.1 + &self.spanning_vecs.2;
        let bbl_corner = &self.center - &self.spanning_vecs.0 - &self.spanning_vecs.1 - &self.spanning_vecs.2;
        AxisAlignedBoundingBox { ftr_corner, bbl_corner, center: self.center.clone() }
    }
}
