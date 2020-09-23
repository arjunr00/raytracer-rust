use raytracer::vec;

const OUT_WIDTH:  u32 = 256;
const OUT_HEIGHT: u32 = 256;
const MAX_COLORS: u32 = 255;

fn main() {
    let mut pixels: Vec<Vec<vec::ColorRGB>> = vec![];

    for i in 0..OUT_HEIGHT {
        pixels.push(vec![]);
        for j in 0..OUT_WIDTH {
            let r = ((OUT_HEIGHT - i) as f64) / f64::from(OUT_WIDTH - 1);
            let g = (j as f64) / f64::from(OUT_HEIGHT - 1);
            let b = 0.25;

            let pixel = vec::ColorRGB::new(
                f64::min(1.0, f64::max(0.0, r)),
                f64::min(1.0, f64::max(0.0, g)),
                f64::min(1.0, f64::max(0.0, b))
            );

            pixels[i as usize].push(pixel);
        }
    }

    print!("{}", raytracer::create_ppm(&pixels, OUT_WIDTH, OUT_HEIGHT, MAX_COLORS));
}
