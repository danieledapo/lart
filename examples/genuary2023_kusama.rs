use lart::*;

sketch_parms! {
    circles: u16 = 100,
    max_radius: f64 = 30.0,
    min_radius: f64 = 2.0,
    lines: u16 = 50,
    margin: f64 = 5.0,
}

pub fn main() {
    let parms = Parms::from_cli();
    let mut doc = Sketch::new("kusama")
        .with_page(Page::A4)
        .plugin(Vpype::new(&["squiggles"]));

    let (w, h) = doc.dimensions();

    let mut cp = CirclePacker::new(Rect::with_dimensions(v(0.0, 0.0), w, h));
    cp.margin = parms.margin;

    cp.min_radius = parms.min_radius;
    cp.max_radius = parms.max_radius;
    for _ in 0..parms.circles {
        cp.generate(&mut doc);
    }

    let mut g1 = Geometry::new();
    let mut g2 = Geometry::new();
    for &(c, r) in cp.circles() {
        let cc = Path::circle(c, r, 60);
        if doc.gen_bool(0.5) {
            g1.push_path(cc);
        } else {
            g2.push_path(cc);
        }
    }

    let mut t1 = Geometry::new();
    for x in frange(0.0, w + 1.0, 1.0) {
        t1.push_path(path!(v(x, 0), v(x, h)));
    }
    let mut t2 = Geometry::new();
    for y in frange(0.0, h + 2.5, 2.5) {
        t2.push_path(path!(v(0, y), v(w, y)));
    }

    doc.geometry(t1 & &g1);
    doc.geometry(t2 & &g2);
    doc.geometry(g1);
    doc.geometry(g2);

    doc.fit_to_page(20.0);
    doc.save().unwrap();
}
