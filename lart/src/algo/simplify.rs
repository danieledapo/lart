use crate::{Geometry, Path, V};

pub trait Simplify {
    /// Simplify a given geometry with the given eps using the
    /// [Ramer-Douglas-Peucker][0] simplification algorithm.
    ///
    /// The higher the epsilon the more simplified and butchered the final path.
    ///
    /// [0]:
    ///     https://en.wikipedia.org/wiki/Ramer%E2%80%93Douglas%E2%80%93Peucker_algorithm
    fn simplify(&self, eps: f64) -> Self;
}

impl Simplify for Path {
    fn simplify(&self, eps: f64) -> Self {
        let mut out = Path::with_capacity(self.len());
        rdp(&mut out, self.points(), eps);
        if self.is_closed() {
            out.close();
        }
        out
    }
}

impl Simplify for Geometry {
    fn simplify(&self, eps: f64) -> Self {
        let mut g = Geometry::new();

        g.push_paths(self.paths.iter().filter_map(|p| {
            let p = p.simplify(eps);
            (!p.is_empty()).then_some(p)
        }));

        g
    }
}

/// Implementation of the Ramer-Douglas-Peucker simplification algorithm.
#[allow(clippy::needless_range_loop)]
fn rdp(out: &mut Path, path: &[V], eps: f64) {
    if path.len() < 3 {
        out.points.extend(path);
        return;
    }

    let p0 = *path.first().unwrap();
    let p1 = *path.last().unwrap();

    // if p0==p1 we cannot calculate the distance properly, just try the point
    // before the last one
    if p0 == p1 {
        rdp(out, &path[..path.len() - 1], eps);
        return;
    }

    let mut maxd: f64 = -1.0;
    let mut i = path.len();
    for j in 1..path.len() - 1 {
        let d = perpendicular_dist((p0, p1), path[j]);
        if d > maxd {
            maxd = d;
            i = j;
        }
    }

    if maxd <= eps {
        out.push(p0);
        out.push(p1);
        return;
    }

    rdp(out, &path[..=i], eps);
    out.pop();
    rdp(out, &path[i..], eps);
}

fn perpendicular_dist((a, b): (V, V), p: V) -> f64 {
    let n = f64::abs((b.x - a.x) * (a.y - p.y) - (b.x - p.x) * (b.y - a.y));
    n / a.dist(b)
}
