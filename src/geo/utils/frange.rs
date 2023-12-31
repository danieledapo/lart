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
pub fn frange(start: f64, end: f64, step: f64) -> F64Range {
    debug_assert!(step != 0.0);
    debug_assert!(
        (end - start).signum() == step.signum(),
        "start={start} end={end} step={step}"
    );

    F64Range { start, end, step }
}

pub struct F64Range {
    start: f64,
    end: f64,
    step: f64,
}

impl Iterator for F64Range {
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
