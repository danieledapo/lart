use lart::*;

sketch_parms! {
    n: u16 = 20,
    in_circle: bool = false,
}

pub fn main() {
    let mut doc = Sketch::new("textile").with_page(Page::A4);

    let bbox = doc.page_bbox();

    let parms = Parms::from_cli();

    let mut pts = vec![];
    for _ in 0..parms.n {
        if parms.in_circle {
            pts.push(
                bbox.center()
                    + V::polar(
                        doc.gen_range(0.0..=TAU),
                        doc.gen_range(0.5..=1.0) * bbox.height() / 2.1,
                    ),
            );
        } else {
            pts.push(V::in_rect(&mut doc, &bbox));
        }
    }

    for g in voronoi(&pts, &Path::from(bbox)).polygons {
        let bbox = match g.bbox() {
            None => continue,
            Some(bbox) => bbox,
        };

        let mut tex = Geometry::new();

        let a = doc.gen_range(0.0..=TAU);
        let step = doc.gen_range(2..=10) as f64 / 2.0;

        let d = V::polar(a, 1.0);
        let pd = V::polar(a + PI / 2.0, 1.0);
        let r = 0.5 * f64::hypot(bbox.width(), bbox.height());

        let p0 = bbox.center() + d * r;
        let p1 = bbox.center() - d * r;
        for dd in frange(-r, r + step, step) {
            let o = pd * dd;
            tex.push_path(path!(p0 + o, p1 + o));
        }

        doc.geometry(tex & &g);
        doc.geometry(g);
    }

    doc.fit_to_page(20.0);
    doc.save().unwrap();
}
