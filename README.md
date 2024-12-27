# Pixel Matrix Image Transformer

This Rust program applies a dot matrix effect to an image, turning a certain percentage of the image's pixels into black dots arranged in a grid pattern. The dots are distributed evenly across the image. The percentage of black dots is configurable via the command-line argument.

## Features

- Reads an image file (JPEG, PNG, etc.).
- Applies a dot matrix transformation, where a specified percentage of the pixels are turned into black dots.
- Supports flexible dot matrix patterns with grid spacing based on the percentage of black dots.

## Examples
Original Image:
![ponyo](https://github.com/pratham-qwq/pixel-matrix-image/blob/master/examples/ponyo.jpg?raw=true)
With 25% pixels turned to black in a matrix
![ponyo_25matrix](https://github.com/pratham-qwq/pixel-matrix-image/blob/master/examples/ponyo_25matrix.png?raw=true)

## Requirements

- Rust 1.60 or later
- The `image` crate, which is used to handle image processing.

## Installation

### Clone the Repository

To use the program, clone this repository:

```bash
git clone https://github.com/your-username/pixel-matrix-image.git
cd pixel-matrix-image
```
### Build the Project

Ensure that you have Rust installed. You can check this by running:
```bash
rustc --version
```
If you haven't installed Rust, you can follow the instructions [here](https://www.rust-lang.org/tools/install)
Once you have Rust installed, build the project:
```bash
cargo build --release
```
This will compile the program and create an executable in the `target/release/` directory.

## Usage 

### Command-Line Arguments
The program accepts two command-line arguments:
  1. `<image_path>`: Path to the image file you want to process.
  2. `<percentage>`: The percentage of the image's pixels that will be turned into black dots. It must be a number between 0 and 100.
### Example
```bash
cargo run image.png 25
```
This will process `image.png`, turning 25% of the image's pixels into black dots arranged in a grid, and save the transformed image as `output.png` in the current directory.

## How It Works
 - The program reads the image file from the specified path and loads it into memory.
 - It calculates the number of pixels to convert into black dots based on the provided percentage.
 - It then applies the dot matrix effect by placing black dots in a grid pattern at regular intervals across the image.
## Error Handling
 - If the image fails to load (e.g., the file path is invalid or the image is not supported), an error message is displayed.
 - If the program fails to save the modified image (due to issues like file permissions), an error message is also displayed.

## License
This project is licensed under the MIT License.
