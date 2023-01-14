pub use lart::*;

sketch_parms! {
    grid_divs: u32 = 3,
    variants: u8 = 2,
    xdivs: u32 = 10,
    ydivs: u32 = 20,
}

pub fn main() {
    let mut doc = Sketch::new("tessellation").with_page(Page::A4);
    let Parms {
        grid_divs,
        variants,
        xdivs,
        ydivs,
    } = Parms::from_cli();

    let mut blueprints = vec![];

    let mut a = Geometry::new();

    for r in doc.page_bbox().subdivide(xdivs, ydivs) {
        if blueprints.is_empty() {
            for _ in 0..variants {
                let mut g = Geometry::new();
                for r in r.subdivide(grid_divs, grid_divs) {
                    if doc.gen_bool(0.5) {
                        g = g | Geometry::from(Polygon::from(r));
                    }
                }
                blueprints.push(g);
            }
        }

        // let g = blueprints.choose(&mut doc).unwrap();
        let g = blueprints[0].clone();
        blueprints.rotate_left(1);

        a = a | g.clone() * Xform::xlate(r.min());
    }

    let mut t = Geometry::new();
    for x in frange(0.0, 210.0, 1.0) {
        t.push_path(Path::from([v(x, 0), v(x, 300)]));
    }
    doc.geometry(t & &a);

    // let mut t = Geometry::new();
    // for x in frange(0.0, 300.0, 1.0) {
    //     t.push_path(Path::from([v(0, x), v(210, x)]));
    // }
    // doc.geometry(t & (Geometry::from(Polygon::from(doc.page_bbox())) - &a));

    doc.geometry(a);

    doc.fit_to_page(20.0);
    doc.save().unwrap();
}
