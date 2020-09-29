use rand::{
    distributions::{ Distribution, Uniform },
    rngs::ThreadRng
};
use std::ops;

const FLOAT_THRESHOLD: f64 = 0.0001;

pub struct Rand {
    pub dist: Uniform<f64>,
    pub rng: ThreadRng
}

/// Evaluate the equality of two floats to within a threshold value of FLOAT_THRESHOLD
pub fn f_eq(lhs: f64, rhs: f64) -> bool {
    (lhs - rhs).abs() <= FLOAT_THRESHOLD
}

pub fn f_leq(lhs: f64, rhs: f64) -> bool {
    lhs < rhs || f_eq(lhs, rhs)
}

pub fn f_geq(lhs: f64, rhs: f64) -> bool {
    lhs > rhs || f_eq(lhs, rhs)
}

/// Linearly interpolate between a starting and ending value with a factor of `t`
pub fn lerp<T>(start: T, end: T, t: f64) -> T
where
    T: ops::Mul<f64, Output=T>,
    T: ops::Add<T, Output=T>,
{
    start * (1.0 - t) + end * t
}

/// TODO: Find the median of a list in O(n) time
// pub fn median<T>(vec: Vec<T>) -> T {
// }

/// Clamp a value between a minimum and maximum
pub fn f_clamp(val: f64, min: f64, max: f64) -> f64 {
    f64::max(min, f64::min(max, val))
}

/// Generate a random float given a distribution
pub fn rand_f64(rand: &mut Rand) -> f64 {
    rand.dist.sample(&mut rand.rng)
}

/// Calculate a Schlick approximation for the specular reflection coefficient
pub fn schlick(cos_theta_i: f64, index_i: f64, index_r: f64) -> f64 {
    let r_0 = ((index_i - index_r) / (index_i + index_r)).powi(2);
    r_0 + (1.0 - r_0) * (1.0 - cos_theta_i).powi(5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f_equality() {
        let f1 = 3.0;
        let f2 = 3.0000004;
        assert!(f_eq(f1, f2), "{} =/= {}", f1, f2);
    }

    #[test]
    fn lerp_floats() {
        let start = 1.0;
        let end = 4.0;
        let result = lerp(start, end, 0.7);
        assert!(f_eq(result, 3.1), "Got {}, not {}", result, 3.1);
    }
}
