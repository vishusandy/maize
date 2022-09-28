pub(crate) mod torus;

#[derive(Clone, Debug)]
pub struct Pt<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

impl<T> Copy for Pt<T> where T: Copy {}

impl<T> Pt<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn x(&self) -> T
    where
        T: Copy,
    {
        self.x
    }

    #[inline]
    pub const fn y(&self) -> T
    where
        T: Copy,
    {
        self.y
    }
}

impl Pt<f64> {
    pub(crate) fn from_radian<T>(angle: f64, radius: T, center: (T, T)) -> Self
    where
        T: Into<f64> + Copy,
    {
        let x = center.0.into() + radius.into() * angle.cos();
        let y = center.1.into() - radius.into() * angle.sin();

        Self { x, y }
    }

    #[inline]
    pub(crate) fn i32(&self) -> Pt<i32> {
        Pt {
            x: self.x.round() as i32,
            y: self.y.round() as i32,
        }
    }
    #[allow(dead_code)]
    pub(crate) fn u32(&self) -> Pt<u32> {
        Pt {
            x: self.x.abs().round() as u32,
            y: self.y.abs().round() as u32,
        }
    }
}

impl Pt<i32> {
    #[inline]
    pub(crate) fn u32(&self) -> Pt<u32> {
        Pt {
            x: self.x as u32,
            y: self.y as u32,
        }
    }
}

impl<T> From<(T, T)> for Pt<T> {
    fn from(tuple: (T, T)) -> Self {
        Self::new(tuple.0, tuple.1)
    }
}

impl<T> From<Pt<T>> for (T, T) {
    fn from(pt: Pt<T>) -> Self {
        (pt.x, pt.y)
    }
}

impl From<Pt<i32>> for Pt<f64> {
    fn from(pt: Pt<i32>) -> Self {
        Self {
            x: pt.x.into(),
            y: pt.y.into(),
        }
    }
}

impl From<Pt<f64>> for Pt<i32> {
    fn from(pt: Pt<f64>) -> Self {
        Self {
            x: pt.x.round() as i32,
            y: pt.y.round() as i32,
        }
    }
}

impl<T> std::ops::Add for Pt<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> std::ops::Sub for Pt<T>
where
    T: std::ops::Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
