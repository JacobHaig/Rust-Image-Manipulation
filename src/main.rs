// This is because of the Crate name.
// please use snake_case when possible
#![allow(non_snake_case)]

use image::*;
use std::path::*;
use std::time::Instant;
use tokio::task;

#[tokio::main]
async fn main() {
    let in_folder = PathBuf::from(r"image\in");
    let out_folder = PathBuf::from(r"image\out");

    let start_time = Instant::now();

    let files = std::fs::read_dir(in_folder)
        .expect("Folder does not exist")
        .map(|d| d.unwrap().path())
        .collect::<Vec<PathBuf>>();

    let mut my_futures = Vec::new();
    for file in files {
        let out_file = out_folder.join(file.file_name().unwrap());
        my_futures.push(task::spawn(start(file.clone(), out_file)));
    }
    futures::future::join_all(my_futures).await;

    println!(
        "Converted all Images : {:?} ms",
        Instant::now().duration_since(start_time).as_millis()
    );
}

async fn start(file_in: PathBuf, file_out: PathBuf) {
    let start_time = Instant::now();

    let loaded_image = image::open(&file_in).expect("Image does not Exist");
    let new_image: DynamicImage = pixel_loop1(loaded_image);
    new_image.save(file_out).expect("Could not save");

    println!(
        "{:?} : {:?} ms",
        file_in,
        Instant::now().duration_since(start_time).as_millis()
    );
}

fn pixel_loop1(mut loaded_image: DynamicImage) -> DynamicImage {
    let rgb: &mut ImageBuffer<Rgb<u8>, Vec<u8>> = loaded_image.as_mut_rgb8().unwrap();
    let height = rgb.height();
    let width = rgb.width();

    for y in 0..height {
        for x in 0..width {
            let color: &mut Rgb<u8> = rgb.get_pixel_mut(x, y);

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

fn brighten(img: &mut Rgb<u8>) {
    let amount = 70;

    img[0] = clamp((img[0] as i16) + amount, 0, 255) as u8;
    img[1] = clamp((img[1] as i16) + amount, 0, 255) as u8;
    img[2] = clamp((img[2] as i16) + amount, 0, 255) as u8;
}

fn greyscale(colr: &mut Rgb<u8>) {
    let avg: u8 = ((colr[0] as i16 + colr[1] as i16 + colr[2] as i16) / 3) as u8;

    colr[0] = avg;
    colr[1] = avg;
    colr[2] = avg;
}

// GO VERSION For GreyScale
// None Par -- Converted all Images : 874 ms
// Some Par -- Converted all Images : 286 ms
// All  Par -- Converted all Images : 156 ms

// RUST VERSION For GreyScale
// None Par -- Converted all Images : 328 ms
// Some Par -- Converted all Images : 117 ms
// All  Par -- Converted all Images : ?
