use std::fs::{ File, create_dir_all };
use std::io::prelude::Write;
use std::path::Path;
use std::f64::consts;

use raytracer::{
    camera::Camera,
    geom::{ HittableGroup, Plane, Sphere, Prism },
    vec::{ colors, ColorRGB, Point3, Vec3 },
    math,
    material
};

enum RenderType {
    Static,
    Animated
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let render_opt =
        if args.len() < 2 { 0 } else { args[1].parse().unwrap_or(1) };

    match render_opt {
        1 => {
            eprintln!("Rendering scene 1:");
            let render_type =
                if args.len() < 3 { RenderType::Static }
                else {
                    match &args[2][..] {
                        "animate" => RenderType::Animated,
                        "static" | _ => RenderType::Static
                    }
                };
            render_scene_1(render_type);
        },
        2 => {
            eprintln!("Rendering scene 2:");
            render_scene_2();
        },
        _ => {
            eprintln!("No scene selected.");
            std::process::exit(1);
        }
    };

}

fn render_scene_1(render_type: RenderType) {
    let out_width = 320;
    let out_height = 240;
    let fov_deg = 50.0;
    let aperture = 0.1;
    let samples = 100;
    let max_depth = 50;

    let background = |t| {
        math::lerp(colors::SKYBLUE, colors::WHITE, t)
    };
    let config = raytracer::ImageConfig {
        width: out_width, height: out_height,
        samples: samples, max_depth: max_depth,
        background: &background
    };

    let mat_dif_soft_blue = material::DiffuseLambert::new(ColorRGB::new(0.3, 0.5, 0.8));
    let mat_dif_soft_red = material::DiffuseLambert::new(ColorRGB::new(0.8, 0.3, 0.4));
    let mat_dif_soft_gray = material::Emissive::new(ColorRGB::new(0.8, 0.8, 0.8), 1.0);
    let mat_glass_white = material::Transparent::new(ColorRGB::new(1.0, 1.0, 1.0), 1.52);
    let mat_metal_rough_soft_green = material::Reflective::new(ColorRGB::new(0.6, 0.8, 0.3), 0.3);

    let ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, &mat_dif_soft_blue);
    let red_ball = Sphere::new(Point3::new(0.6, -0.2, -1.0), 0.3, &mat_dif_soft_red);
    let glass_ball = Sphere::new(Point3::new(-0.27, -0.1, -0.8), 0.4, &mat_glass_white);
    let green_metal_ball = Sphere::new(Point3::new(0.0, 0.0, -1.5), 0.5, &mat_metal_rough_soft_green);
    let gray_plane = Plane::new(
        Point3::new(-0.5, 0.5, -2.5),
        (0.25 * (Vec3::I - Vec3::K), 0.25 * (Vec3::I + Vec3::J)),
        &mat_dif_soft_gray
    );

    let world: HittableGroup = vec![
        &ground, &red_ball, &glass_ball, &green_metal_ball, &gray_plane
    ];

    match render_type {
        RenderType::Static => {
            let camera =
                Camera::new(Point3::new(-1.5, 1.0, 3.0), &Point3::new(0.0, 0.0, -1.0),
                    fov_deg, aperture, out_width, out_height);

            let mut file = File::create(&Path::new("temp.ppm")).unwrap();
            file.write_all(raytracer::create_ppm(&world, &camera, &config).as_bytes()).unwrap();

            // Uncomment to watch render live
            // raytracer::write_ppm(&world, &camera, "temp.ppm", &config);
        },
        RenderType::Animated => {
            let frames = 120;
            let look_at = Point3::new(0.0, 0.0, -1.0);
            create_dir_all("frames/").unwrap();
            for i in 0..frames {
                eprintln!("Frame {}:", i+1);
                let frame_name = format!("frames/frame{}.ppm", i+1);

                let dist = 3.0 * consts::SQRT_2;
                let angle = f64::from(i) * 2.0 * consts::PI / f64::from(frames);
                let camera =
                    Camera::new(&look_at + Point3::new(dist * angle.cos(), 0.4, dist * angle.sin()),
                        &look_at, fov_deg, aperture, out_width, out_height);

                let path = Path::new(&frame_name);
                let mut file = File::create(&path).unwrap();
                file.write_all(raytracer::create_ppm(&world, &camera, &config).as_bytes()).unwrap();

                // Uncomment to watch render live
                // raytracer::write_ppm(&world, &camera, &frame_name, &config);
            }
        }
    };
}

// Cornell box
fn render_scene_2() {
    let out_width = 512;
    let out_height = 512;
    let fov_deg = 37.0;
    let aperture = 0.0;
    let samples = 10000;
    let max_depth = 500;

    let background = |_| {
        0.3 * colors::WHITE
    };
    let config = raytracer::ImageConfig {
        width: out_width, height: out_height,
        samples: samples, max_depth: max_depth,
        background: &background
    };
    let camera =
        Camera::new(Point3::new(278.0, 273.0, -800.0), &Point3::new(278.0, 273.0, 800.0),
            fov_deg, aperture, out_width, out_height);

    let mat_dif_white = material::DiffuseLambert::new(ColorRGB::new(1.0, 1.0, 1.0));
    let mat_dif_red = material::DiffuseLambert::new(ColorRGB::new(0.57, 0.025, 0.025));
    let mat_dif_green = material::DiffuseLambert::new(ColorRGB::new(0.025, 0.236, 0.025));
    let mat_light = material::Emissive::new(ColorRGB::new(1.0, 0.67, 0.21), 16.0);

    let floor = Plane::new(
        Point3::new(278.0, 0.0, 279.6),
        (Point3::new(-278.0, 0.0, 0.0), Point3::new(0.0, 0.0, 279.6)),
        &mat_dif_white
    );
    let ceiling = Plane::new(
        Point3::new(278.0, 548.8, 279.6),
        (Point3::new(278.0, 0.0, 0.0), Point3::new(0.0, 0.0, 279.6)),
        &mat_dif_white
    );
    let back_wall = Plane::new(
        Point3::new(278.0, 274.4, 559.2),
        (Point3::new(-278.0, 0.0, 0.0), Point3::new(0.0, 274.4, 0.0)),
        &mat_dif_white
    );
    let right_wall = Plane::new(
        Point3::new(0.0, 274.4, 279.6),
        (Point3::new(0.0, 0.0, 279.6), Point3::new(0.0, -274.4, 0.0)),
        &mat_dif_green
    );
    let left_wall = Plane::new(
        Point3::new(556.0, 274.4, 279.6),
        (Point3::new(0.0, 0.0, 279.6), Point3::new(0.0, 274.4, 0.0)),
        &mat_dif_red
    );

    let light = Plane::new(
        Point3::new(278.0, 548.7, 279.5),
        (Point3::new(65.0, 0.0, 0.0), Point3::new(0.0, 0.0, 52.5)),
        &mat_light
    );

    let short_block = Prism::new(
        Point3::new(185.0, 82.5, 168.5),
        (Point3::new(80.0, 0.0, 24.0), Point3::new(0.0, 82.5, 0.0), Point3::new(24.0, 0.0, -80.0)),
        &mat_dif_white
    );
    let tall_block = Prism::new(
        Point3::new(368.0, 165.0, 351.0),
        (Point3::new(79.0, 0.0, -24.5), Point3::new(0.0, 165.0, 0.0), Point3::new(24.5, 0.0, 79.0)),
        &mat_dif_white
    );

    let world: HittableGroup = vec![
        &floor, &back_wall, &right_wall, &left_wall, &ceiling,
        &short_block, &tall_block,
        &light
    ];

    let mut file = File::create(&Path::new("cornell.ppm")).unwrap();
    file.write_all(raytracer::create_ppm(&world, &camera, &config).as_bytes()).unwrap();

    // Uncomment to watch render live
    // raytracer::write_ppm(&world, &camera, "cornell.ppm", &config);
}
