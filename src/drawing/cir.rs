#[derive(Clone, Debug)]
pub(crate) struct AnnulusSlice {
    center: (f64, f64),
    /// Theta for the counter-clockwise point (angle in radians)
    ccw: f64,
    /// Theta for the clockwise point (angle in radians)
    cw: f64,
    /// Innter radius
    inner: f64,
    /// Outer radius
    outer: f64,
}
impl AnnulusSlice {
    pub(crate) fn new(center: (f64, f64), ccw: f64, cw: f64, inner: f64, outer: f64) -> Self {
        Self {
            center,
            ccw,
            cw,
            inner,
            outer,
        }
    }
    /// The x coordinate that the torus is centered around
    fn cx(&self) -> f64 {
        self.center.0
    }
    /// The y coordinate that the torus is centered around
    fn cy(&self) -> f64 {
        self.center.1
    }

    pub(crate) fn ccw(&self) -> f64 {
        self.ccw
    }
    pub(crate) fn cw(&self) -> f64 {
        self.cw
    }

    /// Counter-clockwise inner point
    pub(crate) fn a(&self) -> (f64, f64) {
        (
            self.cx() + self.inner * self.ccw.cos(),
            self.cy() - self.inner * self.ccw.sin(),
        )
    }
    /// Counter-clockwise outer point
    pub(crate) fn b(&self) -> (f64, f64) {
        (
            self.cx() + self.outer * self.ccw.cos(),
            self.cy() - self.outer * self.ccw.sin(),
        )
    }
    /// Clockwise inner point
    pub(crate) fn c(&self) -> (f64, f64) {
        (
            self.cx() + self.inner * self.cw.cos(),
            self.cy() - self.inner * self.cw.sin(),
        )
    }
    /// Clockwise outer point
    pub(crate) fn d(&self) -> (f64, f64) {
        (
            self.cx() + self.outer * self.cw.cos(),
            self.cy() - self.outer * self.cw.sin(),
        )
    }
    /// Center of the torus slice
    pub(crate) fn center(&self) -> (f64, f64) {
        let (ax, ay) = self.a();
        let (bx, by) = self.b();
        let (cx, cy) = self.c();
        let (dx, dy) = self.d();
        let x = (ax + bx + cx + dx) / 4.0;
        let y = (ay + by + cy + dy) / 4.0;
        (x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;
    #[test]
    fn torus() -> Result<(), image::ImageError> {
        let mut image =
            image::RgbaImage::from_vec(400, 400, Vec::from([255; 400 * 400 * 4])).unwrap();
        let torus = AnnulusSlice::new((200.0, 200.0), PI * 0.0, PI * 0.25, 50.0, 80.0);
        let (x, y) = torus.a();
        image.put_pixel(
            x.round() as u32,
            y.round() as u32,
            image::Rgba([255, 0, 0, 255]),
        );
        let (x, y) = torus.b();
        image.put_pixel(
            x.round() as u32,
            y.round() as u32,
            image::Rgba([0, 255, 0, 255]),
        );
        let (x, y) = torus.c();
        image.put_pixel(
            x.round() as u32,
            y.round() as u32,
            image::Rgba([0, 0, 255, 255]),
        );
        let (x, y) = torus.d();
        image.put_pixel(
            x.round() as u32,
            y.round() as u32,
            image::Rgba([0, 0, 0, 255]),
        );
        image.save("images/tests/torus.png")
    }
}
