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

640x480 with 100 samples per pixel and a maximum of 50 bounces takes ~8 seconds:

![raytracer](https://user-images.githubusercontent.com/30734384/94311973-1c8eba00-ff4a-11ea-9f90-04cd8413a833.png)

From left to right: transparent ball with refractive index of 1.52, reflective ball with roughness of 0.6, diffuse ball.
