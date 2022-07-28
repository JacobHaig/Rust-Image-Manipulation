// Currently unused functions are hard coded.
#![allow(dead_code)]

use image::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::path::*;

mod apply;
mod util;

fn get_images(in_folder: &PathBuf, out_folder: &PathBuf) -> Vec<PathBuf> {
    let cur_dir = std::env::current_dir().unwrap();

    // If image in and out folders dont exist, create them.
    if std::path::Path::exists(in_folder) {
        std::fs::create_dir_all(cur_dir.join(in_folder)).unwrap();
    }
    if std::path::Path::exists(out_folder) {
        std::fs::create_dir_all(cur_dir.join(out_folder)).unwrap();
    }

    // Get the paths of all the input files.
    std::fs::read_dir(&in_folder)
        .unwrap()
        .map(|d| d.unwrap().path())
        .collect::<Vec<PathBuf>>()
}

fn main() {
    let in_folder = PathBuf::from("image/in");
    let out_folder = PathBuf::from("image/out");
    let files = get_images(&in_folder, &out_folder);

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
    let loaded_image = image::open(&file_in).expect("Image does not Exist.");

    let mut func = apply::colored_shaded_levels;
    let new_image: DynamicImage = apply(loaded_image, &mut func);

    new_image.save(file_out).expect("Could not save.");
}

// Apply a given function to an image.
fn apply(mut loaded_image: DynamicImage, func: &mut dyn FnMut(&mut Rgb<u8>)) -> DynamicImage {
    loaded_image
        .as_mut_rgb8()
        .unwrap()
        .pixels_mut()
        .for_each(|p| func(p));

    loaded_image
}
