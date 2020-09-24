use std::ops;

const FLOAT_THRESHOLD: f64 = 0.0001;

/// Evaluate the equality of two floats to within a threshold value of FLOAT_THRESHOLD
pub fn f_eq(lhs: f64, rhs: f64) -> bool {
    (lhs - rhs).abs() <= FLOAT_THRESHOLD
}

/// Linearly interpolate between a starting and ending value with a factor of `t`
pub fn lerp<T>(start: T, end: T, t: f64) -> T
where
    T: ops::Mul<f64, Output=T>,
    T: ops::Add<T, Output=T>,
{
    start * (1.0 - t) + end * t
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