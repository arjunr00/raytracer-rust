#[derive(Debug)]
pub struct Pixel {
    r: f64, g: f64, b: f64
}

impl Pixel {
    pub fn new(r: f64, g: f64, b: f64) -> Pixel {
        Pixel {
            r: f64::min(1.0, f64::max(0.0, r)),
            g: f64::min(1.0, f64::max(0.0, g)),
            b: f64::min(1.0, f64::max(0.0, b))
        }
    }
}

/// Creates a String containing a PPM representation of the image described by `pixels`
pub fn create_ppm(pixels: &Vec<Vec<Pixel>>, width: u32, height: u32, max_colors: u32) -> String {
    let mut ppm = format!("P3\n{} {}\n{}\n", width, height, max_colors);
    let total_pixels = width * height;

    for (i, row) in pixels.iter().enumerate() {
        for (j, pixel) in row.iter().enumerate() {
            let pixel_str = format!("{} {} {}\n",
                (255.999 * pixel.r) as u32,
                (255.999 * pixel.g) as u32,
                (255.999 * pixel.b) as u32);
            ppm.push_str(&pixel_str);

            let pixel_num = j + (i * width as usize);
            eprint!("\r{}/{} pixels rendered", pixel_num, total_pixels);
        }
    }

    eprintln!("\nDone.");
    ppm
}
