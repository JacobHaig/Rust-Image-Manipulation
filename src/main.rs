// This is because of the Crate name.
// please use snake_case when possible

// Currently unused functions are hard coded.
#![allow(dead_code)]

use image::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::path::*;

mod apply;
mod util;

fn main() {
    let cur_dir = std::env::current_dir().unwrap();
    let in_folder = PathBuf::from(r"image\in");
    let out_folder = PathBuf::from(r"image\out");

    // If image in and out folders dont exist, create them.
    if std::fs::read_dir(&in_folder).is_err() {
        std::fs::create_dir_all(cur_dir.join(&in_folder)).unwrap();
    }
    if std::fs::read_dir(&out_folder).is_err() {
        std::fs::create_dir_all(cur_dir.join(&out_folder)).unwrap();
    }

    // Get the paths of all the input files.
    let files = std::fs::read_dir(&in_folder)
        .expect("Folder does not exist")
        .map(|d| d.unwrap().path())
        .collect::<Vec<PathBuf>>();

    // Preform the image manipulation on each image in parallel
    files.par_iter().for_each(|file| {
        start_manipulation(
            file.to_path_buf(),
            out_folder.join(file.file_name().unwrap()),
        )
    });
}

// Start up function that will run a given file.
fn start_manipulation(file_in: PathBuf, file_out: PathBuf) {
    let loaded_image = image::open(&file_in).expect("Image does not Exist");
    let new_image: DynamicImage = per_pixel_loop(loaded_image);
    new_image.save(file_out).expect("Could not save");
}

// Pixel loop loops over every pixel and applies a given function to it.
fn per_pixel_loop(mut loaded_image: DynamicImage) -> DynamicImage {
    let rgb: &mut ImageBuffer<Rgb<u8>, Vec<u8>> = loaded_image.as_mut_rgb8().unwrap();
    let height = rgb.height();
    let width = rgb.width();

    // Loop over every pixel and apply the function to it.
    for y in 0..height {
        for x in 0..width {
            let color: &mut Rgb<u8> = rgb.get_pixel_mut(x, y);

            apply::colored_shaded_levels(color);
        }
    }

    loaded_image
}
