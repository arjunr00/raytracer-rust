#![allow(unused)]
use rand::prelude::Rng;
use std::{
    f64::consts,
    fs::{ File, create_dir_all },
    io::prelude::Write,
    path::Path,
    sync::Arc
};

use raytracer::{
    camera::Camera,
    geom::{
        World,
        hit::{ Bounded, HittableRefs },
        primitives::{ Plane, Sphere, Triangle },
        objects::{ Icosahedron, Prism, Object, Volume }
    },
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
            eprintln!("Rendering scene 2.");
            render_scene_2();
        },
        3 => {
            eprintln!("Rendering scene 3.");
            render_scene_3();
        },
        4 => {
            eprintln!("Rendering scene 4.");
            render_scene_4();
        },
        _ => {
            eprintln!("{} is not a valid scene number.", render_opt);
            std::process::exit(1);
        }
    };

}

fn render_scene_1(render_type: RenderType) {
    let out_width = 640;
    let out_height = 480;
    let fov_deg = 30.0;
    let aperture = 0.0;
    let samples = 100;
    let max_depth = 50;

    let background = |t| {
        math::lerp(colors::SKYBLUE, colors::WHITE, t)
    };
    let config = raytracer::ImageConfig {
        width: out_width, height: out_height,
        samples: samples, max_depth: max_depth,
        background: Arc::new(background)
    };

    let mat_dif_soft_blue = Arc::new(material::DiffuseLambert::new(ColorRGB::new(0.3, 0.5, 0.8)));
    let mat_dif_soft_red = Arc::new(material::DiffuseLambert::new(ColorRGB::new(0.8, 0.3, 0.4)));
    let mat_dif_soft_gray = Arc::new(material::DiffuseLambert::new(ColorRGB::new(0.8, 0.8, 0.8)));
    let mat_glass_white = Arc::new(material::Translucent::new(ColorRGB::new(1.0, 1.0, 1.0), 1.52, 0.0));
    let mat_air = Arc::new(material::Translucent::new(ColorRGB::new(1.0, 1.0, 1.0), 0.0, 0.0));
    let mat_metal_rough_soft_green = Arc::new(material::Reflective::new(ColorRGB::new(0.6, 0.8, 0.3), 0.3));

    let ground = Plane::new(
        Point3::new(0.0, -0.5, -1.0), 
        (100.0 * Vec3::I, 100.0 * Vec3::K),
        mat_dif_soft_blue.clone()
    );
    let glass_ball = Sphere::new(Point3::new(-0.27, -0.1, -0.8), 0.4, mat_glass_white.clone());
    let green_metal_ball = Sphere::new(Point3::new(0.0, 0.0, -1.5), 0.5, mat_metal_rough_soft_green.clone());
    let gray_plane = Plane::new(
        Point3::new(-0.5, 0.8, -2.5),
        (0.25 * (Vec3::I - Vec3::K), 0.25 * (Vec3::I + Vec3::J)),
        mat_dif_soft_gray.clone()
    );
    let red_ball = Sphere::new(Point3::new(0.6, -0.2, -1.0), 0.3, mat_dif_soft_red.clone());

    let world = World::new(vec![
        Arc::new(ground),
        Arc::new(red_ball),
        Arc::new(glass_ball),
        Arc::new(green_metal_ball),
        Arc::new(gray_plane)
    ]);


    match render_type {
        RenderType::Static => {
            let camera =
                Camera::new(Point3::new(0.7, -0.3, 3.0), &Point3::new(0.0, 0.0, -1.0),
                    fov_deg, aperture, out_width, out_height);

            // Single-threaded
            // let mut file = File::create(&Path::new("temp.ppm")).unwrap();
            // file.write_all(raytracer::create_ppm(&world, &camera, &config).as_bytes()).unwrap();

            // Multi-threaded
            let world_arc = Arc::new(world);
            let camera_arc = Arc::new(camera);
            let config_arc = Arc::new(config);
            raytracer::write_ppm_threaded(world_arc, camera_arc, "temp.ppm", config_arc);

            // Uncomment to watch render live
            // raytracer::write_ppm(&world, &camera, "temp.ppm", &config);
        },
        RenderType::Animated => {
            let frames = 120;
            let look_at = Point3::new(0.0, 0.0, -1.0);
            create_dir_all("frames/").unwrap();
            let world_arc = Arc::new(world);
            let config_arc = Arc::new(config);
            for i in 0..frames {
                eprintln!("Frame {}:", i+1);
                let frame_name = format!("frames/frame{}.ppm", i+1);

                let dist = 3.0 * consts::SQRT_2;
                let angle = f64::from(i) * 2.0 * consts::PI / f64::from(frames);
                let camera =
                    Camera::new(&look_at + Point3::new(dist * angle.cos(), 0.4, dist * angle.sin()),
                        &look_at, fov_deg, aperture, out_width, out_height);


                // Single-threaded
                // let path = Path::new(&frame_name);
                // let mut file = File::create(&path).unwrap();
                // file.write_all(raytracer::create_ppm(&world, &camera, &config).as_bytes()).unwrap();

                // Multi-threaded
                let camera_arc = Arc::new(camera);
                raytracer::write_ppm_threaded(
                    world_arc.clone(), camera_arc.clone(), &frame_name, config_arc.clone()
                );

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
    let max_depth = 50;

    let background = |_| {
        colors::BLACK
    };
    let config = raytracer::ImageConfig {
        width: out_width, height: out_height,
        samples: samples, max_depth: max_depth,
        background: Arc::new(background)
    };
    let camera =
        Camera::new(Point3::new(278.0, 273.0, -800.0), &Point3::new(278.0, 273.0, 0.0),
            fov_deg, aperture, out_width, out_height);

    let mat_dif_white = Arc::new(material::DiffuseLambert::new(ColorRGB::new(1.0, 1.0, 1.0)));
    let mat_mirror = Arc::new(material::Reflective::new(ColorRGB::new(1.0, 1.0, 1.0), 0.0));
    let mat_metal_blue = Arc::new(material::Reflective::new(ColorRGB::new(0.3, 0.7, 0.8), 0.4));
    let mat_glass_white = Arc::new(material::Translucent::new(ColorRGB::new(1.0, 1.0, 1.0), 1.52, 0.0));
    let mat_dif_red = Arc::new(material::DiffuseLambert::new(ColorRGB::new(0.57, 0.025, 0.025)));
    let mat_dif_green = Arc::new(material::DiffuseLambert::new(ColorRGB::new(0.025, 0.236, 0.025)));
    let mat_dif_lavender = Arc::new(material::DiffuseLambert::new(ColorRGB::new(0.776, 0.564, 0.976)));
    let mat_light = Arc::new(material::Emissive::new(ColorRGB::new(1.0, 0.67, 0.21), 16.3));

    let floor = Plane::new(
        Point3::new(278.0, 0.0, 279.6),
        (Point3::new(-278.0, 0.0, 0.0), Point3::new(0.0, 0.0, 279.6)),
        mat_dif_white.clone()
    );
    let ceiling = Plane::new(
        Point3::new(278.0, 548.8, 279.6),
        (Point3::new(278.0, 0.0, 0.0), Point3::new(0.0, 0.0, 279.6)),
        mat_dif_white.clone()
    );
    let back_wall = Plane::new(
        Point3::new(278.0, 274.4, 559.2),
        (Point3::new(-278.0, 0.0, 0.0), Point3::new(0.0, 274.4, 0.0)),
        mat_dif_white.clone()
    );
    let right_wall = Plane::new(
        Point3::new(0.0, 274.4, 279.6),
        (Point3::new(0.0, 0.0, 279.6), Point3::new(0.0, -274.4, 0.0)),
        mat_dif_green.clone()
    );
    let left_wall = Plane::new(
        Point3::new(556.0, 274.4, 279.6),
        (Point3::new(0.0, 0.0, 279.6), Point3::new(0.0, 274.4, 0.0)),
        mat_dif_red.clone()
    );

    let light = Plane::new(
        Point3::new(278.0, 548.7, 279.5),
        (Point3::new(65.0, 0.0, 0.0), Point3::new(0.0, 0.0, 52.5)),
        mat_light.clone()
    );

    let short_block = Prism::new(
        Point3::new(185.0, 82.5, 168.5),
        (Point3::new(80.0, 0.0, 24.0), Point3::new(0.0, 82.5, 0.0), Point3::new(24.0, 0.0, -80.0)),
        mat_dif_white.clone()
    );
    let tall_block = Prism::new(
        Point3::new(368.0, 165.0, 351.0),
        (Point3::new(79.0, 0.0, -24.5), Point3::new(0.0, 165.0, 0.0), Point3::new(24.5, 0.0, 79.0)),
        mat_dif_white.clone()
    );
    let small_glass_sphere = Sphere::new(
        Point3::new(185.0, 82.5, 168.5),
        82.5,
        mat_glass_white.clone()
    );
    let tall_mirror_block = Prism::new(
        Point3::new(368.0, 165.0, 351.0),
        (Point3::new(79.0, 0.0, -24.5), Point3::new(0.0, 165.0, 0.0), Point3::new(24.5, 0.0, 79.0)),
        mat_mirror.clone()
    );

    let lucy = Object::new(
        Point3::new(368.0, 210.0, 351.0),
        0.27,
        vec![(90_f64.to_radians(), -Vec3::I), (15_f64.to_radians(), Vec3::J)],
        &Path::new("models/lucy.obj"), mat_metal_blue.clone()
    );
    let bunny = Object::new(
        Point3::new(185.0, 84.5, 168.5),
        1100.0,
        vec![(180_f64.to_radians(), Vec3::J)],
        &Path::new("models/bunny.obj"), mat_dif_lavender.clone()
    );

    let cube = Prism::new(
        Point3::new(278.0, 274.4, 279.6),
        (Point3::new(-278.0, 0.0, 0.0), Point3::new(0.0, 274.2, 0.0), Point3::new(0.0, 0.0, 279.6)),
        mat_dif_white.clone()
    );
    let volume = Volume::new(Arc::new(cube), 0.0005, mat_dif_white.clone());

    let world = World::new(vec![
        Arc::new(floor),
        Arc::new(back_wall),
        Arc::new(right_wall),
        Arc::new(left_wall),
        Arc::new(ceiling),
        Arc::new(short_block),
        Arc::new(tall_block),
        Arc::new(light)
    ]);

    // Single-threaded
    // let mut file = File::create(&Path::new("cornell.ppm")).unwrap();
    // file.write_all(raytracer::create_ppm(&world, &camera, &config).as_bytes()).unwrap();

    // Multi-threaded
    let world_arc = Arc::new(world);
    let camera_arc = Arc::new(camera);
    let config_arc = Arc::new(config);
    raytracer::write_ppm_threaded(world_arc, camera_arc, "cornell.ppm", config_arc);

    // Uncomment to watch render live
    // raytracer::write_ppm(&world, &camera, "cornell.ppm", &config);
}

fn render_scene_3() {
    let out_width = 640;
    let out_height = 480;
    let fov_deg = 30.0;
    let aperture = 0.1;
    let samples = 100;
    let max_depth = 50;

    let background = |t| {
        math::lerp(colors::SKYBLUE, colors::WHITE, t)
    };
    let config = raytracer::ImageConfig {
        width: out_width, height: out_height,
        samples: samples, max_depth: max_depth,
        background: Arc::new(background)
    };

    let mat_dif_soft_blue = Arc::new(material::DiffuseLambert::new(ColorRGB::new(0.3, 0.5, 0.8)));
    let materials = [
        Arc::new(material::DiffuseLambert::new(ColorRGB::new(0.8, 0.3, 0.4))), // reddish
        Arc::new(material::DiffuseLambert::new(ColorRGB::new(0.8, 0.6, 0.3))), // orangish
        Arc::new(material::DiffuseLambert::new(ColorRGB::new(0.8, 0.8, 0.4))), // yellowish
        Arc::new(material::DiffuseLambert::new(ColorRGB::new(0.5, 0.8, 0.3))), // greenish
        Arc::new(material::DiffuseLambert::new(ColorRGB::new(0.3, 0.7, 0.8))), // bluish
        Arc::new(material::DiffuseLambert::new(ColorRGB::new(0.4, 0.3, 0.8))), // indigoish
        Arc::new(material::DiffuseLambert::new(ColorRGB::new(0.6, 0.3, 0.8)))  // violetish
    ];

    let ground = Plane::new(
        Point3::new(0.0, -0.5, -1.0), 
        (100.0 * Vec3::I, 100.0 * Vec3::K),
        mat_dif_soft_blue.clone()
    );

    let mut red_balls: HittableRefs = vec![Arc::new(ground)];
    let mut rng = rand::thread_rng();
    for i in 0..1000 {
        let rand_x: f64 = -(rng.gen::<f64>() * 50.0) + 25.0;
        let rand_z: f64 = -(rng.gen::<f64>() * 50.0);
        let material = materials[i % materials.len()].clone();
        let radius = 0.5 * rng.gen::<f64>() + 0.3;
        red_balls.push(Arc::new(
            Sphere::new(Point3::new(rand_x, -0.5 + radius, rand_z), radius, material)
        ));
    }

    let world = World::new(red_balls);

    let camera =
        Camera::new(Point3::new(0.7, 2.0, 3.0), &Point3::new(0.0, 0.0, -10.0),
            fov_deg, aperture, out_width, out_height);

    // Single-threaded
    // let mut file = File::create(&Path::new("temp.ppm")).unwrap();
    // file.write_all(raytracer::create_ppm(&world, &camera, &config).as_bytes()).unwrap();

    // Multi-threaded
    let world_arc = Arc::new(world);
    let camera_arc = Arc::new(camera);
    let config_arc = Arc::new(config);
    raytracer::write_ppm_threaded(world_arc, camera_arc, "balls.ppm", config_arc);

    // Uncomment to watch render live
    // raytracer::write_ppm(&world, &camera, "temp.ppm", &config);
}

fn render_scene_4() {
    let out_width = 640;
    let out_height = 480;
    let fov_deg = 60.0;
    let aperture = 0.0;
    let samples = 100;
    let max_depth = 50;

    let background = |t| {
        math::lerp(colors::SKYBLUE, colors::WHITE, t)
    };
    let config = raytracer::ImageConfig {
        width: out_width, height: out_height,
        samples: samples, max_depth: max_depth,
        background: Arc::new(background)
    };

    let mat_floor = Arc::new(material::DiffuseLambert::new(ColorRGB::new(0.3, 0.5, 0.8)));
    let mat_dif = Arc::new(material::DiffuseLambert::new(ColorRGB::new(0.8, 0.3, 0.4)));
    let mat_reflect = Arc::new(material::Reflective::new(ColorRGB::new(0.831, 0.686, 0.215), 0.35));
    let mat_glass = Arc::new(material::Translucent::new(ColorRGB::new(1.0, 1.0, 1.0), 1.52, 0.0));
    let mat_air = Arc::new(material::Translucent::new(ColorRGB::new(1.0, 1.0, 1.0), 0.0, 0.0));

    let mat_light = Arc::new(material::Emissive::new(ColorRGB::new(1.0, 1.0, 1.0), 7.0));

    let mat_skin = Arc::new(material::Translucent::new(ColorRGB::new(0.945, 0.760, 0.490), 0.0, 0.5));
    let mat_volume = Arc::new(material::DiffuseLambert::new(ColorRGB::new(0.772, 0.301, 0.301)));

    let ground = Plane::new(
        Point3::new(0.0, -0.5, 0.0),
        (1000.0 * Vec3::I, 1000.0 * Vec3::K),
        mat_floor.clone()
    );
    let light = Plane::new(
        Point3::new(0.0, 14.0, 7.0),
        (2.3 * Vec3::I, 2.3 * (Vec3::J - Vec3::K)),
        mat_light.clone()
    );

    let size = 0.1;
    let object = Object::new(
        Point3::new(0.0, 7.0, 0.0),
        size, vec![],
        &Path::new("models/armadillo.obj"), mat_reflect.clone()
    );

    let camera_focus = object.bounding_box().center().clone();
    let camera =
        Camera::new(&camera_focus + Point3::new(10.0, 3.0, -20.0), &camera_focus,
            fov_deg, aperture, out_width, out_height);

    let volume = Arc::new(Volume::new(object.clone(), 0.4, mat_volume.clone()));

    let world = World::new(vec![
        Arc::new(ground),
        Arc::new(object)
    ]);

    // Single-threaded
    // let mut file = File::create(&Path::new("temp.ppm")).unwrap();
    // file.write_all(raytracer::create_ppm(&world, &camera, &config).as_bytes()).unwrap();

    // Multi-threaded
    let world_arc = Arc::new(world);
    let camera_arc = Arc::new(camera);
    let config_arc = Arc::new(config);
    raytracer::write_ppm_threaded(world_arc, camera_arc, "armadillo.ppm", config_arc);

    // Uncomment to watch render live
    // raytracer::write_ppm(&world, &camera, "temp.ppm", &config);
}
