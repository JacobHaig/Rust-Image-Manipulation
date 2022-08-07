// Currently unused functions are hard coded.
#![allow(dead_code)]

use image::*;
use rand::Rng;
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
    // let in_folder = PathBuf::from("image/in");
    // let out_folder = PathBuf::from("image/out");

    // start_manipulation(in_folder, out_folder);

    let file1 = PathBuf::from("image/in/landscape.jpg");
    let output = PathBuf::from("image/out/new.jpg");

    quad_setup(file1, output);
}

// Start up function that will run a given file.
fn start_manipulation(in_folder: PathBuf, out_folder: PathBuf) {
    let files = get_images(&in_folder, &out_folder);

    files.par_iter().for_each(|file| {
        let file_in = file.to_path_buf();
        let file_out = out_folder.join(file.file_name().unwrap());

        let loaded_image = image::open(&file_in).expect("Image does not Exist.");

        let mut func = apply::greyscale_shaded_levels;
        let new_image: DynamicImage = apply(loaded_image, &mut func);

        new_image.save(file_out).expect("Could not save.");
    });
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

fn change_image(image: &DynamicImage, size: f32) -> DynamicImage {
    let mut image = image.clone();
    let max_size = size as u32;
    let mut rng = rand::thread_rng();

    let width = image.width();
    let height = image.height();

    let x_offset = rng.gen_range(0..width + 10) - 10;
    let y_offset = rng.gen_range(0..height + 10) - 10;
    let rect_width = rng.gen_range(0..max_size);
    let rect_height = rng.gen_range(0..max_size);

    let color = [
        rand::random::<u8>(),
        rand::random::<u8>(),
        rand::random::<u8>(),
    ];
    // let percentage = rand::random::<f32>();

    // let mut sub_image = image.sub_image(x, y, w, h);
    let rgb = image.as_mut_rgb8().unwrap();

    for a in 0..rect_width {
        for b in 0..rect_height {
            // println!("{}  {}", a, b);

            let pixel = rgb.get_pixel(
                num::clamp(x_offset + a, 0, width - 1),
                num::clamp(y_offset + b, 0, height - 1),
            );

            let new_pixel = image::Rgb::from(apply::apply_color(pixel.0, color));

            rgb.put_pixel(
                num::clamp(x_offset + a, 0, width - 1),
                num::clamp(y_offset + b, 0, height - 1),
                new_pixel,
            );
        }
    }

    image.clone()
}

fn compare_images(file1: PathBuf, file2: PathBuf, output: PathBuf) {
    let image_main = image::open(&file1).expect("Image does not Exist.");
    let mut image_compare = image::open(&file2).expect("Image does not Exist.");
    let mut value_previous = 10000000000000000;

    for i in 1..200001 {
        let size = num::clamp(100000. / (i as f32).powf(0.7), 1., 1000.);
        // let size = 200.0;

        let image_new = change_image(&image_compare, size);

        let value = evaluate_likeness(&image_main, &image_new);

        if value < value_previous {
            println!("{} : value_previous {}: value {}", i, value_previous, value);
            image_compare = image_new;
            value_previous = value;
        }
    }
    image_compare.save(output).expect("Could not save.");
}

fn evaluate_likeness(image: &DynamicImage, other_image: &DynamicImage) -> i64 {
    image
        .as_rgb8()
        .unwrap()
        .pixels()
        .zip(other_image.as_rgb8().unwrap().pixels())
        .map(|(p1, p2)| {
            p1.0.into_iter()
                .zip(p2.0.into_iter())
                .map(|(x, y)| {
                    let x = x as i64;
                    let y = y as i64;

                    num::abs(x - y) as i64
                })
                .sum::<i64>()
        })
        .sum()
}

fn quad_setup(file1: PathBuf, output: PathBuf) {
    let image_main = image::open(&file1).expect("Image does not Exist.");

    let mut image = image::DynamicImage::ImageRgb8(image::RgbImage::new(
        image_main.width(),
        image_main.height(),
    ));

    let width = image_main.width();
    let height = image_main.height();
    let color = [0, 0, 0u8];

    quad_image(&image_main, &mut image, color, 0, 0, width, height);

    image.save(output).expect("Could not save.");
}

fn quad_image(
    image_main: &DynamicImage,
    image: &mut DynamicImage,
    mut color: [u8; 3],
    x_start: u32,
    y_start: u32,
    x_end: u32,
    y_end: u32,
) {
    let size = 100;
    if (x_end - x_start <= size) || (y_end - y_start <= size) {
        return;
    }

    // let mut value_previous = evaluate_likeness(image_main, image);
    let mut value_previous = 10000000000000000;

    for i in 0..3 {
        loop {
            let mut new_image = image.clone();

            color[i] += 5;

            apply::apply_color_region(&mut new_image, color, x_start, y_start, x_end, y_end);
            let value = evaluate_likeness(image_main, &new_image);

            if value < value_previous {
                *image = new_image;
                value_previous = value;
            } else {
                color[i] -= 5;
                break;
            }
        }

        loop {
            let mut new_image = image.clone();

            color[i] -= 5;

            apply::apply_color_region(&mut new_image, color, x_start, y_start, x_end, y_end);
            let value = evaluate_likeness(image_main, &new_image);

            if value < value_previous {
                *image = new_image;
                value_previous = value;
            } else {
                color[i] += 5;
                break;
            }
        }
    }

    let x_mid = (x_start + x_end) / 2;
    let y_mid = (y_start + y_end) / 2;
    let c = color;

    // top left
    quad_image(image_main, image, c, x_start, y_start, x_mid, y_mid);
    // top right
    quad_image(image_main, image, c, x_mid, y_start, x_end, y_mid);
    // bottom left
    quad_image(image_main, image, c, x_start, y_mid, x_mid, y_end);
    // bottom right
    quad_image(image_main, image, c, x_mid, y_mid, x_end, y_end);
}
