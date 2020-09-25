use raytracer::geom::{ HittableGroup, Sphere };
use raytracer::material::{ DiffuseLambert, Metal };
use raytracer::vec::{ ColorRGB, Point3 };

const OUT_WIDTH:  u32 = 640;
const OUT_HEIGHT: u32 = 480;
const SAMPLES:    u32 = 100;
const MAX_DEPTH:  u32 = 50;

fn main() {
    let mat_dif_soft_blue = DiffuseLambert::new(ColorRGB::new(0.3, 0.5, 0.8));
    let mat_dif_soft_red = DiffuseLambert::new(ColorRGB::new(0.8, 0.3, 0.4));
    let mat_metal_smooth_light_gray = Metal::new(ColorRGB::new(0.8, 0.8, 0.8), 0.0);
    let mat_metal_rough_soft_green = Metal::new(ColorRGB::new(0.6, 0.8, 0.3), 0.6);

    let mut world: HittableGroup = vec![];
    let ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, &mat_dif_soft_blue);
    let sphere1 = Sphere::new(Point3::new(0.6, -0.2, -1.0), 0.3, &mat_dif_soft_red);
    let sphere2 = Sphere::new(Point3::new(-0.6, -0.1, -1.0), 0.4, &mat_metal_smooth_light_gray);
    let sphere3 = Sphere::new(Point3::new(0.0, 0.0, -1.5), 0.5, &mat_metal_rough_soft_green);

    world.push(&ground);
    world.push(&sphere1);
    world.push(&sphere2);
    world.push(&sphere3);

    print!("{}", raytracer::create_ppm(&world, OUT_WIDTH, OUT_HEIGHT, SAMPLES, MAX_DEPTH));
}
