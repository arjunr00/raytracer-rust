const OUT_WIDTH:  u32 = 640;
const OUT_HEIGHT: u32 = 480;
const MAX_COLORS: u32 = 255;

fn main() {
    print!("{}", raytracer::create_ppm(OUT_WIDTH, OUT_HEIGHT, MAX_COLORS));
}
