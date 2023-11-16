use lart::*;

sketch_parms! {
    page: Page = Page::A4,
    sides: u16 = 4,
    vor: bool = false,
    vor_points: u16 = 200,
}

fn main() {
    let parms = Parms::from_cli();
    let mut doc = Sketch::new("shadow").with_page(parms.page);

    let bbox = doc.page_bbox().padded(-20.0);
    let r = bbox.width().min(bbox.height()) / 2.0;
    let bbox = bbox!(bbox.center() - r, bbox.center() + r);

    let n = doc.gen_range(3..=parms.sides);

    let mut poly = Geometry::from(Path::circle(bbox.center(), r, n));
    poly *= Xform::rot(doc.gen_range(0.0..=PI));
    poly *= Xform::rect_to_rect(&poly.bbox().unwrap(), &bbox.padded(-20.0));

    let t0 = doc.gen_range(0.3..=0.7);
    let t1 = doc.gen_range(0.3..=0.7);
    let p0 = v(bbox.left(), bbox.top() + t0 * bbox.height());
    let p1 = v(bbox.right(), bbox.top() + t1 * bbox.height());

    let clip_above = Geometry::from(polygon!(bbox.min(), p0, p1, v(bbox.right(), bbox.top())));
    let clip_below = Geometry::from(polygon!(p0, p1, bbox.max(), v(bbox.left(), bbox.bottom())));

    let above = &poly & &clip_above;
    let below = &poly & &clip_below;

    let mut tex_above = Geometry::new();
    let mut tex_below = Geometry::new();

    if !parms.vor {
        for x in frange(-bbox.width() * 2.0, bbox.width() * 2.0, 1.5) {
            tex_above.push_path(path!(v(x, bbox.top()), v(x + bbox.width(), bbox.bottom())));
        }
        for x in frange(-bbox.width() * 2.0, bbox.width() * 2.0, 1.0) {
            tex_below.push_path(path!(v(x, bbox.top()), v(x - bbox.width(), bbox.bottom())));
        }
    } else {
        for x in frange(-bbox.width() * 2.0, bbox.width() * 2.0, 1.5) {
            tex_above.push_path(path!(v(x, bbox.top()), v(x + bbox.width(), bbox.bottom())));
        }
        doc.geometry(tex_above & &above);
        tex_above = Geometry::new();

        for x in frange(-bbox.width() * 2.0, bbox.width() * 4.0, 3.0) {
            tex_below.push_path(path!(v(x, bbox.top()), v(x - bbox.width(), bbox.bottom())));
        }
        doc.geometry(tex_below & (clip_below - below.clone()));
        tex_below = Geometry::new();

        let mut pts = vec![];
        for _ in 0..parms.vor_points {
            pts.push(V::in_rect(&mut doc, &bbox));
        }

        for g in voronoi(&pts, &bbox).paths {
            let g = Geometry::from(g);

            let a = doc.gen_range(0.0..=TAU);
            let step = doc.gen_range(2..=10) as f64 / 2.0;

            let tex = parallel_hatch(&g, a, step);
            tex_above.append(&tex);
            tex_below.append(&tex);
        }
    }

    doc.geometry(&tex_above & (clip_above - above.clone()));
    doc.geometry(&tex_below & &below);
    doc.geometry(above);
    doc.geometry(below);
    doc.geometry(bbox);
    doc.geometry(path!(p0, p1,));

    // doc.fit_to_page(20.0);
    doc.save().unwrap();
}
