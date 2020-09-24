use super::vec::{ Point3, Ray, Vec3 };

pub struct Hit {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64
}

pub trait Hittable {
    fn check_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

pub type HittableGroup<'a> = Vec<&'a dyn Hittable>;

pub struct Sphere {
    center: Point3,
    radius: f64
}

impl Hittable for HittableGroup<'_> {
    fn check_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut hit: Option<Hit> = None;
        let mut closest_t = t_max;

        for obj in self.iter() {
            if let Some(obj_hit) = obj.check_hit(ray, t_min, closest_t) {
                closest_t = obj_hit.t;
                hit = Some(obj_hit);
            }
        }

        hit
    }
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn check_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
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
            Some(Hit {
                point: ray.at(t1),
                normal: (ray.at(t1) - &self.center) * (1.0/self.radius),
                t: 0.0
            })
        } else if t2 < t_max && t2 > t_min {
            Some(Hit {
                point: ray.at(t2),
                normal: (ray.at(t2) - &self.center) * (1.0/self.radius),
                t: 0.0
            })
        } else {
            None
        }
    }
}
