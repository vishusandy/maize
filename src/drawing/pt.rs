pub(crate) struct Pt<T> {
    x: T,
    y: T,
}
impl<T> Pt<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl Pt<f64> {
    fn from_radian<T>(angle: f64, center: (T, T), radius: T) -> Self
    where
        T: Into<f64> + Copy,
    {
        let x = center.0.into() + radius.into() * angle.cos();
        let y = center.1.into() + radius.into() * angle.sin();

        Self {
            x: 0.0f64,
            y: 0.0f64,
        }
    }
}
