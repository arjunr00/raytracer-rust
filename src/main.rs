use raytracer::geom::{ HittableGroup, Sphere };
use raytracer::material::{ DiffuseLambert };
use raytracer::vec::{ ColorRGB, Point3 };

const OUT_WIDTH:  u32 = 320;
const OUT_HEIGHT: u32 = 240;
const SAMPLES:    u32 = 100;
const MAX_DEPTH:  u32 = 50;

fn main() {
    let mat_dif_soft_blue = DiffuseLambert::new(ColorRGB::new(0.3, 0.5, 0.8));
    let mat_dif_soft_red = DiffuseLambert::new(ColorRGB::new(0.8, 0.3, 0.4));

    let mut world: HittableGroup = vec![];
    let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, &mat_dif_soft_red);
    let ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, &mat_dif_soft_blue);
    world.push(&sphere);
    world.push(&ground);

    print!("{}", raytracer::create_ppm(&world, OUT_WIDTH, OUT_HEIGHT, SAMPLES, MAX_DEPTH));
}
