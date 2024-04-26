use image::{RgbImage, Rgb};

/// Draws a chessboard with a specified number of cells per side.
///
/// # Arguments
/// * `cell_count` - The number of cells along one side of the chessboard.
///
/// # Returns
/// A `RgbImage` of the chessboard.

pub fn draw_square(cell_count: u32) -> RgbImage {
    let size = 500;
    let square_size = size / cell_count;
    let mut image = RgbImage::new(size, size);

    // Iterate over each row and column to fill in the chessboard.
    for i in 0..cell_count {
        for j in 0..cell_count {
            let color = if (i + j) % 2 == 0 {
                Rgb([255, 255, 255]) // White
            } else {
                Rgb([0, 0, 0]) // Black
            };
            
            // Fill in each pixel of the current square with the determined color.
            for x in 0..square_size {
                for y in 0..square_size {
                    let px = (i * square_size + x) as u32;
                    let py = (j * square_size + y) as u32;
                    image.put_pixel(px, py, color);
                }
            }
        }
    }
    // Return the completed chessboard image.
    image
}

// Unit tests for the chessboard drawing function.
// I used three tests to verify the image size, square size, and checkerboard pattern.
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that the generated image has the correct dimensions.
    /// It checks that for a given number of cells, the chessboard image size is 500x500 pixels.
    #[test]
    fn test_image_size() {
        let cell_count = 10; // Example value
        let img = draw_square(cell_count);
        assert_eq!(img.dimensions(), (500, 500));
    }

    /// Tests that each square in the chessboard has the correct size and color.
    /// It verifies that each square is consistently sized based on the number of cells per side
    /// and alternates colors correctly in a checkerboard pattern.
    #[test]
    fn test_square_size() {
        let cell_count = 10;
        let img = draw_square(cell_count);
        let square_size = 500 / cell_count;

        // Check each square for correct size and color
        for x in 0..cell_count {
            let start_x = x * square_size;
            for y in 0..cell_count {
                let start_y = y * square_size;
                let expected_color = if (x + y) % 2 == 0 { Rgb([255, 255, 255]) } else { Rgb([0, 0, 0]) };

                // Check all pixels in the current square
                for i in start_x..start_x + square_size {
                    for j in start_y..start_y + square_size {
                        assert_eq!(*img.get_pixel(i, j), expected_color, "Mismatch at ({}, {})", i, j);
                    }
                }
            }
        }
    }

    /// Tests the checkerboard color pattern of the chessboard.
    /// This test uses a minimal board size of 2x2 cells to ensure that the colors alternate correctly
    /// across the chessboard, forming a valid checkerboard pattern.
    #[test]
    fn test_checkerboard_pattern() {
        let cell_count = 2; // Minimal non-trivial board
        let img = draw_square(cell_count);
        let square_size = 500 / cell_count;

        // Check that the checkerboard pattern alternates correctly
        for i in 0..cell_count {
            for j in 0..cell_count {
                let expected_color = if (i + j) % 2 == 0 { Rgb([255, 255, 255]) } else { Rgb([0, 0, 0]) };
                for x in 0..square_size {
                    for y in 0..square_size {
                        assert_eq!(*img.get_pixel((i * square_size + x) as u32, (j * square_size + y) as u32), expected_color);
                    }
                }
            }
        }
    }
}
