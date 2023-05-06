// TODO: this should be implemented for Polygon and Geometry too, but it's not
// trivial because of holes and of concave shapes

use crate::{seg_x_line, Path, V};

/// Split the given Path by the infinite line passing through two points.
pub fn split_path(p: &Path, l: (V, V)) -> Vec<Path> {
    let mut res = vec![];

    let mut cur = Path::new();
    for s in p.segments() {
        cur.push(s.0);

        if let Some(x) = seg_x_line(s, l) {
            cur.push(x);

            res.push(cur);
            cur = Path::new();
        }
    }

    if !cur.is_empty() {
        cur.push(p.last().unwrap());
        res.push(cur);
    }

    res
}

/// Split the given convex polygon into two other convex polygons at most by the
/// infinite line passing through two points
///
/// No checks are made to ensure that the Path is actually convex.
pub fn split_convex_polygon(p: &Path, l: (V, V)) -> Vec<Path> {
    let mut a = Path::new();
    let mut b = Path::new();

    for i in 0..p.len() {
        let s = (p[i], p[(i + 1) % p.len()]);
        a.push(s.0);
        if let Some(x) = seg_x_line(s, l) {
            a.push(x);

            std::mem::swap(&mut a, &mut b);
            a.push(x);
        }
    }

    [a, b].into_iter().filter(|pp| !pp.is_empty()).collect()
}
