# Rust Raytracer

Figured a raytracer would be a fun way to learn how to use Rust.
I'm using [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) as a guide.

## Dependencies
* Rust and Cargo. Just install `rustup` from [here](https://www.rust-lang.org/tools/install) and it'll install the whole Rust toolchain.

## How to build and run
For a static image, just run `cargo run --release > something.ppm`.
Don't forget the redirect, because this currently outputs PPM data to your terminal's standard out.

If you want to generate an animation, comment out `src/main:44-47` and uncomment `src/main:50-71`, then run `cargo run --release`.
No redirection this time; the frames are generated in the `frames/` directory.

Run `cargo build` or `cargo build --release` if you just want to compile without running.

## How to test
Run `cargo test`.
Who would've guessed?

## What it looks like

640x480 with 100 samples per pixel and a maximum of 50 bounces takes ~14 seconds:

![raytracer](https://user-images.githubusercontent.com/30734384/94350133-34813f00-0019-11eb-9844-dedd5879e45a.png)

From left to right: transparent ball with refractive index of 1.52, diffuse ball, reflective ball with roughness of 0.3, and diffuse plane.

Here's an animation to show off reflection and refraction. Took about 28.5 minutes to render 120 frames, so averaging about 14.3 seconds per frame.

![animation](https://user-images.githubusercontent.com/30734384/94350134-35b26c00-0019-11eb-8874-517e1659ad21.gif)
