use raytracer::{
    camera::Camera,
    geom::{ HittableGroup, Sphere },
    material,
    vec::{ ColorRGB, Point3 }
};

const OUT_WIDTH:  u32 = 320;
const OUT_HEIGHT: u32 = 240;
const SAMPLES:    u32 = 100;
const MAX_DEPTH:  u32 = 50;
const FOV_DEG:    f64 = 100.0;

fn main() {
    let config = raytracer::ImageConfig {
        width: OUT_WIDTH, height: OUT_HEIGHT,
        samples: SAMPLES, max_depth: MAX_DEPTH
    };

    let mat_dif_soft_blue = material::DiffuseLambert::new(ColorRGB::new(0.3, 0.5, 0.8));
    let mat_dif_soft_red = material::DiffuseLambert::new(ColorRGB::new(0.8, 0.3, 0.4));
    let mat_glass_white = material::Transparent::new(ColorRGB::new(1.0, 1.0, 1.0), 1.52);
    let mat_metal_rough_soft_green = material::Reflective::new(ColorRGB::new(0.6, 0.8, 0.3), 0.6);

    let mut world: HittableGroup = vec![];
    let ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, &mat_dif_soft_blue);
    let sphere1 = Sphere::new(Point3::new(0.6, -0.2, -1.0), 0.3, &mat_dif_soft_red);
    let sphere2 = Sphere::new(Point3::new(-0.27, -0.1, -0.8), 0.4, &mat_glass_white);
    let sphere3 = Sphere::new(Point3::new(0.0, 0.0, -1.5), 0.5, &mat_metal_rough_soft_green);

    world.push(&ground);
    world.push(&sphere1);
    world.push(&sphere2);
    world.push(&sphere3);

    let camera =
        Camera::new(Point3::new(1.0, -0.4, 1.0), &Point3::new(0.0, 0.0, -1.0),
            FOV_DEG, OUT_WIDTH, OUT_HEIGHT);
    print!("{}", raytracer::create_ppm(&world, &camera, &config));

    /*
    use std::fs::{ File, create_dir_all };
    use std::io::prelude::Write;
    use std::path::Path;

    let look_at = Point3::new(0.0, 0.0, -1.0);
    create_dir_all("frames/").unwrap();
    for i in 0..120 {
        eprintln!("Frame {}:", i+1);
        let frame_name = format!("frames/frame{}.ppm", i+1);
        let path = Path::new(&frame_name);
        let mut file = File::create(&path).unwrap();

        let dist = std::f64::consts::SQRT_2;
        let angle = f64::from(i) * 2.0 * std::f64::consts::PI / 120.0;
        let camera =
            Camera::new(&look_at + Point3::new(dist * angle.cos(), -0.4, dist * angle.sin()),
                &look_at, FOV_DEG, OUT_WIDTH, OUT_HEIGHT);


        file.write_all(raytracer::create_ppm(&world, &camera, &config).as_bytes()).unwrap();
    }
    */
}
