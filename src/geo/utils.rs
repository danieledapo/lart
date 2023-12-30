use std::ops::{Add, Mul, Sub};

/// Create an iterator over a range of floats stepping by the given amount.
///
/// The range is exclusive with regard to the end.
///
/// ```rust
/// # use lart::*;
/// assert_eq!(frange(-1.5, 1.0, 0.5).collect::<Vec<_>>(), vec![-1.5, -1.0, -0.5, 0.0, 0.5]);
/// assert_eq!(frange(1.5, -1.0, -0.5).collect::<Vec<_>>(), vec![1.5, 1.0, 0.5, 0.0, -0.5]);
/// assert_eq!(frange(0.0, 0.1, 0.5).collect::<Vec<_>>(), vec![0.0]);
/// assert_eq!(frange(0.0, -0.1, -0.5).collect::<Vec<_>>(), vec![0.0]);
/// ```
pub fn frange(start: f64, end: f64, step: f64) -> FRange {
    debug_assert!(step != 0.0);
    debug_assert!((end - start).signum() == step.signum());

    FRange { start, end, step }
}

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

/// Dead simple wrapper over a f64 that can be used as the key to the various
/// sort_by_key functions.
///
/// Note that it behaves slightly differently than f64 with regard to equality
/// checking namely that `-0.0_f64 == 0.0_f64` but `F64Key(-0.0) != F64Key(0.0)`.
#[derive(Debug, Clone, Copy)]
pub struct F64Key(pub f64);

impl Eq for F64Key {}
impl PartialEq for F64Key {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Ord for F64Key {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}
impl PartialOrd for F64Key {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct FRange {
    start: f64,
    end: f64,
    step: f64,
}

impl Iterator for FRange {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.step > 0.0 && self.start >= self.end)
            || (self.step < 0.0 && self.start <= self.end)
        {
            return None;
        }

        let v = self.start;
        self.start += self.step;
        Some(v)
    }

    // The following functions rely on the fact that `sizeof::<usize>() >= sizeof::<f64>()`
    // which is fine, I don't plan to support 32bit platforms nor 16bit ones.
    #[cfg(target_pointer_width = "64")]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = f64::abs((self.end - self.start) / self.step).ceil();
        (n as usize, Some(n as usize))
    }

    #[cfg(target_pointer_width = "64")]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.size_hint().0
    }

    #[cfg(target_pointer_width = "64")]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n < f64::MAX as usize {
            self.start += self.step * n as f64;
        } else {
            for _ in 0..n {
                self.next()?;
            }
        }

        self.next()
    }
}
