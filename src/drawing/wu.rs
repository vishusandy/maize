use super::{oct_center, oct_coords};

struct Wu {}

#[derive(Clone, Debug)]
pub struct WuIter {
    /// cur x coordinate
    x: i32,
    /// cur y coordinate
    y: i32,
    /// y coordinate squared
    y2: i32,
    /// radius squared
    r2: i32,
    /// last intensity
    d: u32,
    /// intensity range - determines accuracy
    i: f64,
}
impl WuIter {
    #[allow(dead_code)]
    fn new(radius: i32, i: i32) -> Self {
        Self {
            x: radius,
            y: 0,
            y2: 0,
            r2: radius * radius,
            d: 0,
            i: i.into(),
        }
    }
    fn wu_alpha(&self) -> u32 {
        let ry = (self.r2 - self.y2) as f64;
        let alpha = self.i * ry.sqrt() % self.i;
        match alpha {
            x if x <= std::f64::EPSILON => 0,
            _ => (self.i - alpha) as u32,
        }
    }
}

impl Iterator for WuIter {
    // Returns x, y, and intensity
    type Item = WuPair;
    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.x {
            return None;
        }
        // let int = self.a.find_intensity(self.r2 - self.y2);
        let int = self.wu_alpha();
        if int < self.d {
            self.x -= 1;
        }
        let a = (self.x, self.y);
        let b = (self.x - 1, self.y);
        let pair = WuPair { a, b, int };
        self.y += 1;
        self.y2 = self.y * self.y;
        self.d = int;
        Some(pair)
    }
}

pub struct WuPair {
    pub a: (i32, i32),
    pub b: (i32, i32),
    pub int: u32,
}
impl WuPair {
    pub fn transform(&self, oct: u8) -> Self {
        Self {
            a: oct_coords(oct, self.a.0, self.a.1),
            b: oct_coords(oct, self.b.0, self.b.1),
            int: self.int,
        }
    }
    pub fn transform_with_center(&self, oct: u8, center: (i32, i32)) -> Self {
        Self {
            a: oct_center(oct, self.a.0, self.a.1, center),
            b: oct_center(oct, self.b.0, self.b.1, center),
            int: self.int,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn arc() {}
}
