pub mod canvas;
pub mod color;
pub mod space;

#[cfg(test)]
mod testlib;

pub const EPSILON: f64 = 0.00001;

fn approx_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}
