use crate::{F64Key, Polygon, V};

/// An implementation of the [Gift Wrapping Algorithm][0] to return the Convex
/// Hull of a set of points.
///
/// [0]: https://en.wikipedia.org/wiki/Gift_wrapping_algorithm
pub fn convex_hull(points: &[V]) -> Polygon {
    if points.len() <= 3 {
        return Polygon::from(points.iter().cloned());
    }

    let mut hull = vec![];

    let mut on_hull = *points
        .iter()
        .min_by_key(|p| (F64Key(p.x), F64Key(p.y)))
        .unwrap();

    loop {
        hull.push(on_hull);
        let mut endpoint = points[0];

        for p in points {
            if endpoint == on_hull || p.orient(on_hull, endpoint) > 0.0 {
                endpoint = *p;
            }
        }

        if endpoint == hull[0] {
            break;
        }

        on_hull = endpoint;
    }

    Polygon::from(hull)
}
