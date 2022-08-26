#![feature(test)]
#![cfg(test)]

extern crate maize;
extern crate test;
use test::Bencher;

// #[cfg(test)]
// use maize::TEST;
// #[cfg(test)]
// use crate::bres_octant_colors;

#[bench]
fn bench_arc_iter(b: &mut Bencher) {
    b.iter(|| maize::bench::draw_arc_iter());
}

#[bench]
fn bench_image_pixel(b: &mut Bencher) {
    b.iter(|| image::RgbaImage::from_pixel(400, 400, image::Rgba([255, 255, 255, 255])));
}

#[bench]
fn bench_image_new(b: &mut Bencher) {
    b.iter(|| image::RgbaImage::new(400, 400));
}

#[bench]
fn bench_image_from_vec(b: &mut Bencher) {
    b.iter(|| image::RgbaImage::from_vec(400, 400, Vec::from([255; 400 * 400 * 4])));
}

#[bench]
#[ignore]
// This is really expensive compared to from_pixel
fn bench_image_from_vec_iter(b: &mut Bencher) {
    b.iter(|| {
        image::RgbaImage::from_vec(
            400,
            400,
            Vec::from_iter([0, 0, 0, 255].iter().cycle().take(400 * 400 * 4).copied()),
        )
    });
}
