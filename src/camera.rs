use super::vec::{ Point3, Ray, Vec3 };

pub struct Camera {
    origin: Point3,
    top_left: Point3,
    vp_height: f64,
    vp_width: f64
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Camera {
        let aspect_ratio = f64::from(width) / f64::from(height);
        let vp_y_max = 1.0;
        let vp_x_max = aspect_ratio * vp_y_max;
        let focal_length = 1.0;

        Camera {
            origin: Vec3::O,
            top_left: Vec3::O
                - (Vec3::I * vp_x_max)
                + (Vec3::J * vp_y_max)
                - (Vec3::K * focal_length),
            vp_width: 2.0 * vp_x_max,
            vp_height: 2.0 * vp_y_max
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        let dir =
            &self.top_left
            + u * (self.vp_width * Vec3::I)
            - v * (self.vp_height * Vec3::J);
        Ray::new(&self.origin, &dir)
    }
}
