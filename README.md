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

This is the image produced by redirecting the output of `src/main.rs`:

![raytracer](https://user-images.githubusercontent.com/30734384/94205531-7c755a00-fe91-11ea-9f66-7da2ed7dfd34.png)
