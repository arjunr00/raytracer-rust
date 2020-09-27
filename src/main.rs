use raytracer::{
    camera::Camera,
    geom::{ HittableGroup, Plane, Sphere },
    material::{ DiffuseLambert, Transparent, Reflective },
    vec::{ ColorRGB, Point3, Vec3 }
};

const OUT_WIDTH:  u32 = 640;
const OUT_HEIGHT: u32 = 480;
const SAMPLES:    u32 = 100;
const MAX_DEPTH:  u32 = 50;
const FOV_DEG:    f64 = 30.0;
const APERTURE:   f64 = 0.1;

fn main() {
    let config = raytracer::ImageConfig {
        width: OUT_WIDTH, height: OUT_HEIGHT,
        samples: SAMPLES, max_depth: MAX_DEPTH
    };

    let mat_dif_soft_blue = DiffuseLambert::new(ColorRGB::new(0.3, 0.5, 0.8));
    let mat_dif_soft_red = DiffuseLambert::new(ColorRGB::new(0.8, 0.3, 0.4));
    let mat_dif_soft_gray = DiffuseLambert::new(ColorRGB::new(0.8, 0.8, 0.8));
    let mat_glass_white = Transparent::new(ColorRGB::new(1.0, 1.0, 1.0), 1.52);
    let mat_metal_rough_soft_green = Reflective::new(ColorRGB::new(0.6, 0.8, 0.3), 0.3);

    let mut world: HittableGroup = vec![];
    let ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, &mat_dif_soft_blue);
    let red_ball = Sphere::new(Point3::new(0.6, -0.2, -1.0), 0.3, &mat_dif_soft_red);
    let glass_ball = Sphere::new(Point3::new(-0.27, -0.1, -0.8), 0.4, &mat_glass_white);
    let green_metal_ball = Sphere::new(Point3::new(0.0, 0.0, -1.5), 0.5, &mat_metal_rough_soft_green);
    let gray_plane = Plane::new(
        Point3::new(0.6, 0.5, -1.7), 
        (Vec3::new(0.0, 0.1, 0.2), Vec3::new(0.1, 0.2, -0.1)),
        &mat_dif_soft_gray
    );

    world.push(&ground);
    world.push(&red_ball);
    world.push(&glass_ball);
    world.push(&green_metal_ball);
    world.push(&gray_plane);

    let args: Vec<String> = std::env::args().collect();
    let render_opt =
        if args.len() < 2 { 1 } else { args[1].parse().unwrap_or(1) };

    match render_opt {
        2 => {
            eprintln!("Rendering animated scene 1.");
            use std::fs::{ File, create_dir_all };
            use std::io::prelude::Write;
            use std::f64::consts;
            use std::path::Path;

            let frames = 120;
            let look_at = Point3::new(0.0, 0.0, -1.0);
            create_dir_all("frames/").unwrap();
            for i in 0..frames {
                eprintln!("Frame {}:", i+1);
                let frame_name = format!("frames/frame{}.ppm", i+1);
                let path = Path::new(&frame_name);
                let mut file = File::create(&path).unwrap();

                let dist = 3.0 * consts::SQRT_2;
                let angle = f64::from(i) * 2.0 * consts::PI / f64::from(frames);
                let camera =
                    Camera::new(&look_at + Point3::new(dist * angle.cos(), 0.4, dist * angle.sin()),
                        &look_at, FOV_DEG, APERTURE, OUT_WIDTH, OUT_HEIGHT);

                file.write_all(raytracer::create_ppm(&world, &camera, &config).as_bytes()).unwrap();
            }
        },
        1 | _ => {
            eprintln!("Rendering static scene 1.");
            let camera =
                Camera::new(Point3::new(1.5, -0.3, 3.0), &Point3::new(0.0, 0.0, -1.0),
                    FOV_DEG, APERTURE, OUT_WIDTH, OUT_HEIGHT);
            print!("{}", raytracer::create_ppm(&world, &camera, &config));
        }
    };
}
