use std::f64::consts::PI;

use crate::{frange, path, Bbox, Geometry, Polygon, V};

pub fn parallel_hatch(g: &Polygon, a: f64, step: f64) -> Geometry {
    let mut tex = Geometry::new();
    let Some(bbox) = g.bbox() else { return tex };

    let d = V::polar(a, 1.0);
    let pd = V::polar(a + PI / 2.0, 1.0);
    let r = 0.5 * f64::hypot(bbox.width(), bbox.height());

    let p0 = bbox.center() + d * r;
    let p1 = bbox.center() - d * r;
    for dd in frange(-r, r + step, step) {
        let o = pd * dd;
        tex.push_path(path!(p0 + o, p1 + o));
    }

    tex & g
}
