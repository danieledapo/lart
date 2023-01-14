use lart::*;

pub fn main() {
    let mut doc = Sketch::new("suprematism").with_page(Page::A4);
    let (w, h) = doc.dimensions();

    let mut g = Geometry::from(Polygon::from(Rect::with_dimensions(v(0, 0), w, h)));

    let t0 = doc.gen_range(0.1..=0.9);
    g = g - Geometry::from(Path::from([
        v(t0 * w, 0),
        v(t0 * w + w * doc.gen_range(-0.1..=0.1), h),
    ]))
    .buffer(5.0);

    let t0 = doc.gen_range(0.1..=0.9);
    g = g - Geometry::from(Path::from([
        v(0, t0 * h),
        v(w, t0 * h + h * doc.gen_range(-0.1..=0.1)),
    ]))
    .buffer(5.0);

    g.polygons
        .sort_by_cached_key(|p| F64Key(p.bbox().unwrap().area()));

    let small = Geometry::from(g.polygons[0].clone())
        * Xform::scale_on(g.polygons[0].bbox().unwrap().center(), v(1.1, 1.1));
    let small =
        small.clone() * Xform::rot_on(small.bbox().unwrap().center(), doc.gen_range(0.0..=PI));
    let big = Geometry::from(g.polygons.last().unwrap().clone())
        * Xform::scale_on(
            g.polygons.last().unwrap().clone().bbox().unwrap().center(),
            v(1.1, 1.1),
        );
    let big = big.clone() * Xform::rot_on(big.bbox().unwrap().center(), doc.gen_range(0.0..=PI));

    let mut t1 = Geometry::new();
    for x in frange(-w * 4.0, w * 4.0, 1.0) {
        t1.push_path(Path::from([v(x, -h), v(x, h * 3.0)]));
    }

    let mut t2 = Geometry::new();
    for x in frange(-w * 4.0, w * 4.0, 4.0) {
        t2.push_path(Path::from([v(x, -h), v(x, h * 3.0)]));
    }

    doc.geometry(t1 & &small);
    doc.geometry(t2 & &big);

    // doc.geometry(big);
    // doc.geometry(small);

    doc.fit_to_page(20.0);
    doc.save().unwrap();
}
