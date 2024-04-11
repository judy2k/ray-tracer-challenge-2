pub mod canvas;
pub mod color;
pub mod lighting;
pub mod materials;
pub mod matrix;
pub mod ppm;
pub mod ray;
pub mod shape;
pub mod space;
pub mod world;

#[cfg(test)]
mod testlib;

const EPSILON: f64 = 0.00001;

fn approx_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}
