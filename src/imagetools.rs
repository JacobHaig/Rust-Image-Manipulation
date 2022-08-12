use image::{DynamicImage, Rgb};
use rand::*;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::apply;

// Time: 398.09143 seconds
pub fn evaluate_likeness(image: &DynamicImage, other_image: &DynamicImage) -> i64 {
    image
        .as_rgb8()
        .unwrap()
        .pixels()
        .zip(other_image.as_rgb8().unwrap().pixels())
        .map(|(pixel_1, pixel_2)| {
            pixel_1
                .0
                .into_iter()
                .zip(pixel_2.0.into_iter())
                .map(|(x, y)| {
                    let x = x as i64;
                    let y = y as i64;

                    num::abs(x - y) as i64
                })
                .sum::<i64>()
        })
        .sum()
}

// Time: 1297.0497 seconds
pub fn evaluate_likeness2(image: &DynamicImage, other_image: &DynamicImage) -> i64 {
    let width = image.width().into();
    let height = image.height().into();

    let rgb_1 = image.as_rgb8().unwrap();
    let rgb_2 = other_image.as_rgb8().unwrap();

    let mut sum = 0;

    for a in 0..width {
        for b in 0..height {
            let pixel_1 = rgb_1.get_pixel(a, b);
            let pixel_2 = rgb_2.get_pixel(a, b);

            sum += num::abs(pixel_1.0[0] as i64 - pixel_2.0[0] as i64);
            sum += num::abs(pixel_1.0[1] as i64 - pixel_2.0[1] as i64);
            sum += num::abs(pixel_1.0[2] as i64 - pixel_2.0[2] as i64);
        }
    }
    sum
}

// Time: 283.66388 seconds
pub fn evaluate_likeness3(image: &DynamicImage, other_image: &DynamicImage) -> i64 {
    let width = image.width().into();
    let height = image.height().into();

    let rgb_1 = image.as_rgb8().unwrap();
    let rgb_2 = other_image.as_rgb8().unwrap();

    let sum = std::sync::atomic::AtomicI64::new(0);
    use std::sync::atomic::Ordering;

    (0..width).into_par_iter().for_each(|a| {
        let mut inter_sum = 0;
        (0..height).into_iter().for_each(|b| {
            let pixel_1 = rgb_1.get_pixel(a, b);
            let pixel_2 = rgb_2.get_pixel(a, b);

            inter_sum += num::abs(pixel_1.0[0] as i64 - pixel_2.0[0] as i64);
            inter_sum += num::abs(pixel_1.0[1] as i64 - pixel_2.0[1] as i64);
            inter_sum += num::abs(pixel_1.0[2] as i64 - pixel_2.0[2] as i64);
        });

        sum.fetch_add(inter_sum, Ordering::SeqCst);
    });
    sum.into_inner()
}

// Time: 94.293205 seconds at 25
// Time: 547.8267 seconds at 10
pub fn evaluate_likeness4(
    image: &DynamicImage,
    other_image: &DynamicImage,
    x_start: u32,
    y_start: u32,
    x_end: u32,
    y_end: u32,
) -> i64 {
    let rgb_1 = image.as_rgb8().unwrap();
    let rgb_2 = other_image.as_rgb8().unwrap();

    let mut sum = 0;

    for x in x_start..x_end {
        for y in y_start..y_end {
            let pixel_1 = rgb_1.get_pixel(x, y);
            let pixel_2 = rgb_2.get_pixel(x, y);

            sum += num::abs(pixel_1.0[0] as i64 - pixel_2.0[0] as i64);
            sum += num::abs(pixel_1.0[1] as i64 - pixel_2.0[1] as i64);
            sum += num::abs(pixel_1.0[2] as i64 - pixel_2.0[2] as i64);
        }
    }
    sum
}

// Time: 96.17871 seconds
pub fn evaluate_likeness5(
    image: &DynamicImage,
    other_image: &DynamicImage,
    x_start: u32,
    y_start: u32,
    x_end: u32,
    y_end: u32,
) -> i64 {
    let rgb_1 = image.as_rgb8().unwrap();
    let rgb_2 = other_image.as_rgb8().unwrap();

    let sum = std::sync::atomic::AtomicI64::new(0);
    use std::sync::atomic::Ordering;

    (x_start..x_end).into_par_iter().for_each(|x| {
        let mut inter_sum = 0;
        (y_start..y_end).into_iter().for_each(|y| {
            let pixel_1 = rgb_1.get_pixel(x, y);
            let pixel_2 = rgb_2.get_pixel(x, y);

            inter_sum += num::abs(pixel_1.0[0] as i64 - pixel_2.0[0] as i64);
            inter_sum += num::abs(pixel_1.0[1] as i64 - pixel_2.0[1] as i64);
            inter_sum += num::abs(pixel_1.0[2] as i64 - pixel_2.0[2] as i64);
        });

        sum.fetch_add(inter_sum, Ordering::SeqCst);
    });
    sum.into_inner()
}

pub fn place_random_rect(image: &DynamicImage, size: f32) -> DynamicImage {
    let mut image = image.clone();
    let max_size = size as u32;

    let width = image.width();
    let height = image.height();

    let mut rng = rand::thread_rng();
    let x_offset = rng.gen_range(0..width + 10) - 10;
    let y_offset = rng.gen_range(0..height + 10) - 10;
    let rect_width = rng.gen_range(0..max_size);
    let rect_height = rng.gen_range(0..max_size);

    let color = [
        rand::random::<u8>(),
        rand::random::<u8>(),
        rand::random::<u8>(),
    ];

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

// Apply a given function to an image.
pub fn map(mut loaded_image: DynamicImage, func: &mut dyn FnMut(&mut Rgb<u8>)) -> DynamicImage {
    loaded_image
        .as_mut_rgb8()
        .unwrap()
        .pixels_mut()
        .for_each(|p| func(p));

    loaded_image
}
