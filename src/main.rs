// This is because of the Crate name.
// please use snake_case when possible
#![allow(non_snake_case)]

use image::*;
use std::path::*;

fn main() {
    let in_folder = PathBuf::from(r"image\in");
    let out_folder = PathBuf::from(r"image\out");

    let start_time = std::time::Instant::now();

    let files = std::fs::read_dir(in_folder)
        .expect("Folder does not exist")
        .map(|d| d.unwrap().path())
        .collect::<Vec<PathBuf>>();

    for file in &files {
        let out_file = out_folder.join(file.file_name().unwrap());
        start(&file, &out_file);
    }

    println!(
        "Converted all Images : {:?} ms",
        std::time::Instant::now()
            .duration_since(start_time)
            .as_millis()
    )
}

fn start(file_in: &PathBuf, file_out: &PathBuf) {
    let loaded_image = image::open(&file_in).expect("Image does not Exist");

    let new_image: DynamicImage = pixel_loop(loaded_image);

    new_image.save(file_out).expect("Could not save");
}

fn pixel_loop(mut loaded_image: DynamicImage) -> DynamicImage {
    let rgb = loaded_image.as_mut_rgb8().unwrap();

    for y in 0..rgb.height() {
        for x in 0..rgb.width() {
            let color = rgb.get_pixel_mut(x, y);

            greyscale(color);
        }
    }

    loaded_image
}

fn clamp<T>(value: T, min: T, max: T) -> T
where
    T: std::cmp::PartialOrd,
{
    //assert!(min <= max);
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn brighten(img: &mut Rgb<u8>) {
    let amount = 70;

    img[0] = clamp((img[0] as i16) + amount, 0, 255) as u8;
    img[1] = clamp((img[1] as i16) + amount, 0, 255) as u8;
    img[2] = clamp((img[2] as i16) + amount, 0, 255) as u8;
}

pub fn greyscale(img: &mut Rgb<u8>) {
    let avg: u8 = ((img[0] as i16 + img[1] as i16 + img[2] as i16) / 3) as u8;

    img[0] = avg;
    img[1] = avg;
    img[2] = avg;
}

// GO VERSION For GreyScale -- Non Par
// Converted all Images : 874 ms

// RUST VERSION For GreyScale -- Non Par
// Converted all Images : 328 ms
