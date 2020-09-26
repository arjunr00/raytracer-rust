use super::vec::{ Coord, Point3, Ray, Vec3 };
use super::math;

pub struct Camera {
    location: Point3,
    top_left: Point3,
    vp_width: f64,
    vp_height: f64,
    lens_radius: f64,
    local_system: (Point3, Point3, Point3)
}

impl Camera {
    pub fn new(location: Point3, look_at: &Point3, fov_deg: f64, aperture: f64,
        vp_width: u32, vp_height: u32) -> Camera
    {
        let focal_length = (&location - look_at).norm();

        let aspect_ratio = f64::from(vp_width) / f64::from(vp_height);
        let vp_x_max = focal_length * (f64::to_radians(fov_deg) / 2.0).tan();
        let vp_y_max = vp_x_max / aspect_ratio;

        let local_k = (&location - look_at).unit();
        let local_i = Vec3::J.cross(&local_k).unit();
        let local_j = local_k.cross(&local_i);

        Camera {
            top_left: &location
                - (&local_i * vp_x_max)
                + (&local_j * vp_y_max)
                - &local_k * focal_length,
            location,
            vp_width: 2.0 * vp_x_max,
            vp_height: 2.0 * vp_y_max,
            lens_radius: 0.5 * aperture,
            local_system: (local_i, local_j, local_k)
        }
    }

    pub fn ray(&self, u: f64, v: f64, rand: &mut math::Rand) -> Ray {
        let random_vec = self.lens_radius * Vec3::random_in_unit_disc(rand);
        let offset =
            random_vec[Coord::X] * &self.local_system.0
            + random_vec[Coord::Y] * &self.local_system.1;

        let dir =
            &self.top_left
            + (u * self.vp_width) * &self.local_system.0
            - (v * self.vp_height) * &self.local_system.1
            - &self.location - &offset;
        Ray::new(&(&self.location + &offset), &dir)
    }
}
