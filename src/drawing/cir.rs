/// Iterators always start at either the vertical or horizontal axis and move towards the diagonal axis

const OCT_RADS: f64 = std::f64::consts::PI / 4.0;

// could (possibly?) later change T to T: IntoIterator to allow calling by passing a node reference where the node implements IntoIterator
pub(crate) fn draw_iter<T: Iterator<Item = (u32, u32)>>(
    image: &mut image::RgbaImage,
    iter: T,
    color: image::Rgba<u8>,
) {
    iter.for_each(|(x, y)| image.put_pixel(x, y, color));
}

/// The radian passed to this function must be a valid radian (less than or equal to 2*PI)
pub(crate) const fn rad_to_oct(rad: f64) -> u8 {
    match rad {
        f if f <= OCT_RADS => 0,
        f if f <= OCT_RADS * 2.0 => 1,
        f if f <= OCT_RADS * 3.0 => 2,
        f if f <= OCT_RADS * 4.0 => 3,
        f if f <= OCT_RADS * 5.0 => 4,
        f if f <= OCT_RADS * 6.0 => 5,
        f if f <= OCT_RADS * 7.0 => 6,
        f if f <= OCT_RADS * 8.0 => 7,
        _ => 0,
    }
}

const fn is_inc(oct: u8) -> bool {
    match oct {
        1 | 3 | 5 | 7 => false,
        _ => true,
    }
}

#[derive(Clone, Debug)]
struct ArcTrans {
    oct: u8,
    c: (i32, i32),
    end: (i32, i32),
}
impl ArcTrans {
    fn new(oct: u8, c: (u32, u32), end: (i32, i32)) -> Self {
        Self {
            oct,
            c: (c.0 as i32, c.1 as i32),
            end,
        }
    }
}
#[derive(Clone, Debug)]
pub(crate) struct Arc {
    x: i32,
    y: i32,
    d: i32,
    t: ArcTrans,
}
impl Arc {
    fn new(mut ccw: f64, mut cw: f64, radius: i32, center: (i32, i32)) -> Self {
        use crate::drawing::quad;
        if ccw > cw {
            std::mem::swap(&mut ccw, &mut cw);
        }
        // let start_oct = rad_to_oct(ccw);
        // let real_start = quad::rad_to_xy(ccw, radius, center);
        // let (x, y) = quad::quad_to_iter(start_oct, real_start.0, real_start.1, center); // local start
        // let end_oct = rad_to_oct(cw);
        // let real_end = quad::rad_to_xy(cw, radius, center);
        // let end = quad::quad_to_iter(
        //     end_oct,
        //     real_end.0.round() as i32,
        //     real_end.1.round() as i32,
        //     center,
        // ); // local end
        // let d = (x + 1).pow(2) + (y);
        todo!()
    }
}

#[derive(Clone, Debug)]
// Doesn't process arc directions properly - could be improved.  Replaced by Arc
pub(crate) struct ArcIter {
    oct: u8,       // current octant
    r: i32,        // radius
    x: i32,        // x
    y: i32,        // y
    d: i32,        // decision parameter
    c: (i32, i32), // circle center
    end_oct: u8,   // end point
    ex: i32,       // end x-coordinate
    ey: i32,       // end y-coordinate
    inc: bool,
}
impl ArcIter {
    pub(crate) fn new(mut ccw: f64, mut cw: f64, radius: i32, center: (u32, u32)) -> Self {
        use super::octs::rev_trans_oct;
        if ccw > cw {
            std::mem::swap(&mut ccw, &mut cw);
        }

        let oct = rad_to_oct(ccw);
        let end_oct = rad_to_oct(cw);

        let sxf: f64 = center.0 as f64 + (radius as f64) * ccw.cos(); // real starting x coordinate as a float
        let syf: f64 = center.1 as f64 - (radius as f64) * ccw.sin(); // real starting y coordinate as a float
        let exf: f64 = center.0 as f64 + (radius as f64) * cw.cos(); // real starting x coordinate as a float
        let eyf: f64 = center.1 as f64 - (radius as f64) * cw.sin(); // real starting y coordinate as a float

        // translate starting coordinates into coordinates used by the iterator
        let (x, y) = rev_trans_oct(oct, (sxf, syf), (center.0 as f64, center.1 as f64));
        let (ex, ey) = rev_trans_oct(end_oct, (exf, eyf), (center.0 as f64, center.1 as f64));

        let d: i32 = ((x.round() + 1.0).powi(2) + (y.round() - 0.5).powi(2) - radius.pow(2) as f64)
            .round() as i32;

        Self {
            oct,
            r: radius,
            x: x.round() as i32,
            y: y.round() as i32,
            d,
            c: (center.0 as i32, center.1 as i32),
            end_oct,
            ex: ex.round() as i32,
            ey: ey.round() as i32,
            inc: is_inc(oct),
        }
    }
    fn next_oct(&mut self) {
        self.oct += 1;
        self.inc = is_inc(self.oct);
        if self.inc {
            self.x = 0;
            self.y = self.r;
            self.d = 1 - self.r;
        } else {
            self.x = self.r;
            self.y = 0;
            self.d = 1 - self.r;
        }
    }
    fn inc(&mut self) {
        self.x += 1;
        if self.d < 0 {
            self.d += 2 * self.x + 1;
        } else {
            self.y -= 1;
            self.d += 2 * (self.x - self.y) + 1;
        }
        println!("inc: o={} x={} y={}", self.oct, self.x, self.y);
    }
    fn dec(&mut self) {
        self.y += 1;
        if self.d < 0 {
            self.d += 2 * self.y + 1;
        } else {
            self.x -= 1;
            self.d += 2 * (self.y - self.x) + 1;
        }
        println!("dec: o={} x={} y={}", self.oct, self.x, self.y);
    }
}
impl Iterator for ArcIter {
    type Item = (u32, u32);
    fn next(&mut self) -> Option<Self::Item> {
        use super::octs;
        // if self.x > self.y {
        if (self.inc && self.x > self.y) || (!self.inc && self.y > self.x) {
            if self.oct == self.end_oct || self.oct == 7 {
                return None;
            } else {
                self.next_oct();
            }
        }

        if self.oct == self.end_oct && self.x == self.ex && self.y == self.ey {
            return None;
        }
        let (x, y) = (self.x, self.y);
        if self.inc {
            self.inc();
        } else {
            self.dec();
        }

        let (x, y) = octs::trans_to_iter(self.oct, (x, y), self.c);
        Some((x as u32, y as u32))
    }
}

#[derive(Clone, Debug)]
pub struct MPCircle {
    x: i32,
    y: i32,
    d: i32,
}
impl MPCircle {
    pub(crate) fn new(radius: i32) -> Self {
        Self {
            x: 0,
            y: radius,
            d: 1 - radius,
        }
    }
    /// Takes coordinates used in the octant iterators
    pub(crate) fn at(x: f64, y: f64, radius: i32) -> Self {
        let r = radius as f64;
        // d = (x+1)^2 + (y-0.5)^2 - r^2
        let d = (x.floor() + 1.0).powi(2) + (y.floor() - 0.5).powi(2) - radius.pow(2) as f64;

        Self {
            x: x.floor() as i32,
            y: y.floor() as i32,
            d: d.floor() as i32,
        }
    }
}
impl Iterator for MPCircle {
    type Item = (i32, i32);
    fn next(&mut self) -> Option<Self::Item> {
        if self.x > self.y {
            return None;
        }
        let x = self.x;
        let y = self.y;
        self.x += 1;
        if self.d < 0 {
            self.d += 2 * self.x + 1;
        } else {
            self.y -= 1;
            self.d += 2 * (self.x - self.y) + 1;
        }
        Some((x, y))
    }
}

pub struct MPCirleUntil {
    x: i32,
    y: i32,
    d: i32,
    ex: i32,
    ey: i32,
}
impl MPCirleUntil {
    /// Takes coordinates used in the octant iterators
    pub(crate) fn new(radius: i32, end: (i32, i32)) -> Self {
        Self {
            x: 0,
            y: radius,
            d: 1 - radius,
            ex: end.0,
            ey: end.1,
        }
    }
    /// Takes coordinates used in the octant iterators
    pub(crate) fn at(x: f64, y: f64, end: (f64, f64), radius: i32) -> Self {
        let r = radius as f64;
        // d = (x+1)^2 + (y-0.5)^2 - r^2
        let d = (x.floor() + 1.0).powi(2) + (y.floor() - 0.5).powi(2) - radius.pow(2) as f64;
        let end_d =
            (end.0.floor() + 1.0).powi(2) + (end.1.floor() - 0.5).powi(2) - radius.pow(2) as f64;

        println!(
            "r={} x={:.5} y={:.5} ex={:.5} ey={:.5} rounded_x={} rounded_y={} d={:.5} end_d={:.5}",
            radius,
            x,
            y,
            end.0,
            end.1,
            end.0.floor() as i32,
            end.1.floor() as i32,
            d,
            end_d,
        );

        Self {
            x: x.floor() as i32,
            y: y.floor() as i32,
            d: d.floor() as i32,
            ex: end.0.floor() as i32,
            ey: end.1.floor() as i32,
        }
    }
}
impl Iterator for MPCirleUntil {
    type Item = (i32, i32);
    fn next(&mut self) -> Option<Self::Item> {
        // if self.x > self.y || (self.ex == self.x && self.ey == self.y) {
        if self.x > self.y || self.y < 0 {
            return None;
        }
        // if self.d <= 0 && self.x == self.ex && self.y == self.ey {
        //     return None;
        // }
        // if self.d > 0 && self.x == self.ex && self.y == self.ey {
        //     return None;
        // }

        let x = self.x;
        let y = self.y;
        self.x += 1;

        if self.d < 0 {
            if self.x == self.ex && self.y == self.ey {
                println!("Terminating ");
                return None;
            }
            self.d += 2 * self.x + 1;
        } else {
            if self.x == self.ex && self.y == self.ey {
                return None;
            }
            self.y -= 1;
            self.d += 2 * (self.x - self.y) + 1;
        }
        Some((x, y))
    }
}

// WARNING: needs refinement - produces fairly good but not perfect circles
pub struct Bres {
    x: i32,
    y: i32,
    d: i32,
}
impl Bres {
    pub(crate) fn new(radius: i32) -> Self {
        let d = 3 - 2 * radius;
        Self { x: 0, y: radius, d }
    }
}
impl Iterator for Bres {
    type Item = (i32, i32);
    fn next(&mut self) -> Option<Self::Item> {
        if self.x > self.y {
            return None;
        }
        self.x += 1;
        if self.d > 0 {
            self.y -= 1;
            self.d += 4 * (self.x - self.y) + 10;
        } else {
            self.d += 4 * self.x + 6;
        }
        Some((self.x, self.y))
    }
}

// WARNING: needs refinement - produces fairly good but not perfect circles
pub struct BresTo {
    x: i32,
    y: i32,
    d: i32,
    ex: i32,
    ey: i32,
}
impl BresTo {
    pub(crate) fn new(radius: i32, end: (i32, i32)) -> Self {
        let (ex, ey) = end;
        let d = 3 - 2 * radius;
        Self {
            x: 0,
            y: radius,
            d,
            ex,
            ey,
        }
    }
}
impl Iterator for BresTo {
    type Item = (i32, i32);
    fn next(&mut self) -> Option<Self::Item> {
        if self.x > self.y || (self.ex == self.x && self.ey == self.y) {
            return None;
        }
        self.x += 1;
        if self.d > 0 {
            self.y -= 1;
            self.d += 4 * (self.x - self.y) + 10;
        } else {
            self.d += 4 * self.x + 6;
        }
        Some((self.x, self.y))
    }
}

// WARNING: needs refinement - produces fairly good but not perfect circles
pub struct BresFrom {
    x: i32,
    y: i32,
    d: i32,
}
impl BresFrom {
    pub(crate) fn new(radius: i32, start: (i32, i32)) -> Self {
        let (sx, sy) = start;
        let d = 2 * (sx + 1).pow(2) + (sy.pow(2)) + (sy - 1).pow(2) - 2 * radius.pow(2);
        println!("d={}", d);
        Self { x: sx, y: sy, d }
    }
}
impl Iterator for BresFrom {
    type Item = (i32, i32);
    fn next(&mut self) -> Option<Self::Item> {
        if self.x > self.y {
            return None;
        }
        self.x += 1;
        if self.d > 0 {
            self.y -= 1;
            self.d += 4 * (self.x - self.y) + 10;
        } else {
            self.d += 4 * self.x + 6;
        }
        Some((self.x, self.y))
    }
}

// pub struct ArcSegment {
//     oct: u8,   // octant
//     tccw: f64, // counter-clockwise theta
//     tcw: f64,  // clockwise theta
//     r: f64,    // radius
//     x: f64,    // x-coordinate
//     y: f64,    // y-coordinate
//     d: f64,    // decision parameter
//     ex: f64,   // ending x-coordinate
//     ey: f64,   // ending y-coordinate
// }
// impl ArcSegment {
//     fn new(oct: u8, tccw: f64, tcw: f64, r: f64, end: (f64, f64)) -> Self {
//         Self {
//             oct,
//             tccw,
//             tcw,
//             r,
//             x:
//             ex: end.0,
//             ey: end.1,
//         };
//         todo!()
//     }
//     /// Returns wether the current x and y are close to being a match for given coordinates within a given tolerance `n`
//     fn near(&self, x: f64, y: f64, n: f64) -> bool {
//         (self.x - x).abs() < n
//     }
// }
// impl Iterator for ArcSegment {
//     type Item = (i32, i32);
//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }
