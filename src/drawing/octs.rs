/// Add center coordinates to a specified point
pub(super) fn center<T: std::ops::Add<Output = T>>(center: (T, T), coords: (T, T)) -> (T, T) {
    (center.0 + coords.0, center.1 + coords.1)
}

/// Translate coordinates used in the octant iterators to a specified octant
pub(super) fn trans_oct<T>(oct: u8, coords: (T, T), cent: (T, T)) -> (T, T)
where
    T: std::ops::Add<Output = T> + std::ops::Neg<Output = T>,
{
    trans_fn(coords, cent, &oct_fn(oct))
}

/// Translate coordinates used in the octant iterators to a specified octant by specifying a translation
/// function (this avoids having to lookup the function for each call)
pub(super) fn trans_fn<F, T>(coords: (T, T), cent: (T, T), f: &F) -> (T, T)
where
    T: std::ops::Add<Output = T> + std::ops::Neg<Output = T>,
    F: Fn((T, T)) -> (T, T),
{
    center(cent, f(coords))
}

/// Return the function to translate coordinates used in the octant iterators to a specified octant
pub(super) const fn oct_fn<T: std::ops::Neg<Output = T>>(oct: u8) -> fn((T, T)) -> (T, T) {
    match oct {
        0 => bres_to_o1,
        1 => bres_to_o2,
        2 => bres_to_o3,
        3 => bres_to_o4,
        4 => bres_to_o5,
        5 => bres_to_o6,
        6 => bres_to_o7,
        7 => bres_to_o8,
        _ => bres_to_o1,
    }
}

/// Translate coordinates in a specified octant to the coordinates used by the octant iterators
pub(super) fn rev_trans_oct<T>(oct: u8, coords: (T, T), cent: (T, T)) -> (T, T)
where
    T: std::ops::Sub<Output = T> + std::ops::Neg<Output = T>,
{
    rev_trans_fn(coords, cent, &rev_oct_fn(oct))
}

/// Translate coordinates in a specified octant to coordinates used by the octant iterators by specifying
/// a translation function (this avoids having to lookup the function for each call)
pub(super) fn rev_trans_fn<F, T>(coords: (T, T), cent: (T, T), f: &F) -> (T, T)
where
    T: std::ops::Sub<Output = T> + std::ops::Neg<Output = T>,
    F: Fn((T, T)) -> (T, T),
{
    f(rev_center(cent, coords))
}

/// Subtract center coordinates from a specified point
pub(super) fn rev_center<T: std::ops::Sub<Output = T>>(center: (T, T), coords: (T, T)) -> (T, T) {
    (coords.0 - center.0, coords.1 - center.1)
}

/// Return the function to translate coordinates in a specified octant to the coordinates used by the octant iterators
pub(super) const fn rev_oct_fn<T: std::ops::Neg<Output = T>>(oct: u8) -> fn((T, T)) -> (T, T) {
    match oct {
        0 => o1_to_bres,
        1 => o1_to_bres,
        2 => o3_to_bres,
        3 => o3_to_bres,
        4 => o5_to_bres,
        5 => o5_to_bres,
        6 => o7_to_bres,
        7 => o7_to_bres,
        _ => o1_to_bres,
    }
}

/// Translate coordinates used in the octant iterators to a specified octant
pub(super) fn trans_to_iter<T>(oct: u8, coords: (T, T), cent: (T, T)) -> (T, T)
where
    T: std::ops::Add<Output = T> + std::ops::Neg<Output = T>,
{
    trans_to_iter_fn(coords, cent, &oct_to_iter_fn(oct))
}

/// Translate coordinates used in the octant iterators to a specified octant by specifying a translation
/// function (this avoids having to lookup the function for each call)
pub(super) fn trans_to_iter_fn<F, T>(coords: (T, T), cent: (T, T), f: &F) -> (T, T)
where
    T: std::ops::Add<Output = T> + std::ops::Neg<Output = T>,
    F: Fn((T, T)) -> (T, T),
{
    center(cent, f(coords))
}

/// Return the function to translate coordinates used in the octant iterators to a specified octant
pub(super) const fn oct_to_iter_fn<T: std::ops::Neg<Output = T>>(oct: u8) -> fn((T, T)) -> (T, T) {
    match oct {
        0 => bres_to_o1,
        1 => bres_to_o1,
        2 => bres_to_o3,
        3 => bres_to_o3,
        4 => bres_to_o5,
        5 => bres_to_o5,
        6 => bres_to_o7,
        7 => bres_to_o7,
        _ => bres_to_o1,
    }
}

/// Convert coordinates from `BresenhamIter` to coordinates in the octant 1
pub(super) fn bres_to_o1<T: std::ops::Neg<Output = T>>(c: (T, T)) -> (T, T) {
    (c.1, -c.0)
}
pub(super) fn bres_to_o2<T: std::ops::Neg<Output = T>>(c: (T, T)) -> (T, T) {
    (c.0, -c.1)
}
pub(super) fn bres_to_o3<T: std::ops::Neg<Output = T>>(c: (T, T)) -> (T, T) {
    (-c.0, -c.1)
}
pub(super) fn bres_to_o4<T: std::ops::Neg<Output = T>>(c: (T, T)) -> (T, T) {
    (-c.1, -c.0)
}
pub(super) fn bres_to_o5<T: std::ops::Neg<Output = T>>(c: (T, T)) -> (T, T) {
    (-c.1, c.0)
}
pub(super) fn bres_to_o6<T: std::ops::Neg<Output = T>>(c: (T, T)) -> (T, T) {
    (-c.0, c.1)
}
pub(super) fn bres_to_o7<T: std::ops::Neg<Output = T>>(c: (T, T)) -> (T, T) {
    c
}
pub(super) fn bres_to_o8<T: std::ops::Neg<Output = T>>(c: (T, T)) -> (T, T) {
    (c.1, c.0)
}

pub(super) fn o1_to_bres<T: std::ops::Neg<Output = T>>(c: (T, T)) -> (T, T) {
    (-c.1, c.0) // negative sign needed to be swapped from y to x
}
pub(super) fn o2_to_bres<T: std::ops::Neg<Output = T>>(c: (T, T)) -> (T, T) {
    (c.0, -c.1)
}
pub(super) fn o3_to_bres<T: std::ops::Neg<Output = T>>(c: (T, T)) -> (T, T) {
    (-c.0, -c.1)
}
pub(super) fn o4_to_bres<T: std::ops::Neg<Output = T>>(c: (T, T)) -> (T, T) {
    (-c.1, -c.0)
}
pub(super) fn o5_to_bres<T: std::ops::Neg<Output = T>>(c: (T, T)) -> (T, T) {
    (c.1, -c.0) // needed negative y instead of positive
}
pub(super) fn o6_to_bres<T: std::ops::Neg<Output = T>>(c: (T, T)) -> (T, T) {
    (-c.0, c.1)
}
pub(super) fn o7_to_bres<T: std::ops::Neg<Output = T>>(c: (T, T)) -> (T, T) {
    c
}
pub(super) fn o8_to_bres<T: std::ops::Neg<Output = T>>(c: (T, T)) -> (T, T) {
    (c.1, c.0)
}

pub(crate) mod tests {
    use super::*;
    use crate::drawing::cir::{self, MPCircle};
    #[cfg(test)]
    use std::f64::consts::PI;

    const C1: image::Rgba<u8> = image::Rgba([255, 0, 0, 255]);
    const C2: image::Rgba<u8> = image::Rgba([252, 98, 3, 255]);
    const C3: image::Rgba<u8> = image::Rgba([0, 255, 0, 255]);
    const C4: image::Rgba<u8> = image::Rgba([3, 252, 177, 255]);
    const C5: image::Rgba<u8> = image::Rgba([3, 3, 255, 255]);
    const C6: image::Rgba<u8> = image::Rgba([227, 3, 252, 255]);
    const C7: image::Rgba<u8> = image::Rgba([0, 0, 0, 255]);
    const C8: image::Rgba<u8> = image::Rgba([150, 150, 150, 255]);
    fn map_colors(oct: u8) -> image::Rgba<u8> {
        match oct {
            0 => C1,
            1 => C2,
            2 => C3,
            3 => C4,
            4 => C5,
            5 => C6,
            6 => C7,
            7 => C8,
            _ => C1,
        }
    }

    fn draw_mp<F>(image: &mut image::RgbaImage, r: i32, c: (i32, i32), f: F, col: image::Rgba<u8>)
    where
        F: Fn((i32, i32)) -> (i32, i32),
    {
        cir::MPCircle::new(r)
            .map(|xy| trans_fn(xy, c, &f))
            .for_each(|(x, y)| image.put_pixel(x as u32, y as u32, col));
    }
    fn draw_mp_at<F>(
        image: &mut image::RgbaImage,
        r: i32,
        start: (f64, f64),
        c: (i32, i32),
        f: F,
        col: image::Rgba<u8>,
    ) where
        F: Fn((i32, i32)) -> (i32, i32),
    {
        cir::MPCircle::at(start.0, start.1, r)
            .map(|xy| trans_fn(xy, c, &f))
            .for_each(|(x, y)| image.put_pixel(x as u32, y as u32, col));
    }
    // `start` and `end` must be converted to coordinates used for octant iterators
    fn draw_mp_segment<F>(
        image: &mut image::RgbaImage,
        r: i32,
        start: (f64, f64),
        end: (f64, f64),
        c: (i32, i32),
        f: F,
        col: image::Rgba<u8>,
    ) where
        F: Fn((i32, i32)) -> (i32, i32),
    {
        cir::MPCirleUntil::at(start.0, start.1, end, r)
            .map(|xy| trans_fn(xy, c, &f))
            .for_each(|(x, y)| image.put_pixel(x as u32, y as u32, col));
    }

    const IMG_SIZE: u32 = 600;
    fn guidelines() -> image::RgbaImage {
        let mut image = image::RgbaImage::from_vec(
            IMG_SIZE,
            IMG_SIZE,
            Vec::from([255; (IMG_SIZE * IMG_SIZE) as usize * 4]),
        )
        .unwrap();
        // Draw guide lines
        imageproc::drawing::draw_line_segment_mut(
            &mut image,
            (IMG_SIZE as f32 / 2.0, 0.0),
            (IMG_SIZE as f32 / 2.0, IMG_SIZE as f32),
            image::Rgba([252, 190, 3, 255]),
        );
        imageproc::drawing::draw_line_segment_mut(
            &mut image,
            (0.0, IMG_SIZE as f32 / 2.0),
            (IMG_SIZE as f32, IMG_SIZE as f32 / 2.0),
            image::Rgba([252, 190, 3, 255]),
        );
        imageproc::drawing::draw_line_segment_mut(
            &mut image,
            (0.0, 0.0),
            (IMG_SIZE as f32, IMG_SIZE as f32),
            image::Rgba([255, 242, 206, 255]),
        );
        imageproc::drawing::draw_line_segment_mut(
            &mut image,
            (0.0, IMG_SIZE as f32),
            (IMG_SIZE as f32, 0.0),
            image::Rgba([255, 242, 206, 255]),
        );
        image
    }

    #[cfg(test)]
    #[test]
    fn midpoint_circle_segment() -> Result<(), image::ImageError> {
        // let mut image =
        //     image::RgbaImage::from_vec(600, 600, Vec::from([255; 600 * 600 * 4])).unwrap();
        let mut image: image::RgbaImage = guidelines();
        let center: (i32, i32) = (IMG_SIZE as i32 / 2, IMG_SIZE as i32 / 2);
        let centerf = (IMG_SIZE as f64 / 2.0, IMG_SIZE as f64 / 2.0);
        let r1: i32 = 140;
        let r2: i32 = 180;
        let octant = 1;

        let oct_rads = PI / 4.0;
        let torus = crate::drawing::torus::TorusSlice::new(
            centerf,
            oct_rads * (octant as f64),
            oct_rads * (octant as f64 + 1.5),
            r1 as f64,
            r2 as f64,
        );

        let ra = rev_trans_oct(octant, torus.a(), centerf);
        let rb = rev_trans_oct(octant, torus.b(), centerf);
        let rc = rev_trans_oct(octant, torus.c(), centerf);
        let rd = rev_trans_oct(octant, torus.d(), centerf);

        // Draw guide lines
        imageproc::drawing::draw_line_segment_mut(
            &mut image,
            (300.0, 0.0),
            (300.0, 600.0),
            image::Rgba([252, 190, 3, 255]),
        );
        imageproc::drawing::draw_line_segment_mut(
            &mut image,
            (0.0, 300.0),
            (600.0, 300.0),
            image::Rgba([252, 190, 3, 255]),
        );
        imageproc::drawing::draw_line_segment_mut(
            &mut image,
            (0.0, 0.0),
            (600.0, 600.0),
            image::Rgba([255, 242, 206, 255]),
        );
        imageproc::drawing::draw_line_segment_mut(
            &mut image,
            (0.0, 600.0),
            (600.0, 0.0),
            image::Rgba([255, 242, 206, 255]),
        );

        // draw full segment in black to contrast the segments with
        draw_mp(
            &mut image,
            r1,
            center,
            oct_fn(octant),
            image::Rgba([200, 200, 200, 255]),
        );
        // draw full segment in black to contrast the segments with
        draw_mp(
            &mut image,
            r2,
            center,
            oct_fn(octant),
            image::Rgba([200, 200, 200, 255]),
        );

        // inner torus segment
        draw_mp_segment(
            &mut image,
            r1,
            rc,
            ra,
            center,
            oct_fn(octant),
            image::Rgba([255, 0, 0, 255]),
        );

        // outer torus segment
        draw_mp_segment(
            &mut image,
            r2,
            rb,
            rd,
            center,
            oct_fn(octant),
            image::Rgba([0, 255, 0, 255]),
        );

        image.put_pixel(
            torus.a().0.floor() as u32,
            torus.a().1.floor() as u32,
            image::Rgba([128, 128, 128, 255]),
        );
        image.put_pixel(
            torus.b().0.floor() as u32,
            torus.b().1.floor() as u32,
            image::Rgba([128, 128, 128, 255]),
        );
        image.put_pixel(
            torus.c().0.floor() as u32,
            torus.c().1.floor() as u32,
            image::Rgba([128, 128, 128, 255]),
        );
        image.put_pixel(
            torus.d().0.floor() as u32,
            torus.d().1.floor() as u32,
            image::Rgba([128, 128, 128, 255]),
        );

        draw_raw_octant(&mut image, r1, image::Rgba([255, 0, 255, 255]));
        draw_raw_octant(&mut image, r2, image::Rgba([0, 255, 255, 255]));

        image.put_pixel(
            ra.0.floor() as u32,
            ra.1.floor() as u32,
            image::Rgba([0, 0, 0, 255]),
        );
        image.put_pixel(
            rb.0.floor() as u32,
            rb.1.floor() as u32,
            image::Rgba([0, 0, 0, 255]),
        );
        image.put_pixel(
            rc.0.floor() as u32,
            rc.1.floor() as u32,
            image::Rgba([0, 0, 0, 255]),
        );
        image.put_pixel(
            rd.0.floor() as u32,
            rd.1.floor() as u32,
            image::Rgba([0, 0, 0, 255]),
        );

        image.save("images/tests/mp_octant_segment.png")
    }

    /// Don't apply center or translate octant coordinates
    fn draw_raw_octant(image: &mut image::RgbaImage, r: i32, col: image::Rgba<u8>) {
        MPCircle::new(r).for_each(|(x, y)| image.put_pixel(x as u32, y as u32, col));
    }

    #[test]
    pub fn arc_iter() -> Result<(), image::ImageError> {
        draw_arc_iter().save("images/tests/arc_iter.png")
    }
    pub fn draw_arc_iter() -> image::RgbaImage {
        let mut image: image::RgbaImage = guidelines();
        let center: (u32, u32) = (IMG_SIZE / 2, IMG_SIZE / 2);
        let centerf = (IMG_SIZE as f64 / 2.0, IMG_SIZE as f64 / 2.0);
        let r1: i32 = 180;
        let r2: i32 = 250;

        let oct_rads = std::f64::consts::PI / 4.0;
        let torus = crate::drawing::torus::TorusSlice::new(
            centerf,
            oct_rads * 6.1,
            oct_rads * 7.0,
            r1 as f64,
            r2 as f64,
        );

        let iter = cir::ArcIter::new(torus.ccw(), torus.cw(), r1, center);
        cir::draw_iter(&mut image, iter, image::Rgba([255, 0, 0, 255]));
        image
    }

    #[test]
    fn backwards_arc() -> Result<(), image::ImageError> {
        let mut image: image::RgbaImage = guidelines();
        let center: (u32, u32) = (IMG_SIZE / 2, IMG_SIZE / 2);
        let centerf = (IMG_SIZE as f64 / 2.0, IMG_SIZE as f64 / 2.0);
        let r: i32 = 150;

        {
            let mut x: i32 = 0;
            let mut y: i32 = r;
            let mut d: i32 = 1 - r;
            let mut i = 0;
            loop {
                if x >= y {
                    break;
                }
                image.put_pixel(x as u32, y as u32, image::Rgba([255, 0, 0, 255]));
                // println!("x={} y={}", x, y);
                x += 1;
                if d < 0 {
                    d += 2 * x + 1;
                } else {
                    y -= 1;
                    d += 2 * (x - y) + 1;
                }
                i += 1;
            }
            {
                let a = std::f64::consts::PI / 4.0;
                let r = r as f64;
                let fx = (r * a.cos()).round() as u32;
                let fy = (r * a.sin()).round() as u32;
                // image.put_pixel(fx, fy, image::Rgba([0, 255, 0, 255]));
                // println!("fx={} fy={}", fx, fy);
            }
            println!("i={}", i);
        }

        {
            let a = std::f64::consts::PI / 4.0;
            let mut i = 0;
            let rf = r as f64;
            let xf = rf * a.cos();
            let yf = rf * a.sin();
            let mut x = xf.round() as i32;
            let mut y = yf.round() as i32;
            let df = (xf.round() + 1.0).powi(2) + (yf.round() - 0.5).powi(2) - r.pow(2) as f64;
            let mut d = df.round() as i32;
            loop {
                image.put_pixel(x as u32, y as u32, image::Rgba([0, 255, 0, 255]));
                // should return coordinates if y == 0 but no more coordinates afterwards
                if y == 0 {
                    break;
                }
                if x >= y {
                    y -= 1;
                    if d <= 0 {
                        d += 2 * y + 1;
                    } else {
                        x += 1;
                        d += 2 * (y - x) + 1;
                    }
                }
                i += 1;
            }
            println!("i={}", i);
        }

        image.save("images/tests/backwards_arc.png")
    }

    #[test]
    fn full_quadrant_arc() -> Result<(), image::ImageError> {
        let mut image: image::RgbaImage = guidelines();
        let c: (i32, i32) = (IMG_SIZE as i32 / 2, IMG_SIZE as i32 / 2);
        let r: i32 = 150;

        imageproc::drawing::draw_hollow_circle_mut(&mut image, c, r, image::Rgba([0, 0, 255, 255]));
        // for o in 0..8 {
        //     draw_mp(&mut image, r, c, oct_fn(o), image::Rgba([0, 0, 255, 255]));
        // }
        // return image.save("images/tests/full_quad_arc.png");
        let mut x: i32 = 0;
        let mut y: i32 = r;
        let mut d: i32 = 1 - r;
        let mut quad: u8 = 3;

        // p = (x+1)² + (y - ½)² - r²

        loop {
            // println!("x={} y={}", x, y);
            image.put_pixel(
                (x + c.0) as u32,
                (y + c.1) as u32,
                image::Rgba([255, 0, 0, 255]),
            );

            if quad == 3 {
                // if x >= 0 && y >= 0 {
                println!("x={} y={} d={}", x, y, d);
                if y == 0 {
                    x = r;
                    y = 0;
                    d = 1 - r;
                    quad = 0;
                }
                if x < y {
                    // octect 7
                    x += 1;
                    if d <= 0 {
                        d += 2 * x + 1;
                        // d = ((x + 1) * (x + 1)) + (y as f64 - 0.5).powi(2).round() as i32 - (r * r);
                    } else {
                        y -= 1;
                        d += 2 * (x - y) + 1;
                        // d = ((x + 1) * (x + 1)) + (y as f64 - 0.5).powi(2).round() as i32 - (r * r);
                    }
                } else {
                    // octect 8
                    y -= 1;
                    if d <= 0 {
                        d += 2 * y + 1;
                        // d = ((x + 1) * (x + 1)) + (y as f64 - 0.5).powi(2).round() as i32 - (r * r);
                    } else {
                        x += 1;
                        d += 2 * (y - x) + 1;
                        // d = ((x + 1) * (x + 1)) + (y as f64 - 0.5).powi(2).round() as i32 - (r * r);
                    }
                }
            // } else if x >= 0 && y < 0 {
            } else if quad == 0 {
                // octect 0
                if x == 0 {
                    x = 0;
                    y = -r;
                    d = 1 - r;
                    quad = 1;
                }
                if y > -x {
                    y -= 1;
                    if d <= 0 {
                        d += 2 * -y - 1;
                    } else {
                        x -= 1;
                        d += 2 * (-y - x) + 1;
                    }
                } else {
                    x -= 1;
                    if d <= 0 {
                        d += 2 * x - 1;
                    } else {
                        y -= 1;
                        d += 2 * (x - -y) + 1;
                    }
                }
            } else if quad == 1 {
                if y == 0 {
                    x = -r;
                    y = 0;
                    d = 1 - r;
                    quad = 2;
                }
                if x > y {
                    x -= 1;
                    if d <= 0 {
                        d += 2 * -x - 1;
                    } else {
                        y += 1;
                        d += 2 * (-x - -y) + 1;
                    }
                } else {
                    y += 1;
                    if d <= 0 {
                        d += 2 * -y - 1;
                    } else {
                        x -= 1;
                        d += 2 * (-y - -x) + 1;
                    }
                }
            } else if quad == 2 {
                if x == 0 {
                    break;
                }
                if -x > y {
                    y += 1;
                    if d <= 0 {
                        d += 2 * y + 1;
                    } else {
                        x += 1;
                        d += 2 * (y - -x) + 1;
                    }
                } else {
                    x += 1;
                    if d <= 0 {
                        d += 2 * -x + 1;
                    } else {
                        y += 1;
                        d += 2 * (-x - y) + 1;
                    }
                }
            } else {
                println!("invalid quadrant");
                break;
            }
        }

        image.save("images/tests/full_quad_arc.png")
    }
}
