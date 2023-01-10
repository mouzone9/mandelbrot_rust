extern crate image;
extern crate num_complex;
use rayon::prelude::*;

fn compute_iterations_mandelbrot(complex_x:f32, complex_y:f32, max_iterations:usize) -> usize {
    let c = num_complex::Complex::new(complex_x, complex_y);
    let mut z = num_complex::Complex::new(0f32, 0f32);

    let mut nb_iterations = 0;
    while nb_iterations < max_iterations && z.norm() < 2.0 {
        z = z * z + c;
        nb_iterations += 1;
    }

    nb_iterations
}

pub fn compute_iterations(
    width: u32,
    height: u32,
    xa: f32,
    xb: f32,
    ya: f32,
    yb: f32,
    max_iterations:usize,
    ) -> Vec<usize> {
        (0..width * height)
            .into_par_iter()
            .map(|offset|{

                let image_x = offset % width;
                let image_y = offset / width;

                let complex_x = (image_x as f32) * (xb - xa) / (width as f32 - 1.0f32) + xa;
                let complex_y = (image_y as f32) * (yb - ya) / (height as f32 - 1.0f32) + ya;

                compute_iterations_mandelbrot(complex_x, complex_y, max_iterations)
        })
        .collect()
}

pub fn save_image(
    nb_iterations: &[usize],
    width: u32,
    height: u32,
    max_iterations: usize,
    path: &str,
) {
    let mut _imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in _imgbuf.enumerate_pixels_mut() {
        let i = nb_iterations[(y * width + x) as usize];

        let red: u8 = (((max_iterations as f32 - i as f32) / (max_iterations as f32)) * 255f32) as u8;
        let (green, blue) = (red, red);

        *pixel = image::Rgb([red as u8, green as u8, blue as u8]);

    }

    _imgbuf.save(path).unwrap();
}
