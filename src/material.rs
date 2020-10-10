use super::geom::hit::Hit;
use super::math::{ Rand, lerp, f_clamp };
use super::vec::{ colors, Ray, ColorRGB, Vec3 };

pub trait MaterialBase {
    fn attenuation(&self) -> &ColorRGB;
    fn scatter(&self, _: &Ray, _: &Hit, _: &mut Rand) -> Option<Ray> { None }
    fn emit(&self) -> ColorRGB { colors::BLACK }
}

pub trait Material: MaterialBase + Send + Sync + std::fmt::Debug {}

#[derive(Debug)]
pub struct DiffuseLambert {
    albedo: ColorRGB
}

#[derive(Debug)]
pub struct Reflective {
    albedo: ColorRGB,
    roughness: f64
}

#[derive(Debug)]
pub struct Translucent {
    albedo: ColorRGB,
    ref_index: f64,
    roughness: f64
}

#[derive(Debug)]
pub struct Emissive {
    albedo: ColorRGB,
    intensity: f64
}

impl DiffuseLambert {
    pub fn new(albedo: ColorRGB) -> DiffuseLambert {
        DiffuseLambert { albedo }
    }
}

impl Material for DiffuseLambert {}

impl MaterialBase for DiffuseLambert {
    fn scatter(&self, _: &Ray, hit: &Hit, rand: &mut Rand) -> Option<Ray>
    {
        let random_unit = Vec3::random_unit(rand);
        let dir = &hit.normal + random_unit;
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

impl Material for Reflective {}

impl MaterialBase for Reflective {
    fn scatter(&self, in_ray: &Ray, hit: &Hit, rand: &mut Rand) -> Option<Ray> {
        let reflection_dir = in_ray.dir.reflect(&hit.normal);
        let scattered = Ray::new(&hit.point, &(reflection_dir + self.roughness * Vec3::random_unit(rand)));
        if scattered.dir.dot(&hit.normal) > 0.0 { Some(scattered) } else { None }
    }

    fn attenuation(&self) -> &ColorRGB {
        &self.albedo
    }
}

impl Translucent {
    const REF_INDEX_OF_AIR: f64 = 1.0;

    pub fn new(albedo: ColorRGB, ref_index: f64, roughness: f64) -> Translucent {
        Translucent {
            albedo,
            ref_index: if ref_index < 1.0 { 1.0 } else { ref_index },
            roughness: f_clamp(roughness, 0.0, 1.0)
        }
    }

    pub fn get_refractive_index(&self) -> f64 {
        self.ref_index
    }
}

impl Material for Translucent {}

impl MaterialBase for Translucent {
    fn scatter(&self, in_ray: &Ray, hit: &Hit, rand: &mut Rand) -> Option<Ray> {
        let refraction_dir =
            if hit.outer {
                in_ray.dir.refract(&hit.normal, Translucent::REF_INDEX_OF_AIR, self.ref_index, rand)
            } else {
                in_ray.dir.refract(&hit.normal, self.ref_index, Translucent::REF_INDEX_OF_AIR, rand)
            };

        Some(Ray::new(&hit.point,
                &lerp(
                    refraction_dir,
                    &hit.normal + Vec3::random_unit(rand),
                    self.roughness
                )))
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

impl Material for Emissive {}

impl MaterialBase for Emissive {
    fn attenuation(&self) -> &ColorRGB {
        &self.albedo
    }

    fn emit(&self) -> ColorRGB {
        ColorRGB::new(self.intensity, self.intensity, self.intensity)
    }
}
