use image::*;
use std::{cmp::max, cmp::min};

use crate::util;

pub fn brighten(img: &mut Rgb<u8>) {
    let amount = 70;

    img[0] = util::clamp((img[0] as i16) + amount, 0, 255) as u8;
    img[1] = util::clamp((img[1] as i16) + amount, 0, 255) as u8;
    img[2] = util::clamp((img[2] as i16) + amount, 0, 255) as u8;
}

pub fn greyscale_average(colr: &mut Rgb<u8>) {
    let avg: u8 = ((colr[0] as i16 + colr[1] as i16 + colr[2] as i16) / 3) as u8;

    colr[0] = avg;
    colr[1] = avg;
    colr[2] = avg;
}

pub fn greyscale_high_low_average(colr: &mut Rgb<u8>) {
    let high = max(max(colr[0], colr[1]), colr[2]);
    let low = min(min(colr[0], colr[1]), colr[2]);

    let high_low_average: u8 = ((high as u16 + low as u16) / 2) as u8;

    colr[0] = high_low_average;
    colr[1] = high_low_average;
    colr[2] = high_low_average;
}

pub fn desaturate(colr: &mut Rgb<u8>) {
    use lerp::Lerp;
    let percentage = 0.50;

    let avg: f32 = ((colr[0] as i16 + colr[1] as i16 + colr[2] as i16) / 3) as f32;

    colr[0] = (colr[0] as f32).lerp(avg as f32, percentage) as u8;
    colr[1] = (colr[1] as f32).lerp(avg as f32, percentage) as u8;
    colr[2] = (colr[2] as f32).lerp(avg as f32, percentage) as u8;
}

pub fn greyscale_shaded_levels(colr: &mut Rgb<u8>) {
    let num_levels: f32 = 20.;

    let avg: u8 = ((colr[0] as i16 + colr[1] as i16 + colr[2] as i16) / 3) as u8;
    let normal = util::normalize(avg as f32, 0 as f32, 255., 0., num_levels as f32) as u8;

    let shaded = util::clamp(normal as f32 * (255. / (num_levels - 1.)), 0., 255.) as u8;

    colr[0] = shaded;
    colr[1] = shaded;
    colr[2] = shaded;
}

pub fn colored_shaded_levels(colr: &mut Rgb<u8>) {
    let num_levels: f32 = 10.0;

    let r_normal = util::normalize(colr[0] as f32, 0., 255., 0., num_levels) as u8;
    let g_normal = util::normalize(colr[1] as f32, 0., 255., 0., num_levels) as u8;
    let b_normal = util::normalize(colr[2] as f32, 0., 255., 0., num_levels) as u8;

    let r_shaded = util::clamp(r_normal as f32 * (255. / (num_levels - 1.)), 0., 255.) as u8;
    let g_shaded = util::clamp(g_normal as f32 * (255. / (num_levels - 1.)), 0., 255.) as u8;
    let b_shaded = util::clamp(b_normal as f32 * (255. / (num_levels - 1.)), 0., 255.) as u8;

    colr[0] = r_shaded;
    colr[1] = g_shaded;
    colr[2] = b_shaded;
}

pub fn apply_color(mut pixel: [u8; 3], color: [u8; 3], percentage: f32) -> [u8; 3] {
    use lerp::Lerp;

    // pixel[0] = (pixel[0] as f32).lerp(color[0] as f32, percentage) as u8;
    // pixel[1] = (pixel[1] as f32).lerp(color[1] as f32, percentage) as u8;
    // pixel[2] = (pixel[2] as f32).lerp(color[2] as f32, percentage) as u8;
    pixel[0] = color[0];
    pixel[1] = color[1];
    pixel[2] = color[2];

    pixel
}
