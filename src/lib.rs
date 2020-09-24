use vec::ColorRGB;

pub mod math;
pub mod ray;
pub mod vec;

/// Creates a String containing a PPM representation of a single pixel
pub fn write_pixel(pixel: &ColorRGB) -> String {
    format!("{} {} {}\n",
        (255.999 * pixel[vec::Color::R]) as u32,
        (255.999 * pixel[vec::Color::G]) as u32,
        (255.999 * pixel[vec::Color::B]) as u32
    )
}

/// Creates a String containing a PPM representation of the image described by `pixels`
pub fn create_ppm(pixels: &Vec<Vec<ColorRGB>>, width: u32, height: u32, max_colors: u32) -> String {
    let mut ppm = format!("P3\n{} {}\n{}\n", width, height, max_colors);
    let total_pixels = width * height;

    for (i, row) in pixels.iter().enumerate() {
        for (j, pixel) in row.iter().enumerate() {
            ppm.push_str(&write_pixel(pixel));

            let pixel_num = j + 1 + (i * width as usize);
            eprint!("\r{}/{} pixels rendered", pixel_num, total_pixels);
        }
    }

    eprintln!("\nDone.");
    ppm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_basic_ppm() {
        let expected_output = "P3\n16 16\n255\n255 0 63\n255 17 63\n255 34 63\n255 51 63\n255 68 63\n255 85 63\n255 102 63\n255 119 63\n255 136 63\n255 153 63\n255 170 63\n255 187 63\n255 204 63\n255 221 63\n255 238 63\n255 255 63\n255 0 63\n255 17 63\n255 34 63\n255 51 63\n255 68 63\n255 85 63\n255 102 63\n255 119 63\n255 136 63\n255 153 63\n255 170 63\n255 187 63\n255 204 63\n255 221 63\n255 238 63\n255 255 63\n238 0 63\n238 17 63\n238 34 63\n238 51 63\n238 68 63\n238 85 63\n238 102 63\n238 119 63\n238 136 63\n238 153 63\n238 170 63\n238 187 63\n238 204 63\n238 221 63\n238 238 63\n238 255 63\n221 0 63\n221 17 63\n221 34 63\n221 51 63\n221 68 63\n221 85 63\n221 102 63\n221 119 63\n221 136 63\n221 153 63\n221 170 63\n221 187 63\n221 204 63\n221 221 63\n221 238 63\n221 255 63\n204 0 63\n204 17 63\n204 34 63\n204 51 63\n204 68 63\n204 85 63\n204 102 63\n204 119 63\n204 136 63\n204 153 63\n204 170 63\n204 187 63\n204 204 63\n204 221 63\n204 238 63\n204 255 63\n187 0 63\n187 17 63\n187 34 63\n187 51 63\n187 68 63\n187 85 63\n187 102 63\n187 119 63\n187 136 63\n187 153 63\n187 170 63\n187 187 63\n187 204 63\n187 221 63\n187 238 63\n187 255 63\n170 0 63\n170 17 63\n170 34 63\n170 51 63\n170 68 63\n170 85 63\n170 102 63\n170 119 63\n170 136 63\n170 153 63\n170 170 63\n170 187 63\n170 204 63\n170 221 63\n170 238 63\n170 255 63\n153 0 63\n153 17 63\n153 34 63\n153 51 63\n153 68 63\n153 85 63\n153 102 63\n153 119 63\n153 136 63\n153 153 63\n153 170 63\n153 187 63\n153 204 63\n153 221 63\n153 238 63\n153 255 63\n136 0 63\n136 17 63\n136 34 63\n136 51 63\n136 68 63\n136 85 63\n136 102 63\n136 119 63\n136 136 63\n136 153 63\n136 170 63\n136 187 63\n136 204 63\n136 221 63\n136 238 63\n136 255 63\n119 0 63\n119 17 63\n119 34 63\n119 51 63\n119 68 63\n119 85 63\n119 102 63\n119 119 63\n119 136 63\n119 153 63\n119 170 63\n119 187 63\n119 204 63\n119 221 63\n119 238 63\n119 255 63\n102 0 63\n102 17 63\n102 34 63\n102 51 63\n102 68 63\n102 85 63\n102 102 63\n102 119 63\n102 136 63\n102 153 63\n102 170 63\n102 187 63\n102 204 63\n102 221 63\n102 238 63\n102 255 63\n85 0 63\n85 17 63\n85 34 63\n85 51 63\n85 68 63\n85 85 63\n85 102 63\n85 119 63\n85 136 63\n85 153 63\n85 170 63\n85 187 63\n85 204 63\n85 221 63\n85 238 63\n85 255 63\n68 0 63\n68 17 63\n68 34 63\n68 51 63\n68 68 63\n68 85 63\n68 102 63\n68 119 63\n68 136 63\n68 153 63\n68 170 63\n68 187 63\n68 204 63\n68 221 63\n68 238 63\n68 255 63\n51 0 63\n51 17 63\n51 34 63\n51 51 63\n51 68 63\n51 85 63\n51 102 63\n51 119 63\n51 136 63\n51 153 63\n51 170 63\n51 187 63\n51 204 63\n51 221 63\n51 238 63\n51 255 63\n34 0 63\n34 17 63\n34 34 63\n34 51 63\n34 68 63\n34 85 63\n34 102 63\n34 119 63\n34 136 63\n34 153 63\n34 170 63\n34 187 63\n34 204 63\n34 221 63\n34 238 63\n34 255 63\n17 0 63\n17 17 63\n17 34 63\n17 51 63\n17 68 63\n17 85 63\n17 102 63\n17 119 63\n17 136 63\n17 153 63\n17 170 63\n17 187 63\n17 204 63\n17 221 63\n17 238 63\n17 255 63\n";

        let width = 16;
        let height = 16;
        let max_colors = 255;
        let mut pixels: Vec<Vec<ColorRGB>> = vec![];

        for i in 0..height {
            pixels.push(vec![]);
            for j in 0..width {
                let r = ((height - i) as f64) / f64::from(width - 1);
                let g = (j as f64) / f64::from(height - 1);
                let b = 0.25;

                let pixel = ColorRGB::new(
                    f64::min(1.0, f64::max(0.0, r)),
                    f64::min(1.0, f64::max(0.0, g)),
                    f64::min(1.0, f64::max(0.0, b))
                );

                pixels[i as usize].push(pixel);
            }
        }

        let output = create_ppm(&pixels, width, height, max_colors);

        assert_eq!(output, expected_output);
    }
}
