pub(crate) mod cir;
pub(crate) mod octs;
pub(crate) mod pt;
pub(crate) mod quad;
pub(crate) mod torus;
pub(crate) mod wu;

// Bresenham's circle algorithm
// https://arcade.makecode.com/graphics-math/bresenham-circle
// http://members.chello.at/~easyfilter/bresenham.html
//      d = 2 * (x + 1)^2 + Yi^2 + (y - 1)^2 - 2 * radius^2

// Midpoint Circle Algorithm
// https://lectureloops.com/mid-point-circle-drawing-algorithm/
//      d = (x+1)^2 + (y-0.5)^2 - r^2

// Wu Antialiasing
// http://www.landkey.net/d/antialiased/wu4_RF/
// https://yellowsplash.wordpress.com/2009/10/23/fast-antialiased-circles-and-ellipses-from-xiaolin-wus-concepts/
// https://create.stephan-brumme.com/antialiased-circle/

pub(crate) struct Pt<T> {
    x: T,
    y: T,
}
impl<T> Pt<T>
where
    T: Copy,
{
    pub(crate) fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    pub(crate) fn x(&self) -> T {
        self.x
    }
    pub(crate) fn y(&self) -> T {
        self.y
    }
    pub(crate) fn set_x(&self, x: T) -> Self
    where
        T: Copy,
    {
        Self { x, y: self.y }
    }
    pub(crate) fn set_y(&self, y: T) -> Self
    where
        T: Copy,
    {
        Self { x: self.x, y }
    }
    pub(crate) fn add(&self, other: Self) -> Self
    where
        T: std::ops::Add<Output = T>,
    {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    pub(crate) fn sub(&self, other: Self) -> Self
    where
        T: std::ops::Sub<Output = T>,
    {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Pt<f64> {
    pub(crate) fn from_radian<T: Into<f64>>(angle: f64, radius: T) -> Self {
        let r: f64 = radius.into();
        Self {
            x: r * angle.cos(),
            y: r * angle.sin(),
        }
    }
}

impl From<Pt<f64>> for Pt<u32> {
    fn from(p: Pt<f64>) -> Self {
        Pt::new(p.x.round() as u32, p.y.round() as u32)
    }
}
impl From<Pt<f32>> for Pt<u32> {
    fn from(p: Pt<f32>) -> Self {
        Pt::new(p.x.round() as u32, p.y.round() as u32)
    }
}
impl From<Pt<i32>> for Pt<u32> {
    fn from(p: Pt<i32>) -> Self {
        Pt::new(p.x as u32, p.y as u32)
    }
}

fn oct_coords<T>(oct: u8, x: T, y: T) -> (T, T)
where
    T: std::ops::Neg<Output = T>,
{
    match oct {
        0 => (x, y),
        1 => (y, x),
        2 => (-y, x),
        3 => (-x, y),
        4 => (-x, -y),
        5 => (-y, -x),
        6 => (y, -x),
        7 => (x, -y),
        _ => (x, y),
    }
}
fn oct_center<T>(oct: u8, x: T, y: T, center: (T, T)) -> (T, T)
where
    T: std::ops::Neg<Output = T> + std::ops::Add<Output = T>,
{
    match oct {
        0 => (x + center.0, y + center.1),
        1 => (y + center.0, x + center.1),
        2 => (-y + center.0, x + center.1),
        3 => (-x + center.0, y + center.1),
        4 => (-x + center.0, -y + center.1),
        5 => (-y + center.0, -x + center.1),
        6 => (y + center.0, -x + center.1),
        7 => (x + center.0, -y + center.1),
        _ => (x + center.0, y + center.1),
    }
}
