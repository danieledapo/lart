use crate::{frange, V};

#[derive(Debug, Clone, PartialEq)]
pub struct PathSample {
    pub point: V,
    pub t: f64,
    pub segment_len: f64,
    pub segment_dir: V,
}

/// Sample the segment going from a to b every step.
///
/// Whether b is included or excluded is driven by the include_b parameter.
///
/// ```rust
/// # use lart::*;
/// let samples: Vec<_> = sample_seg(v(1,1), v(1,1), 1.0, true).collect();
/// assert_eq!(samples.len(), 1);
/// assert_eq!((samples[0].point, samples[0].t, samples[0].segment_len), (v(1,1), 0.0, 0.0));
/// assert!(samples[0].segment_dir.x.is_nan() && samples[0].segment_dir.y.is_nan());
///
/// let samples: Vec<_> = sample_seg(v(-1, 2), v(0, 2), 0.5, false).collect();
/// assert_eq!(samples.len(), 2);
/// assert_eq!(samples[0], PathSample {point: v(-1, 2), t: 0.0, segment_len: 1.0, segment_dir: v(1,0)});
/// assert_eq!(samples[1], PathSample {point: v(-0.5, 2), t: 0.5, segment_len: 1.0, segment_dir: v(1,0)});
///
/// let samples: Vec<_> = sample_seg(v(2, -1), v(2, 0), 0.5, true).collect();
/// assert_eq!(samples.len(), 3);
/// assert_eq!(samples[0], PathSample {point: v(2, -1), t: 0.0, segment_len: 1.0, segment_dir: v(0,1)});
/// assert_eq!(samples[1], PathSample {point: v(2, -0.5), t: 0.5, segment_len: 1.0, segment_dir: v(0,1)});
/// assert_eq!(samples[2], PathSample {point: v(2, 0), t: 1.0, segment_len: 1.0, segment_dir: v(0,1)});
/// ```
pub fn sample_seg(a: V, b: V, step: f64, include_b: bool) -> impl Iterator<Item = PathSample> {
    let mut d = b - a;
    let l = d.norm();
    d /= l;

    frange(0.0, l, step)
        .map(move |t| PathSample {
            t,
            point: a + d * t,
            segment_dir: d,
            segment_len: l,
        })
        .chain(if include_b || a == b {
            Some(PathSample {
                point: b,
                t: if a == b { 0.0 } else { l },
                segment_len: l,
                segment_dir: d,
            })
        } else {
            None
        })
}
