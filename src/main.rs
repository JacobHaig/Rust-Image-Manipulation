// Currently unused functions are hard coded.
#![allow(dead_code)]

use image::*;
use std::path::*;
use util::{check_folder_exists, get_files};

mod apply;
mod imagetools;
mod processes;
mod util;

fn main() {
    check_folder_exists(&PathBuf::from("image/in"));
    check_folder_exists(&PathBuf::from("image/out"));

    // start_manipulation(in_folder, out_folder);

    let file1 = PathBuf::from("image/in/landscape.jpg");
    let output = PathBuf::from("image/out/new.jpg");

    let start = std::time::Instant::now();
    processes::quad_setup(file1, output);

    println!("Time: {} seconds", start.elapsed().as_secs_f32());
}

// Start up function that will run a given file.
fn map_folder(in_folder: PathBuf, out_folder: PathBuf) {
    let files = get_files(&in_folder);

    files.iter().for_each(|file| {
        let file_in = file.to_path_buf();
        let file_out = out_folder.join(file.file_name().unwrap());

        let loaded_image = image::open(&file_in).expect("Image does not Exist.");

        let mut func = apply::greyscale_shaded_levels;
        let new_image: DynamicImage = imagetools::map(loaded_image, &mut func);

        new_image.save(file_out).expect("Could not save.");
    });
}
