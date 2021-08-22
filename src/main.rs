//! An example of generating julia fractals.
//mkdir julia
// cargo run --release
extern crate image;
use num::complex::Complex;
use rayon::prelude::*;

fn main() {
    let imgx = 600; //image resolution x
    let imgy = 600; //image resolution y

    let scalex = 3.0 / imgx as f64;
    let scaley = 3.0 / imgy as f64;

    const IMAGE_CNT: usize = 400; //image files count
    const RE_START: f64 = -0.8;
    const RE_STOP: f64 = -0.4;
    const IM_START: f64 = 0.1;
    const IM_STOP: f64 = 0.6;
    let mut c_array: [Complex<f64>; IMAGE_CNT] = [Complex::new(RE_START, IM_START); IMAGE_CNT];

    let delta = Complex::new(
        (RE_STOP - RE_START) / IMAGE_CNT as f64,
        (IM_STOP - IM_START) / IMAGE_CNT as f64,
    );

    (0..IMAGE_CNT)
        .into_iter()
        .for_each(|i| c_array[i] += delta * (i as f64));

    (0..IMAGE_CNT).into_par_iter().for_each(|n| {//IMAGE_CNT parallel iter 
        let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            //Initialize color
            let r = (0.3 * x as f64) as u8;
            let b = (0.3 * y as f64) as u8;
            *pixel = image::Rgb([r, 0, b]);
        }
        //1 image file generation
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let cx = y as f64 * scalex - 1.5;
            let cy = x as f64 * scaley - 1.5;
            let mut z = Complex::new(cx, cy);
            let mut i = 0;

            while i < 255 && z.norm_sqr() <= 4.0 {
                z = z * z + c_array[n];
                i += 1;
            }
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb([data[0], i as u8, data[2]]);
        }
        // Save the image as “julia/foo-0000.png”, the format is deduced from the path
        let s = format!("julia/foo-{:04}.png", n);
        imgbuf.save(&s).unwrap();
    })
}
