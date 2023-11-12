use lart::*;

sketch_parms! {
    circles: u16 = 10,
    step: f64 = 1.0,
    max_delta: f64 = 15.0,
}

pub fn main() {
    let parms = Parms::from_cli();

    let mut doc = Sketch::new("minmax")
        .with_page(Page::A4)
        .plugin(Vpype::new(&["squiggles"]));
    let (w, h) = doc.dimensions();

    let mut cp = CirclePacker::new(doc.page_bbox());
    cp.margin = 3.0;
    for _ in 0..parms.circles {
        cp.generate(&mut doc);
    }

    cp.allow_nested = true;
    for _ in 0..parms.circles {
        cp.generate(&mut doc);
    }

    let mut sp = Geometry::new();
    for &(c, r) in cp.circles() {
        sp.push_path(Path::circle(c, r, 100));
    }

    let mut tex = Geometry::new();

    for y in frange(0.0, h + parms.step, parms.step) {
        for x in frange(0.0, w + parms.step, parms.step) {
            let p0 = v(x, y);

            let t = 1.0 - p0.y / h * p0.x / w;
            let t = t + doc.gen_range(-0.1..=0.1);
            let t = t.clamp(0.0, 1.0);
            if doc.gen_bool(t) {
                let d = doc.gen_range(1.0..=parms.max_delta);
                tex.push_path(path!(p0, p0 + d));
            } else if doc.gen_bool(1.0 - t) {
                let d = -doc.gen_range(1.0..=parms.max_delta);
                tex.push_path(path!(p0, p0 + v(-d, d)));
            }
        }
    }

    let bbox = doc.page_bbox();
    doc.geometry(tex & (Geometry::from(bbox) - sp.clone()));
    doc.geometry(sp);
    // doc.geometry(doc.page_bbox());

    doc.fit_to_page(20.0);
    doc.save().unwrap();
}
