use std::ops::{Add, Mul};

pub fn lerp<T>(t: f32, x0: T, x1: T) -> T
where T: Add<T, Output = T> + Mul<f32, Output = T>{
    x0*(1.0-t) + x1*t
}
