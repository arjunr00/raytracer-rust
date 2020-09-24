use raytracer::vec::{ ColorRGB, Point3, Vec3 };
use raytracer::ray::Ray;

const OUT_WIDTH:  u32 = 640;
const OUT_HEIGHT: u32 = 480;
const MAX_COLORS: u32 = 255;

fn main() {
    let aspect_ratio = f64::from(OUT_WIDTH / OUT_HEIGHT);
    let vp_y_max = 1.0;
    let vp_x_max = aspect_ratio * vp_y_max;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let i_vec = Vec3::new(vp_x_max, 0.0, 0.0).unit();
    let j_vec = Vec3::new(0.0, vp_y_max, 0.0).unit();
    let k_vec = Vec3::new(0.0, 0.0, focal_length).unit();
    let top_left =
        &origin
        - (&i_vec * vp_x_max)
        + (&j_vec * vp_y_max)
        - (&k_vec * focal_length);

    let mut pixels: Vec<Vec<ColorRGB>> = vec![];

    for i in 0..OUT_HEIGHT {
        pixels.push(vec![]);
        for j in 0..OUT_WIDTH {
            let u = (j as f64) / f64::from(OUT_WIDTH - 1);
            let v = (i as f64) / f64::from(OUT_HEIGHT - 1);

            let ray_dir = &top_left + u * &i_vec - v * &j_vec;
            let r = Ray::new(&origin, &ray_dir);

            pixels[i as usize].push(r.get_color());
        }
    }

    print!("{}", raytracer::create_ppm(&pixels, OUT_WIDTH, OUT_HEIGHT, MAX_COLORS));
}
