use image::DynamicImage;
use std::path::PathBuf;

use crate::{apply, imagetools};

fn generate_from_random(file1: PathBuf, output: PathBuf) {
    let image_main = image::open(&file1).expect("Image does not Exist.");
    let mut image_compare = image::DynamicImage::ImageRgb8(image::RgbImage::new(
        image_main.width(),
        image_main.height(),
    ));
    let mut value_previous = 10000000000000000;

    for i in 1..200001 {
        let size = num::clamp(100000. / (i as f32).powf(0.7), 1., 1000.);
        // let size = 200.0;

        let image_new = imagetools::place_random_rect(&image_compare, size);

        let value = imagetools::evaluate_likeness(&image_main, &image_new);

        if value < value_previous {
            println!("{} : value_previous {}: value {}", i, value_previous, value);
            image_compare = image_new;
            value_previous = value;
        }
    }
    image_compare.save(output).expect("Could not save.");
}

pub fn quad_setup(file1: PathBuf, output: PathBuf) {
    let image_main = image::open(&file1).expect("Image does not Exist.");

    let mut image = image::DynamicImage::ImageRgb8(image::RgbImage::new(
        image_main.width(),
        image_main.height(),
    ));

    let width = image_main.width();
    let height = image_main.height();
    let color = [0, 0, 0u8];

    quad_process_image(&image_main, &mut image, color, 0, 0, width, height);

    image.save(output).expect("Could not save.");
}

fn quad_process_image(
    image_main: &DynamicImage,
    image: &mut DynamicImage,
    mut color: [u8; 3],
    x_start: u32,
    y_start: u32,
    x_end: u32,
    y_end: u32,
) {
    let size = 5;
    if (x_end - x_start <= size) && (y_end - y_start <= size) {
        return;
    }

    // let mut value_previous = evaluate_likeness(image_main, image);
    let mut value_previous = 10000000000000000;
    let incr = 5;

    for i in 0..3 {
        for x in 0..2 {
            loop {
                let mut new_image = image.clone();

                if x == 0 {
                    color[i] += incr;
                } else {
                    color[i] -= incr;
                }

                apply::apply_color_region(&mut new_image, color, x_start, y_start, x_end, y_end);
                // let value = imagetools::evaluate_likeness3(image_main, &new_image);
                let value = imagetools::evaluate_likeness4(
                    image_main, &new_image, x_start, y_start, x_end, y_end,
                );

                if value < value_previous {
                    *image = new_image;
                    value_previous = value;
                } else {
                    if x == 0 {
                        color[i] -= incr;
                    } else {
                        color[i] += incr;
                    }

                    break;
                }
            }
        }
    }

    let x_mid = (x_start + x_end) / 2;
    let y_mid = (y_start + y_end) / 2;
    let c = color;

    // top left
    quad_process_image(image_main, image, c, x_start, y_start, x_mid, y_mid);
    // top right
    quad_process_image(image_main, image, c, x_mid, y_start, x_end, y_mid);
    // bottom left
    quad_process_image(image_main, image, c, x_start, y_mid, x_mid, y_end);
    // bottom right
    quad_process_image(image_main, image, c, x_mid, y_mid, x_end, y_end);
}
