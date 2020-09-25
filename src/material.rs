use super::math::{ Rand, f_clamp };
use super::vec::{ Ray, ColorRGB, Point3, Vec3 };

pub trait Material {
    fn scatter(&self, in_ray: &Ray, point: &Point3, normal: &Vec3, rand: &mut Rand) -> Option<Ray>;
    fn attenuation(&self) -> &ColorRGB;
}

pub struct DiffuseLambert {
    albedo: ColorRGB
}

pub struct Metal {
    albedo: ColorRGB,
    roughness: f64
}

impl DiffuseLambert {
    pub fn new(albedo: ColorRGB) -> DiffuseLambert {
        DiffuseLambert { albedo }
    }
}

impl Material for DiffuseLambert {
    fn scatter(&self, _: &Ray, point: &Point3, normal: &Vec3, rand: &mut Rand) -> Option<Ray>
    {
        let dir = normal + Vec3::random_unit(rand);
        Some(Ray::new(point, &dir))
    }

    fn attenuation(&self) -> &ColorRGB {
        &self.albedo
    }
}

impl Metal {
    pub fn new(albedo: ColorRGB, roughness: f64) -> Metal {
        Metal {
            albedo,
            roughness: f_clamp(roughness, 0.0, 1.0)
        }
    }
}

impl Material for Metal {
    fn scatter(&self, in_ray: &Ray, point: &Point3, normal: &Vec3, rand: &mut Rand) -> Option<Ray> {
        let reflection_dir = in_ray.dir.reflect(normal);
        let scattered = Ray::new(point, &(reflection_dir + self.roughness * Vec3::random_unit(rand)));
        if scattered.dir.dot(normal) > 0.0 { Some(scattered) } else { None }
    }

    fn attenuation(&self) -> &ColorRGB {
        &self.albedo
    }
}
