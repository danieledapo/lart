use lart::*;

pub fn main() {
    let mut doc = Sketch::new("asemic").with_page(Page::A4);

    doc.layer(1).with_pen_width(0.5);

    for _ in 0..2 {
        for r in doc.page_bbox().subdivide(2, 2) {
            let r = r.padded(-5.0);

            let mut p = Path::new();

            for r in r.subdivide(1, 30) {
                if p.is_empty() {
                    p.push(V::in_rect(&mut doc, &r.padded(0.0)));
                } else {
                    let l = p.last().unwrap();

                    p.push(l + V::polar(doc.gen_range(0.0..=TAU), doc.gen_range(10.0..=30.0)));
                }
            }

            doc.geometry(Geometry::from(spline::sample(&p, 0.1)));
        }
    }

    doc.fit_to_page(20.0);
    doc.save().unwrap();
}
