use lart::*;

sketch_parms! {
    points: u16 = 30,
    reflections: u16 = 3
}

fn main() {
    let mut doc = Sketch::new("reflections").with_page(Page::A4);
    let bbox = doc.page_bbox();

    let parms = Parms::from_cli();

    let mut poly = Path::new();
    for _ in 0..parms.points {
        poly.push(V::in_rect(&mut doc, &bbox));
    }

    let mut poly = Geometry::from(Polygon::from(poly));

    for _ in 0..parms.reflections {
        let a0 = doc.gen_range(0.0..=PI);
        // let a0 = i as f64 * PI / 2.13;

        poly *= Xform::rot(-a0);
        let bbox = poly.bbox().unwrap();
        poly.push_path(Path::from([
            v(-bbox.width() / 2.0, 0.0),
            v(bbox.width() / 2.0, 0.0),
        ]));

        poly = poly.clone() | (poly * Xform::scale(v(1, -1)));

        poly *= Xform::rot(a0);
    }

    let bbox = poly.bbox().unwrap();
    let mut tex = Geometry::new();
    for y in frange(bbox.top(), bbox.bottom(), 3.0) {
        tex.push_path(Path::from([v(bbox.left(), y), v(bbox.right(), y)]));
    }

    doc.geometry(tex & &poly);
    doc.geometry(poly);

    doc.fit_to_page(20.0);
    doc.save().unwrap();
}
