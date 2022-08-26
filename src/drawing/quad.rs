use std::ops::{Add, Neg};

pub(crate) fn rad_to_xy<T>(angle: f64, r: T, c: (T, T)) -> (f64, f64)
where
    T: Into<f64> + Copy,
{
    let oct = crate::drawing::cir::rad_to_oct(angle);
    let x = c.0.into() + (r.into() * angle.cos());
    let y = c.1.into() + (r.into() * angle.sin());
    (x, y)
}

pub(crate) fn iter_to_quad<T: Neg<Output = T> + Add<Output = T>>(
    quad: u8,
    x: T,
    y: T,
    c: (T, T),
) -> (T, T) {
    match quad {
        0 => add(c, q3_to_q0(x, y)),
        1 => add(c, q3_to_q1(x, y)),
        2 => add(c, q3_to_q2(x, y)),
        3 => add(c, q3_to_q3(x, y)),
        _ => iter_to_quad(quad % 4, x, y, c),
    }
}
pub(crate) fn quad_to_iter<T: Neg<Output = T> + Add<Output = T>>(
    quad: u8,
    x: T,
    y: T,
    c: (T, T),
) -> (T, T) {
    match quad {
        0 => add(c, q0_to_q3(x, y)),
        1 => add(c, q1_to_q3(x, y)),
        2 => add(c, q2_to_q3(x, y)),
        3 => add(c, q3_to_q3(x, y)),
        _ => quad_to_iter(quad % 4, x, y, c),
    }
}
fn q3_to_q0<T: Neg<Output = T>>(x: T, y: T) -> (T, T) {
    (x, -y)
}
fn q3_to_q1<T: Neg<Output = T>>(x: T, y: T) -> (T, T) {
    (-x, -y)
}
fn q3_to_q2<T: Neg<Output = T>>(x: T, y: T) -> (T, T) {
    (-x, y)
}
fn q3_to_q3<T: Neg<Output = T>>(x: T, y: T) -> (T, T) {
    (x, y)
}
fn q0_to_q3<T: Neg<Output = T>>(x: T, y: T) -> (T, T) {
    (x, -y)
}
fn q1_to_q3<T: Neg<Output = T>>(x: T, y: T) -> (T, T) {
    (-x, -y)
}
fn q2_to_q3<T: Neg<Output = T>>(x: T, y: T) -> (T, T) {
    (-x, y)
}

pub(crate) fn add<T: Add<Output = T>>(a: (T, T), b: (T, T)) -> (T, T) {
    (a.0 + b.0, a.1 + b.1)
}
