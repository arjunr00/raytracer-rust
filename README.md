# Rust Raytracer

Figured a raytracer would be a fun way to learn how to use Rust.
I'm using [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) as a guide.

## Dependencies
* Rust and Cargo. Just install `rustup` from [here](https://www.rust-lang.org/tools/install) and it'll install the whole Rust toolchain.

## How to build and run
Clone this repository and run `cargo run > something.ppm`.
It's that easy!
(Use a redirect because this currently outputs PPM data to your terminal's standard out.)

Run `cargo build` if you just want to compile without running.

## How to test
Run `cargo test`.
Who would've guessed?

## What it looks like

640x480 with 100 samples per pixel and a maximum of 50 bounces takes ~11 seconds:

![raytracer](https://user-images.githubusercontent.com/30734384/94325480-95067280-ff6c-11ea-9a51-5563a16be795.png)

From left to right: transparent ball with refractive index of 1.52, diffuse ball, reflective ball with roughness of 0.6.

Here's a lower-resolution (320x240) animation to show off reflection and refraction (without any DoF blur), taking ~6 minutes to render every frame:

![animation](https://user-images.githubusercontent.com/30734384/94319968-cf1a4900-ff59-11ea-9b2c-234b353027eb.gif)
