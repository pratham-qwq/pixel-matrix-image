use image::open;
use std::env;
use std::process;

fn main() -> Result<(), image::ImageError> {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <image_path> <percentage>", args[0]);
        process::exit(1);
    }

    let image_path = &args[1];
    let percentage_str = &args[2];

    // Parse percentage
    let percentage = match percentage_str.parse::<f32>() {
        Ok(p) if p >= 0.0 && p <= 100.0 => p,
        _ => {
            eprintln!("Invalid percentage. Please enter a value between 0 and 100.");
            process::exit(1);
        }
    };

    // Open the image file
    let img = match open(image_path) {
        Ok(image) => image,
        Err(_) => {
            eprintln!("Failed to open image: {}", image_path);
            process::exit(1);
        }
    };

    // Convert the image to RGBA format
    let mut rgba_img = img.to_rgba8();

    // Get image dimensions
    let (width, height) = rgba_img.dimensions();

    // Calculate the total number of pixels
    let total_pixels = (width * height) as f32;
    let num_pixels_to_convert = (total_pixels * percentage / 100.0) as u64;

    // Calculate the number of rows and columns needed to create the desired number of black dots
    let num_dots = num_pixels_to_convert as usize;
    let dot_spacing = (total_pixels / num_dots as f32).sqrt().ceil() as u32; // Grid spacing based on the number of dots

    // Ensure we have a sensible spacing value
    let step_x = if dot_spacing > 0 { dot_spacing } else { 1 };
    let step_y = if dot_spacing > 0 { dot_spacing } else { 1 };

    // Create the dot matrix by modifying pixels at regular intervals
    let mut pixels_modified = 0;
    for x in (0..width).step_by(step_x as usize) {
        for y in (0..height).step_by(step_y as usize) {
            // Get the pixel at the coordinates
            let pixel = rgba_img.get_pixel_mut(x, y);

            // Set the pixel to black (dot)
            *pixel = image::Rgba([0, 0, 0, 255]);

            // Track how many pixels have been modified
            pixels_modified += 1;
            if pixels_modified >= num_dots {
                break;
            }
        }
        if pixels_modified >= num_dots {
            break;
        }
    }

    // Save the modified image
    match rgba_img.save("output.png") {
        Ok(_) => println!("Image processed successfully. Output saved as output.png"),
        Err(_) => {
            eprintln!("Failed to save the output image.");
            process::exit(1);
        }
    }

    Ok(())
}
