# Rust Raytracer

Figured a raytracer would be a fun way to learn how to use Rust.
I'm using [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) as a guide.

## Dependencies
* Rust and Cargo. Just install `rustup` from [here](https://www.rust-lang.org/tools/install) and it'll install the whole Rust toolchain.

## How to build and run
Run `cargo build` or `cargo build --release` if you just want to compile without running.

In `src/main.rs` there are currently two available scenes you can render:
1. A basic scene with a diffuse sphere, a reflective sphere, a glass sphere, and a diffuse plane.
2. A [Cornell box](https://www.graphics.cornell.edu/online/box/data.html)

Just run `cargo run --release [1|2]` to render one of these.
(You can add `animate` at the end if you're rendering 1 to output a series of frames of an animated camera pivoting about the center of the scene to a subdirectory named `frames/`).

By default, this code will run on all available threads.
You can go into `src/main.rs` and comment and uncomment the appropriate code to change this.

## How to test
Run `cargo test`.
Who would've guessed?

## What it looks like

640x480 with 100 samples per pixel and a maximum of 50 bounces takes ~15 seconds:

![raytracer](https://user-images.githubusercontent.com/30734384/94495091-dbeba680-01be-11eb-9887-86fb676e1fe0.png)

From left to right: transparent ball with refractive index of 1.52, diffuse ball, reflective ball with roughness of 0.3, and diffuse plane.

Here's a lower-resolution (320x240) animation to show off reflection and refraction. The 640x480 version of this took about 35.7 minutes to render 120 frames, so averaging about 17.8 seconds per frame.

![animation](https://user-images.githubusercontent.com/30734384/94495092-dd1cd380-01be-11eb-9ada-fd34f5da4549.gif)

Here's a [Cornell box](https://www.graphics.cornell.edu/online/box/data.html) render (it's not perfect, as you can see from the weird lighting on the front of the boxes, but it's something).
360x360 with 10,000 samples per pixel and a maximum of 500 bounces took ~54 minutes on an 8-core i7 CPU:

![cornell](https://user-images.githubusercontent.com/30734384/94751915-c65db480-0357-11eb-8d6e-a49952c5b2f2.png)
