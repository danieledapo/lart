use lart::{split::split_convex_polygon, *};

sketch_parms! {
    splits: u16 = 2,
    shape: Choice = Choice::new("square", &["square", "circle"]),
    cut_mode: u16 = 0,
}

fn main() {
    let parms = Parms::from_cli();

    let mut doc = Sketch::new("split").with_page(Page::A4);
    let bbox = doc.page_bbox();

    let shape = match parms.shape.value() {
        "square" => bbox.scaled(0.8).closed_path(),
        "circle" => Path::circle(bbox.center(), bbox.radius() * 0.8, 120),
        _ => panic!("unknown shape"),
    };

    let mut shapes = vec![shape];

    for i in 0..parms.splits {
        let poly_bbox = bbox_union(&shapes).unwrap();
        let (l, r) = (poly_bbox.left(), poly_bbox.right());
        let (b, t) = (poly_bbox.bottom(), poly_bbox.top());

        let p0;
        let p1;
        if parms.cut_mode == 0 {
            let a0 = doc.gen_range(0.0..=TAU);
            let a1 = a0 + TAU / 2.0 + TAU * doc.gen_range(-6.0..=6.0);

            p0 = V::polar(a0, 10.0) + poly_bbox.center();
            p1 = V::polar(a1, 10.0) + poly_bbox.center();
        } else {
            if i % 2 == 0 {
                let mut x0 = linterp(l, r, doc.gen_range(0.2..=0.45));
                let mut x1 = linterp(l, r, doc.gen_range(0.55..=0.8));
                if doc.gen_bool(0.5) {
                    std::mem::swap(&mut x0, &mut x1);
                }

                p0 = v(x0, t);
                p1 = v(x1, b);
            } else {
                let mut y0 = linterp(t, b, doc.gen_range(0.2..=0.45));
                let mut y1 = linterp(t, b, doc.gen_range(0.55..=0.8));
                if doc.gen_bool(0.5) {
                    std::mem::swap(&mut y0, &mut y1);
                }

                p0 = v(l, y0);
                p1 = v(r, y1);
            }
        }

        let displacement = [1.0, -1.0].choose(&mut doc).unwrap() * doc.gen_range(5.0..=30.0);

        shapes = shapes
            .into_iter()
            .flat_map(|s| split_convex_polygon(&s, (p0, p1)))
            .map(|s| {
                let p = s.centroid();
                let dd: f64 = p.orient(p0, p1).signum();

                s * Xform::xlate((p1 - p0).normalized() * (displacement * dd))
            })
            .collect();
    }

    let xform = Xform::rect_to_rect(&bbox_union(&shapes).unwrap(), &bbox.padded(-20.0));

    let mut to_texture = Geometry::new();
    for s in shapes {
        let s = s * &xform;
        let g = Geometry::from(s.closed()).buffer(-doc.gen_range(1.0..=5.0));

        for p in g.paths() {
            if p.area() < 10.0 {
                continue;
            }

            if doc.gen_bool(0.5) {
                to_texture.push_path(p.clone());
            }

            doc.geometry(p.clone());
        }
    }

    doc.geometry(parallel_hatch(&to_texture, -TAU / 8.0, 2.0));

    doc.save().unwrap();
}
