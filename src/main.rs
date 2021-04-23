// This is because of the Crate name.
// please use snake_case when possible
#![allow(non_snake_case)]

use image::*;
use std::{cmp::max, path::*};
use std::{cmp::min, time::Instant};
use tokio::task;

#[tokio::main]
async fn main() {
    let cur_dir = std::env::current_dir().unwrap();
    let in_folder = PathBuf::from(r"image\in");
    let out_folder = PathBuf::from(r"image\out");

    let start_time = Instant::now();

    if std::fs::read_dir(&in_folder).is_err() {
        std::fs::create_dir_all(cur_dir.join(&in_folder)).unwrap();
    }
    if std::fs::read_dir(&out_folder).is_err() {
        std::fs::create_dir_all(cur_dir.join(&out_folder)).unwrap();
    }

    let files = std::fs::read_dir(&in_folder)
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

            colored_shaded_levels(color);
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

fn greyscale_average(colr: &mut Rgb<u8>) {
    let avg: u8 = ((colr[0] as i16 + colr[1] as i16 + colr[2] as i16) / 3) as u8;

    colr[0] = avg;
    colr[1] = avg;
    colr[2] = avg;
}

fn greyscale_high_low_average(colr: &mut Rgb<u8>) {
    let high = max(max(colr[0], colr[1]), colr[2]);
    let low = min(min(colr[0], colr[1]), colr[2]);

    let high_low_average: u8 = ((high as u16 + low as u16) / 2) as u8;

    colr[0] = high_low_average;
    colr[1] = high_low_average;
    colr[2] = high_low_average;
}

fn desaturate(colr: &mut Rgb<u8>) {
    use lerp::Lerp;
    let percentage = 0.50;

    let avg: f32 = ((colr[0] as i16 + colr[1] as i16 + colr[2] as i16) / 3) as f32;

    colr[0] = (colr[0] as f32).lerp(avg as f32, percentage) as u8;
    colr[1] = (colr[1] as f32).lerp(avg as f32, percentage) as u8;
    colr[2] = (colr[2] as f32).lerp(avg as f32, percentage) as u8;
}

// The normalize function takes a value between a range of numbers and normalize
// it between a new range of numbers. For instance, if the range is 10..20 with
// a value of 15, and the new range is 0..100, the new value will be 50.
fn normalize<T, I: 'static>(value: T, from_min: I, from_max: I, to_min: I, to_max: I) -> I
where
    T: num::cast::AsPrimitive<I>,
    I: Copy + num::Num,
{
    to_min + ((value.as_() - from_min) * (to_max - to_min)) / (from_max - from_min)
}

fn greyscale_shaded_levels(colr: &mut Rgb<u8>) {
    let num_levels: f32 = 8.;

    let avg: u8 = ((colr[0] as i16 + colr[1] as i16 + colr[2] as i16) / 3) as u8;

    let normal = normalize(avg as f32, 0 as f32, 255., 0., num_levels as f32) as u8;

    let shaded = clamp(normal as f32 * (255. / (num_levels - 1.)), 0., 255.) as u8;

    colr[0] = shaded;
    colr[1] = shaded;
    colr[2] = shaded;
}

fn colored_shaded_levels(colr: &mut Rgb<u8>) {
    let num_levels: f32 = 3.;
 
    let r_normal = normalize(colr[0] as f32, 0 as f32, 255., 0., num_levels as f32) as u8;
    let g_normal = normalize(colr[1] as f32, 0 as f32, 255., 0., num_levels as f32) as u8;
    let b_normal = normalize(colr[2] as f32, 0 as f32, 255., 0., num_levels as f32) as u8;

    let r_shaded = clamp(r_normal as f32 * (255. / (num_levels - 1.)), 0., 255.) as u8;
    let g_shaded = clamp(g_normal as f32 * (255. / (num_levels - 1.)), 0., 255.) as u8;
    let b_shaded = clamp(b_normal as f32 * (255. / (num_levels - 1.)), 0., 255.) as u8;

    colr[0] = r_shaded;
    colr[1] = g_shaded;
    colr[2] = b_shaded;
}

// GO VERSION For GreyScale
// None Par -- Converted all Images : 874 ms
// Some Par -- Converted all Images : 286 ms
// All  Par -- Converted all Images : 156 ms

// RUST VERSION For GreyScale
// None Par -- Converted all Images : 328 ms
// Some Par -- Converted all Images : 117 ms
// All  Par -- Converted all Images : ?
