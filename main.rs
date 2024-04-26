/*
Entry Point Explanation for the main function:

Here's a breakdown of the key components and functionalities of the `main` function:
1. User Input:
   - The program prompts the user to choose between generating a chessboard or a Mandelbrot set by entering '1' or '2'.
   - It captures the user's input as a string and processes it to decide the subsequent action.

2. Match Statement:
   - The match statement is used to handle different inputs:
     - "1": Triggers the generation of a chessboard.
     - "2": Leads to further prompts to decide between generating a colored or grayscale Mandelbrot set.

3. Chessboard Generation:
   - If "1" is chosen, the user is asked to specify the number of cells per side for the chessboard.
   - The chessboard image is generated, saved to a file, and then displayed.

4. Mandelbrot Set Generation:
   - Choosing "2" initiates another loop asking for the type of Mandelbrot set to generate: colored or grayscale.
   - Depending on the user's subsequent choice, the program either uses default bounds or prompts for custom bounds to generate the set.
   - The image is then generated, saved, and displayed. If successful, the inner loop breaks.

This setup ensures that the program remains responsive and interactive.
*/


// Import necessary modules and traits from local modules and external crates
mod util;
mod chessboard;
mod mandelbrot;

use image::RgbImage;
use show_image::{create_window};
use crate::util::to_showable_image;
use crate::mandelbrot::{GrayscaleMap, ColoredColorMap, ColorMap};
use text_io::read;
use std::error::Error;

// Entry point of the program using show_image's macro for GUI applications
#[show_image::main]
fn main() -> Result<(), Box<dyn Error>> {
    // Infinite loop to keep asking for user input until valid input is given
    loop {
        println!("Choose an option by inputing either: 1 or 2:");
        println!("1: Generate a chessboard");
        println!("2: Generate a Mandelbrot set");

        let choice: String = read!();
        // Handle user input to determine the program's action
        match choice.trim() {
            "1" => {
                // Prompt for and read the number of cells for the chessboard
                println!("Enter the number of cells:");
                let cell_count: u32 = read!();
                let image = chessboard::draw_square(cell_count);
                let filename = format!("chessboard_{}x{}.png", cell_count, cell_count);
                // Save and display the generated chessboard image
                image.save(&filename)?;
                println!("Chessboard saved as {}", filename);
                display_image(image)?;
                break; // Exit loop after displaying and saving the image
            },
            "2" => {
                // Loop to ensure valid input for Mandelbrot set generation
                loop {
                    println!("Enter 'c' for colored or 'gs' for grayscale:");
                    let color_choice: String = read!("{}\n");

                    if color_choice.trim() == "c" || color_choice.trim() == "gs" {
                        let max_iterations = 100; // Or other appropriate value
                         // Determine the bounds for the Mandelbrot set based on user input
                        let bounds = if color_choice.trim() == "gs" {
                            println!("Enter the space to display in the format xmin;xmax;ymin;ymax:");
                            let input: String = read!("{}\n");
                            parse_bounds(&input).unwrap_or((-2.0, 2.0, -1.5, 1.5))
                        } else {
                            (-2.0, 2.0, -1.5, 1.5) // Default bounds for colored
                        };

                        let image = generate_mandelbrot_set(color_choice.clone(), max_iterations, bounds);
                        let filename = if color_choice.trim() == "c" {
                            "colored_mandelbrot.png"
                        } else {
                            "grayscale_mandelbrot.png"
                        };
                        image.save(filename)?;
                        println!("Mandelbrot set saved as {}", filename);
                        display_image(image)?;
                        break; // Exit loop after displaying and saving the image
                    } else {
                        println!("Invalid color option. Please enter 'c' for colored or 'gs' for grayscale.");
                    }
                }
                break; // Exit loop after handling Mandelbrot set
            },
            _ => {
                // Handle incorrect option entries
                println!("Invalid option, please enter '1' or '2'.");
            }
        }
    }

    Ok(())
}

// Helper function to parse spatial bounds from user input
fn parse_bounds(input: &str) -> Result<(f32, f32, f32, f32), &'static str> {
    let parts: Vec<&str> = input.split(';').collect();
    if parts.len() == 4 {
        let xmin = parts[0].parse::<f32>().map_err(|_| "Error parsing xmin")?;
        let xmax = parts[1].parse::<f32>().map_err(|_| "Error parsing xmax")?;
        let ymin = parts[2].parse::<f32>().map_err(|_| "Error parsing ymin")?;
        let ymax = parts[3].parse::<f32>().map_err(|_| "Error parsing ymax")?;
        Ok((xmin, xmax, ymin, ymax))
    } else {
        Err("Input must be in the format xmin;xmax;ymin;ymax")
    }
}

// Function to generate a Mandelbrot set image using specified color map and bounds
fn generate_mandelbrot_set(color_choice: String, max_iterations: u32, bounds: (f32, f32, f32, f32)) -> RgbImage {
    let color_map: Box<dyn ColorMap> = if color_choice.trim() == "c" {
        Box::new(ColoredColorMap::new(max_iterations))
    } else {
        Box::new(GrayscaleMap::new(max_iterations))
    };

    mandelbrot::generate_mandelbrot_set(800, 600, &*color_map, bounds)
}

// Function to display an image in a window using show_image crate
fn display_image(image: RgbImage) -> Result<(), Box<dyn Error>> {
    let window = create_window("Image Display", Default::default())?;
    window.set_image("image-001", to_showable_image(&image))?;
    window.wait_until_destroyed()?;
    Ok(())
}
