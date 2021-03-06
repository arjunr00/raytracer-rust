use std::ffi::OsStr;
use std::path::Path;
use std::sync::Arc;

use super::hit::{
    AxisAlignedBoundingBox,
    Bounded,
    BoundedHittable,
    Hit,
    Hittable,
    HittableRefs,
    HittableGroup,
};
use super::primitives::{ Plane, Triangle };

use crate::loader::{ Loader, Polygon };
use crate::material::Material;
use crate::math;
use crate::vec::{ Point3, Ray, Vec3 };

#[derive(Debug)]
pub struct Prism {
    center: Point3,
    spanning_vecs: (Vec3, Vec3, Vec3),
    primitives: HittableGroup
}

#[derive(Debug)]
pub struct Icosahedron {
    center: Point3,
    radius: f64,
    primitives: HittableGroup
}

#[derive(Debug)]
pub struct Object {
    center: Point3,
    primitives: HittableGroup
}

#[derive(Debug)]
pub struct Volume {
    boundary: Arc<dyn BoundedHittable>,
    material: Arc<dyn Material>,
    inv_density: f64
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
    fn is_hit(&self, ray: &Ray, t_min: f64, t_max: f64, rand: &mut math::Rand) -> Option<Hit> {
        self.primitives.is_hit(ray, t_min, t_max, rand)
    }

    fn surface_area(&self) -> f64 {
        self.primitives.hittables().iter().fold(0.0, |acc, plane| acc + plane.surface_area())
    }
}

impl Bounded for Prism {
    fn bounding_box(&self) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox::union_from_objs(self.primitives.hittables())
    }
}

impl Icosahedron {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Icosahedron {
        let phi = (1.0 + f64::sqrt(5.0)) / 2.0;
        let inv_radius = 1.0 / f64::sqrt(phi + 2.0);

        let verts = [
            &center + radius * inv_radius * Point3::new(0.0, -1.0,  phi), // 0
            &center + radius * inv_radius * Point3::new( phi, 0.0,  1.0), // 1
            &center + radius * inv_radius * Point3::new( phi, 0.0, -1.0), // 2
            &center + radius * inv_radius * Point3::new(-phi, 0.0, -1.0), // 3
            &center + radius * inv_radius * Point3::new(-phi, 0.0,  1.0), // 4
            &center + radius * inv_radius * Point3::new(-1.0,  phi, 0.0), // 5
            &center + radius * inv_radius * Point3::new( 1.0,  phi, 0.0), // 6
            &center + radius * inv_radius * Point3::new( 1.0, -phi, 0.0), // 7
            &center + radius * inv_radius * Point3::new(-1.0, -phi, 0.0), // 8
            &center + radius * inv_radius * Point3::new(0.0, -1.0, -phi), // 9
            &center + radius * inv_radius * Point3::new(0.0,  1.0, -phi), // 10
            &center + radius * inv_radius * Point3::new(0.0,  1.0,  phi), // 11
        ];
        let icosahedron_tris = HittableGroup::new(vec![
            Arc::new(Triangle::new((verts[1].clone(), verts[2].clone(), verts[6].clone()), material.clone())),
            Arc::new(Triangle::new((verts[1].clone(), verts[7].clone(), verts[2].clone()), material.clone())),
            Arc::new(Triangle::new((verts[3].clone(), verts[4].clone(), verts[5].clone()), material.clone())),
            Arc::new(Triangle::new((verts[4].clone(), verts[3].clone(), verts[8].clone()), material.clone())),
            Arc::new(Triangle::new((verts[6].clone(), verts[5].clone(), verts[11].clone()), material.clone())),
            Arc::new(Triangle::new((verts[5].clone(), verts[6].clone(), verts[10].clone()), material.clone())),
            Arc::new(Triangle::new((verts[9].clone(), verts[10].clone(), verts[2].clone()), material.clone())),
            Arc::new(Triangle::new((verts[10].clone(), verts[9].clone(), verts[3].clone()), material.clone())),
            Arc::new(Triangle::new((verts[7].clone(), verts[8].clone(), verts[9].clone()), material.clone())),
            Arc::new(Triangle::new((verts[8].clone(), verts[7].clone(), verts[0].clone()), material.clone())),
            Arc::new(Triangle::new((verts[11].clone(), verts[0].clone(), verts[1].clone()), material.clone())),
            Arc::new(Triangle::new((verts[0].clone(), verts[11].clone(), verts[4].clone()), material.clone())),
            Arc::new(Triangle::new((verts[6].clone(), verts[2].clone(), verts[10].clone()), material.clone())),
            Arc::new(Triangle::new((verts[1].clone(), verts[6].clone(), verts[11].clone()), material.clone())),
            Arc::new(Triangle::new((verts[3].clone(), verts[5].clone(), verts[10].clone()), material.clone())),
            Arc::new(Triangle::new((verts[5].clone(), verts[4].clone(), verts[11].clone()), material.clone())),
            Arc::new(Triangle::new((verts[2].clone(), verts[7].clone(), verts[9].clone()), material.clone())),
            Arc::new(Triangle::new((verts[7].clone(), verts[1].clone(), verts[0].clone()), material.clone())),
            Arc::new(Triangle::new((verts[3].clone(), verts[9].clone(), verts[8].clone()), material.clone())),
            Arc::new(Triangle::new((verts[4].clone(), verts[8].clone(), verts[0].clone()), material.clone()))
        ]);

        Icosahedron {
            center, radius,
            primitives: icosahedron_tris
        }
    }
}

impl BoundedHittable for Icosahedron {}

impl Hittable for Icosahedron {
    fn is_hit(&self, ray: &Ray, t_min: f64, t_max: f64, rand: &mut math::Rand) -> Option<Hit> {
        self.primitives.is_hit(ray, t_min, t_max, rand)
    }

    fn surface_area(&self) -> f64 {
        self.primitives.hittables().iter().fold(0.0, |acc, tri| acc + tri.surface_area())
    }
}

impl Bounded for Icosahedron {
    fn bounding_box(&self) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox::union_from_objs(self.primitives.hittables())
    }
}

impl Object {
    pub fn new(center: Point3, scale: f64, rotations: Vec<(f64, Vec3)>,
        filepath: &Path, material: Arc<dyn Material>)
        -> Object
    {
        match filepath.extension().and_then(OsStr::to_str) {
            Some("obj") => {
                let obj = Loader::load_obj(filepath).unwrap();
                let mut tris: Vec<Triangle> = vec![];
                let mut primitives: HittableRefs = vec![];
                for face in obj.indices {
                    match face.0 {
                        Polygon::Tri => {
                            let a = &center + scale * (obj.vertices[face.1[0]].clone());
                            let b = &center + scale * (obj.vertices[face.1[1]].clone());
                            let c = &center + scale * (obj.vertices[face.1[2]].clone());

                            tris.push(Triangle::new(
                                (a.clone(), b.clone(), c.clone()), material.clone()
                            ));
                            primitives.push(Arc::new(Triangle::new(
                                (a, b, c), material.clone()
                            )));
                        }
                    }
                }

                let mut adjusted_primitives: HittableRefs = vec![];
                let bound_center = Vec3::O + AxisAlignedBoundingBox::union_from_objs(&primitives).center();
                for tri in &tris {
                    let mut new_a = tri.a() - &bound_center;
                    let mut new_b = tri.b() - &bound_center;
                    let mut new_c = tri.c() - &bound_center;

                    for rotation in &rotations {
                         new_a = new_a.rotate(rotation.0, &rotation.1);
                         new_b = new_b.rotate(rotation.0, &rotation.1);
                         new_c = new_c.rotate(rotation.0, &rotation.1);
                    }

                    new_a += &center;
                    new_b += &center;
                    new_c += &center;

                    adjusted_primitives.push(Arc::new(Triangle::new(
                        (new_a, new_b, new_c), material.clone()
                    )));
                }

                Object {
                    center,
                    primitives: HittableGroup::new(adjusted_primitives)
                }
            },
            Some(_) | None => todo!()
        }
    }


}

impl BoundedHittable for Object {}

impl Hittable for Object {
    fn is_hit(&self, ray: &Ray, t_min: f64, t_max: f64, rand: &mut math::Rand) -> Option<Hit> {
        self.primitives.is_hit(ray, t_min, t_max, rand)
    }

    fn surface_area(&self) -> f64 {
        self.primitives.hittables().iter().fold(0.0, |acc, tri| acc + tri.surface_area())
    }
}

impl Bounded for Object {
    fn bounding_box(&self) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox::union_from_objs(self.primitives.hittables())
    }
}

impl Volume {
    pub fn new(boundary: Arc<dyn BoundedHittable>, density: f64, material: Arc<dyn Material>)
        -> Volume
    {
        Volume {
            boundary, material,
            inv_density: 1.0 / density
        }
    }
}

impl BoundedHittable for Volume {}

impl Hittable for Volume {
    fn is_hit(&self, ray: &Ray, t_min: f64, t_max: f64, rand: &mut math::Rand) -> Option<Hit> {
        let mut entry_hit = self.boundary.is_hit(ray, -f64::INFINITY, f64::INFINITY, rand)?;
        let mut exit_hit =
            match self.boundary.is_hit(ray, entry_hit.t + Hit::FP_OFFSET, f64::INFINITY, rand) {
                Some(hit) => hit,
                None => { return Some(entry_hit); }
            };

        if entry_hit.t < t_min { entry_hit.t = t_min; }
        if exit_hit.t > t_max { exit_hit.t = t_max; }

        if math::f_geq(entry_hit.t, exit_hit.t) {
            return None;
        }

        let entry_exit_dist = exit_hit.t - entry_hit.t;
        let hit_dist = self.inv_density * -math::rand_f64(rand).log10();
        if hit_dist > entry_exit_dist {
            return None
        }

        let t = entry_hit.t + hit_dist;

        Some(Hit::new(
            ray.at(t),
            Vec3::O,
            t,
            true,
            self.material.clone()
        ))
    }

    fn surface_area(&self) -> f64 {
        self.boundary.surface_area()
    }
}

impl Bounded for Volume {
    fn bounding_box(&self) -> AxisAlignedBoundingBox {
        self.boundary.bounding_box()
    }
}
