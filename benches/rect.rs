use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

use maize::Orth;
use maize::RectCell;

pub fn blank(size: u32) -> image::RgbaImage {
    image::RgbaImage::from_pixel(size, size, image::Rgba([255, 255, 255, 255]))
}

fn rect_grid(c: &mut Criterion) {
    c.bench_function("rect_grid", |b| {
        b.iter(|| {
            let grid: Orth<RectCell> = Orth::new(6, 6);
            grid.render()
        })
    });
}

fn rect_new_image(c: &mut Criterion) {
    c.bench_function("rect_render_new_image", |b| {
        b.iter(|| {
            let grid: Orth<RectCell> = Orth::new(6, 6);
            grid.render()
        })
    });
}

fn rect_render_only(c: &mut Criterion) {
    c.bench_function("rect_render_only", |b| {
        b.iter_batched(
            || Orth::new(6, 6),
            |grid| grid.render(),
            BatchSize::SmallInput,
        )
    });
}

fn rect_same_image(c: &mut Criterion) {
    let grid: Orth<RectCell> = Orth::new(6, 6);
    c.bench_function("rect_same_image", |b| b.iter(|| grid.render()));
}

criterion_group!(
    rect,
    rect_grid,
    rect_new_image,
    rect_render_only,
    rect_same_image
);
criterion_main!(rect);
