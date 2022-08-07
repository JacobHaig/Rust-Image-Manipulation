use image::{DynamicImage, Rgb};
use rand::*;

use crate::apply;

pub fn evaluate_likeness(image: &DynamicImage, other_image: &DynamicImage) -> i64 {
    image
        .as_rgb8()
        .unwrap()
        .pixels()
        .zip(other_image.as_rgb8().unwrap().pixels())
        .map(|(pixel_1, pixel_2)| {
            pixel_1.0.into_iter()
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
