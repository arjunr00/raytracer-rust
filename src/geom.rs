use super::vec::{ Point3, Ray, Vec3 };

pub struct Hit {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64
}

pub trait Hittable {
    fn is_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

pub type HittableGroup<'a> = Vec<&'a dyn Hittable>;

pub struct Sphere {
    center: Point3,
    radius: f64
}

impl Hittable for HittableGroup<'_> {
    fn is_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut hit: Option<Hit> = None;
        let mut closest_t = t_max;

        for obj in self.iter() {
            if let Some(obj_hit) = obj.is_hit(ray, t_min, closest_t) {
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
            if outer {
                return Some(Hit {
                    point: ray.at(t1),
                    normal: normal.unit(),
                    t: 0.0
                });
            }
        } else if t2 < t_max && t2 > t_min {
            let normal = (ray.at(t2) - &self.center) * (1.0/self.radius);
            let outer = ray.dir.dot(&normal) < 0.0;
            if outer {
                return Some(Hit {
                    point: ray.at(t2),
                    normal: normal.unit(),
                    t: 0.0
                });
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sphere_hit() {
        let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
        let ray = Ray::new(&Vec3::O, &-Vec3::K);
        assert!(sphere.is_hit(&ray, 0.0, f64::INFINITY).is_some(),
            "Ray should have hit sphere but didn't.")
    }

    #[test]
    fn sphere_miss() {
        let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
        let ray = Ray::new(&Vec3::O, &Vec3::J);
        assert!(sphere.is_hit(&ray, 0.0, f64::INFINITY).is_none(),
            "Ray shouldn't have hit sphere but did.")
    }

    #[test]
    fn sphere_inside_miss() {
        let sphere = Sphere::new(Point3::new(0.0, 0.0, -0.3), 0.5);
        let ray = Ray::new(&Vec3::O, &-Vec3::K);
        assert!(sphere.is_hit(&ray, 0.0, f64::INFINITY).is_none(),
            "Ray shouldn't have hit sphere but did.")
    }
}
