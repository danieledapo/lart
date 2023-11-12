// TODO: this should be implemented for Geometry too, but it's not trivial
// because of holes and of concave shapes

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

    let mut last_endpoint_match = false;
    for i in 0..p.len() {
        let s = (p[i], p[(i + 1) % p.len()]);

        if s.0 == s.1 {
            continue;
        }

        a.push(s.0);

        let x;
        if last_endpoint_match {
            x = s.0;
        } else if let Some(intr) = seg_x_line(s, l) {
            x = intr;
            // if the intersection is almost equal to the final endpoint skip
            // this segment, the intersection will be added only once at the
            // next segment.
            if x.almost_equal(s.1) {
                last_endpoint_match = true;
                continue;
            }
        } else {
            continue;
        }

        a.push(x);

        std::mem::swap(&mut a, &mut b);
        a.push(x);

        last_endpoint_match = false;
    }

    if last_endpoint_match {
        a.push(p[0]);
    }

    a.dedup();
    b.dedup();

    // the above loop can generate lines if there are coincident points, keep
    // only polygons
    [a, b]
        .into_iter()
        .filter(|pp| pp.len() > 2)
        .map(|mut p| {
            // close path so that it's always considered a closed polygon
            p.push(p[0]);
            p
        })
        .collect()
}
