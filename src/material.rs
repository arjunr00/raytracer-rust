use super::math::Rand;
use super::vec::{ Ray, ColorRGB, Point3, Vec3 };

pub trait Material {
    fn scatter(&self, in_ray: &Ray, point: &Point3, normal: &Vec3, rand: &mut Rand) -> Option<Ray>;
    fn attenuation(&self) -> &ColorRGB;
}

pub struct DiffuseLambert {
    albedo: ColorRGB
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
