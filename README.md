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

640x480 with 100 samples per pixel and a maximum of 50 bounces takes ~15 seconds:

![raytracer](https://user-images.githubusercontent.com/30734384/94374576-46c3b180-00db-11eb-8ac8-6561cb1f42fe.png)

From left to right: transparent ball with refractive index of 1.52, diffuse ball, reflective ball with roughness of 0.3, and diffuse plane.

Here's an animation to show off reflection and refraction. Took about 35.7 minutes to render 120 frames, so averaging about 17.8 seconds per frame.

![animation](https://user-images.githubusercontent.com/30734384/94374578-47f4de80-00db-11eb-9119-79268db7baa7.gif)
