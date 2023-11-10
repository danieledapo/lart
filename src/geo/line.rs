use crate::{linterp, PRECISION, V};

/// Find the intersection between two segments returning the position of the
/// intersection, if any.
pub fn seg_x_seg(seg1: (V, V), seg2: (V, V)) -> Option<V> {
    let t = seg_x_line_t(seg1, seg2)?;
    seg_x_line_t(seg2, seg1)?;
    Some(linterp(seg1.0, seg1.1, t))
}

/// Find the intersection between two segments returning the parameter along the
/// first segment of the intersection, if any.
pub fn seg_x_seg_t(seg1: (V, V), seg2: (V, V)) -> Option<f64> {
    let t = seg_x_line_t(seg1, seg2)?;
    seg_x_line_t(seg2, seg1)?;
    Some(t)
}

/// Find the intersection between a segment and an infinite line passing through
/// two points returning the position of the intersection, if any.
pub fn seg_x_line(seg: (V, V), l: (V, V)) -> Option<V> {
    seg_x_line_t(seg, l).map(|t| linterp(seg.0, seg.1, t))
}

/// Find the intersection between a segment and an infinite line passing through
/// two points returning the parameter along the first segment of the
/// intersection, if any.
pub fn seg_x_line_t(seg: (V, V), l: (V, V)) -> Option<f64> {
    let mut t = line_x_line_t(seg, l)?;
    if (-PRECISION..0.0).contains(&t) {
        t = 0.0;
    } else if (1.0..1.0 + PRECISION).contains(&t) {
        t = 1.0;
    }
    (0.0..=1.0).contains(&t).then_some(t)
}

/// Find the intersection between two infinite lines passing through two points
/// returning the position of the intersection, if any.
pub fn line_x_line(l1: (V, V), l2: (V, V)) -> Option<V> {
    line_x_line_t(l1, l2).map(|t| linterp(l1.0, l1.1, t))
}

/// Find the intersection between two infinite lines passing through two points
/// returning the parameter along the first line, if any.
pub fn line_x_line_t((a, b): (V, V), (c, d): (V, V)) -> Option<f64> {
    let det = (a.x - b.x) * (c.y - d.y) - (a.y - b.y) * (c.x - d.x);
    if det.abs() < PRECISION {
        return None;
    }
    let num = (a.x - c.x) * (c.y - d.y) - (a.y - c.y) * (c.x - d.x);
    Some(num / det)
}
