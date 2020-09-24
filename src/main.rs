use raytracer::geom::{ HittableGroup, Sphere };
use raytracer::vec::{ Point3 };

const OUT_WIDTH:  u32 = 640;
const OUT_HEIGHT: u32 = 480;
const SAMPLES:    u32 = 100;

fn main() {
    let mut world: HittableGroup = vec![];
    let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    let ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0);
    world.push(&sphere);
    world.push(&ground);

    print!("{}", raytracer::create_ppm(&world, OUT_WIDTH, OUT_HEIGHT, SAMPLES));
}
