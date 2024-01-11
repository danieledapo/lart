use std::{f64::consts::PI, ops::BitAnd};

use crate::{frange, path, Bbox, Geometry, V};

pub fn parallel_hatch<'a, T>(g: &'a T, a: f64, step: f64) -> Geometry
where
    T: Bbox,
    Geometry: BitAnd<&'a T, Output = Geometry>,
{
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

pub fn vertical_hatch<'a, T>(g: &'a T, step: f64) -> Geometry
where
    T: Bbox,
    Geometry: BitAnd<&'a T, Output = Geometry>,
{
    let mut tex = Geometry::new();
    let Some(bbox) = g.bbox() else { return tex };

    for x in frange(bbox.left(), bbox.right() + step, step) {
        tex.push_path(path!((x, bbox.top()), (x, bbox.bottom())));
    }

    tex & g
}

pub fn horizontal_hatch<'a, T>(g: &'a T, step: f64) -> Geometry
where
    T: Bbox,
    Geometry: BitAnd<&'a T, Output = Geometry>,
{
    let mut tex = Geometry::new();
    let Some(bbox) = g.bbox() else { return tex };

    for y in frange(bbox.top(), bbox.bottom() + step, step) {
        tex.push_path(path!((bbox.left(), y), (bbox.right(), y)));
    }

    tex & g
}
