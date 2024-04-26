// Import necessary image handling and gradient functionalities from external crates.
use image::{Rgb, RgbImage};
use colorgrad::Gradient;

// Define a trait to specify behaviors for color mapping in different scenarios.
pub trait ColorMap {
    fn color(&self, i: u32) -> Rgb<u8>;
    fn get_max_iterations(&self) -> u32;
}
// A structure to handle grayscale mapping with a specific maximum iteration count.
pub struct GrayscaleMap {
    max_iterations: u32,
}

// Implementation block for GrayscaleMap.
impl GrayscaleMap {
    pub fn new(max_iterations: u32) -> Self {
        // Constructor to create a new GrayscaleMap with a specified max_iterations.
        Self { max_iterations }
    }
}

// Implement the ColorMap trait for GrayscaleMap.
impl ColorMap for GrayscaleMap {
    fn color(&self, i: u32) -> Rgb<u8> {
        if i == self.max_iterations {
            Rgb([0, 0, 0]) // Points inside the set are black
        } else {
            let intensity = (i as f32 / self.max_iterations as f32 * 255.0).round() as u8;
            Rgb([intensity, intensity, intensity]) // Grayscale based on iteration count
        }
    }
    // Getter for max_iterations.
    fn get_max_iterations(&self) -> u32 {
        self.max_iterations
    }
}

// A structure to handle colored mapping using a gradient, supporting a specific max iteration count.
pub struct ColoredColorMap {
    max_iterations: u32,
    // Gradient to use for coloring outside the set.
    gradient: Gradient,
}
// Implementation block for ColoredColorMap.
impl ColoredColorMap {
    pub fn new(max_iterations: u32) -> Self {
        Self {
            max_iterations,
            gradient: colorgrad::turbo(), // Utilizes the turbo gradient from colorgrad crate
        }
    }
}

// Implement the ColorMap trait for ColoredColorMap.
impl ColorMap for ColoredColorMap {
    // Define how to color a pixel based on the iteration count for a colored image.
    fn color(&self, i: u32) -> Rgb<u8> {
        if i >= self.max_iterations {
            Rgb([0, 0, 0]) // Points inside the set are black
        } else {
            let t = i as f64 / (self.max_iterations - 1) as f64; // Normalized iteration value
            let color = self.gradient.at(t).to_rgba8();
            Rgb([color[0], color[1], color[2]])
        }
    }

    fn get_max_iterations(&self) -> u32 {
        self.max_iterations
    }
}

// Function to generate a Mandelbrot set image based on the provided ColorMap and dimensions.
pub fn generate_mandelbrot_set(width: u32, height: u32, color_map: &dyn ColorMap, bounds: (f32, f32, f32, f32)) -> RgbImage {
    let mut img = RgbImage::new(width, height);
    let (xmin, xmax, ymin, ymax) = bounds;
    let scale_x = (xmax - xmin) / width as f32;
    let scale_y = (ymax - ymin) / height as f32;

    // Iterate over each pixel in the image.
    for px in 0..width {
        for py in 0..height {
            let x0 = px as f32 * scale_x + xmin;
            let y0 = py as f32 * scale_y + ymin;
            let (mut x, mut y, mut iteration) = (0.0, 0.0, 0);

            // Compute whether the point (x, y) escapes the Mandelbrot set within max_iterations.
            while x * x + y * y <= 4.0 && iteration < color_map.get_max_iterations() {
                let xtemp = x * x - y * y + x0;
                y = 2.0 * x * y + y0;
                x = xtemp;
                iteration += 1;
            }
            // Set the pixel color based on the number of iterations and the colormap.
            img.put_pixel(px, py, color_map.color(iteration));
        }
    }
    // Return the completed image.
    img
}

