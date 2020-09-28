use super::geom::Hit;
use super::math::{ Rand, f_clamp };
use super::vec::{ colors, Ray, ColorRGB, Vec3 };

pub trait Material {
    fn attenuation(&self) -> &ColorRGB;
    fn scatter(&self, _: &Ray, _: &Hit, _: &mut Rand) -> Option<Ray> { None }
    fn emit(&self) -> ColorRGB { colors::BLACK }
}

pub struct DiffuseLambert {
    albedo: ColorRGB
}

pub struct Reflective {
    albedo: ColorRGB,
    roughness: f64
}

pub struct Transparent {
    albedo: ColorRGB,
    ref_index: f64
}

pub struct Emissive {
    albedo: ColorRGB,
    intensity: f64
}

impl DiffuseLambert {
    pub fn new(albedo: ColorRGB) -> DiffuseLambert {
        DiffuseLambert { albedo }
    }
}

impl Material for DiffuseLambert {
    fn scatter(&self, _: &Ray, hit: &Hit, rand: &mut Rand) -> Option<Ray>
    {
        let mut random_vec = Vec3::random_unit(rand);
        if random_vec.dot(&hit.normal) < 0.0 { random_vec *= -1.0; }
        let dir = &hit.normal + random_vec;
        Some(Ray::new(&hit.point, &dir))
    }

    fn attenuation(&self) -> &ColorRGB {
        &self.albedo
    }
}

impl Reflective {
    pub fn new(albedo: ColorRGB, roughness: f64) -> Reflective {
        Reflective {
            albedo,
            roughness: f_clamp(roughness, 0.0, 1.0)
        }
    }
}

impl Material for Reflective {
    fn scatter(&self, in_ray: &Ray, hit: &Hit, rand: &mut Rand) -> Option<Ray> {
        let reflection_dir = in_ray.dir.reflect(&hit.normal);
        let scattered = Ray::new(&hit.point, &(reflection_dir + self.roughness * Vec3::random_unit(rand)));
        if scattered.dir.dot(&hit.normal) > 0.0 { Some(scattered) } else { None }
    }

    fn attenuation(&self) -> &ColorRGB {
        &self.albedo
    }
}

impl Transparent {
    const REF_INDEX_OF_AIR: f64 = 1.0;

    pub fn new(albedo: ColorRGB, ref_index: f64) -> Transparent {
        Transparent {
            albedo,
            ref_index: if ref_index < 1.0 { 1.0 } else { ref_index }
        }
    }

    pub fn get_refractive_index(&self) -> f64 {
        self.ref_index
    }
}

impl Material for Transparent {
    fn scatter(&self, in_ray: &Ray, hit: &Hit, rand: &mut Rand) -> Option<Ray> {
        let refraction_dir =
            if hit.outer {
                in_ray.dir.refract(&hit.normal, Transparent::REF_INDEX_OF_AIR, self.ref_index, rand)
            } else {
                in_ray.dir.refract(&hit.normal, self.ref_index, Transparent::REF_INDEX_OF_AIR, rand)
            };

        Some(Ray::new(&hit.point, &refraction_dir))
    }

    fn attenuation(&self) -> &ColorRGB {
        &self.albedo
    }
}

impl Emissive {
    pub fn new(albedo: ColorRGB, intensity: f64) -> Emissive {
        Emissive {
            albedo,
            intensity: if intensity < 0.0 { 0.0 } else { intensity }
        }
    }
}

impl Material for Emissive {
    fn attenuation(&self) -> &ColorRGB {
        &self.albedo
    }

    fn emit(&self) -> ColorRGB {
        ColorRGB::new(self.intensity, self.intensity, self.intensity)
    }
}
