# Rust Raytracer

Figured a raytracer would be a fun way to learn how to use Rust.

## Dependencies
* Rust and Cargo. Just install `rustup` from [here](https://www.rust-lang.org/tools/install) and it'll install the whole Rust toolchain.

## How to build and run
Run `cargo build` or `cargo build --release` if you just want to compile without running.

In `src/main.rs` there are currently two available scenes you can render:
1. A basic scene with a diffuse sphere, a reflective sphere, a glass sphere, and a diffuse plane.
2. A [Cornell box](https://www.graphics.cornell.edu/online/box/data.html).
3. A thousand randomly generated spheres.

Just run `cargo run --release [1|2|3]` to render one of these.
(You can add `animate` at the end if you're rendering Scene 1 in particular to output a series of frames of an animated camera pivoting about the center of the scene to a subdirectory named `frames/`).

By default, this code will run on all available cores of your CPU.
You can go into `src/main.rs` and comment and uncomment particular lines of code (labelled appropriately) to change this.

## How to test
Run `cargo test` to run some basic sanity unit tests.

## What it looks like

The following images were produced (unless otherwise specified) using all 8-cores of my laptop's i7-8550U and a [bounding volume hierarchy](https://www.wikiwand.com/en/Bounding_volume_hierarchy) as an acceleration structure.

640x480 with 100 samples per pixel and a maximum of 50 bounces takes ~12 seconds:

![raytracer](https://user-images.githubusercontent.com/30734384/94495091-dbeba680-01be-11eb-9887-86fb676e1fe0.png)

From left to right: transparent ball with refractive index of 1.52, diffuse ball, reflective ball with roughness of 0.3, and diffuse plane.

Here's a lower-resolution (320x240) animation to show off reflection and refraction. The 640x480 version of this took about 35.7 minutes to render 120 frames, so averaging about 17.8 seconds per frame.

![animation](https://user-images.githubusercontent.com/30734384/94495092-dd1cd380-01be-11eb-9ada-fd34f5da4549.gif)

Here's a [Cornell box](https://www.graphics.cornell.edu/online/box/data.html) render.
512x512 with 10,000 samples per pixel and a maximum of 50 bounces (without using an acceleration structure) took ~1 hour and 32 minutes (as opposed to just over 4 hours and 15 minutes on only one of those cores):

![cornell](https://user-images.githubusercontent.com/30734384/94876378-8238e580-0425-11eb-9607-0edcb477728d.png)

And here's an image of 1,000 spheres, to show off how using an acceleration structure can dramatically improve performance.
This took about 24 seconds to render at 640x480 pixels, 100 samples, as opposed to about 3.65 minutes with a basic linear search:

![balls](https://user-images.githubusercontent.com/30734384/95020130-542aef80-0637-11eb-97d0-dc3e74f49b5a.png)

## How it works

TODO

## Resources

* [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
* [Physically Based Rendering: From Theory to Implementation](https://pbrt.org/)
