# Rust Raytracer

Figured a raytracer would be a fun way to learn how to use Rust.
I'm using [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) as a guide.

## Dependencies
* Rust and Cargo. Just install `rustup` from [here](https://www.rust-lang.org/tools/install) and it'll install the whole Rust toolchain.

## How to build and run
**NOTE: These instructions are outdated, will update soon.**
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

![raytracer](https://user-images.githubusercontent.com/30734384/94495091-dbeba680-01be-11eb-9887-86fb676e1fe0.png)

From left to right: transparent ball with refractive index of 1.52, diffuse ball, reflective ball with roughness of 0.3, and diffuse plane.

Here's a lower-resolution (320x240) animation to show off reflection and refraction. The 640x480 version of this took about 35.7 minutes to render 120 frames, so averaging about 17.8 seconds per frame.

![animation](https://user-images.githubusercontent.com/30734384/94495092-dd1cd380-01be-11eb-9ada-fd34f5da4549.gif)

Here's a [Cornell box](https://www.graphics.cornell.edu/online/box/data.html) render (approximately---the sides of the box in this render are in fact perfectly perpendicular, and the "outside" in front of the box offers some illumination, which is why it looks a little off in my opinion).
512x512 with 10,000 samples per pixel and a maximum of 500 bounces took ~4 hours and 15 minutes:

![cornell](https://user-images.githubusercontent.com/30734384/94492370-26b5f000-01b8-11eb-87b6-974427258016.png)
