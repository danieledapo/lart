use std::ops::{Add, Mul, Sub};

mod fkey;
mod frange;

pub use fkey::*;
pub use frange::*;

/// Linearly interpolate between the two given numbers or points with the given
/// t parameter.
///
/// ```rust
/// # use lart::*;
/// assert_eq!(linterp(0.0, 10.0, 0.5), 5.0);
/// assert_eq!(linterp(v(0,5), v(-10,10), 0.0), v(0,5));
/// assert_eq!(linterp(v(0,5), v(0,10), 2.0), v(0,15));
/// assert_eq!(linterp(v(0,5), v(0,10), -2.0), v(0,-5));
/// ```
pub fn linterp<N>(a: N, b: N, t: f64) -> N
where
    N: Add<Output = N> + Sub<Output = N> + Mul<f64, Output = N> + Copy,
{
    a + (b - a) * t
}

/// Map the input value from the given range to the unit range [0..1].
///
/// ```rust
/// # use lart::*;
/// assert_eq!(mapu(5, 5, 100), 0.0);
/// assert_eq!(mapu(50, 0, 100), 0.5);
/// assert_eq!(mapu(25, 100, 0), 0.75);
/// ````
pub fn mapu(v: impl Into<f64>, start: impl Into<f64>, end: impl Into<f64>) -> f64 {
    let start = start.into();
    (v.into() - start) / (end.into() - start)
}

/// Map the input value from the given input range to the given output range.
///
/// ```rust
/// # use lart::*;
/// assert_eq!(map(25, 0, 100, 0, 4), 1.0);
/// assert_eq!(map(50, -100, 100, 0, 4), 3.0);
/// ````
pub fn map(
    v: impl Into<f64>,
    start: impl Into<f64>,
    end: impl Into<f64>,
    ostart: impl Into<f64>,
    oend: impl Into<f64>,
) -> f64 {
    linterp(ostart.into(), oend.into(), mapu(v, start, end))
}
