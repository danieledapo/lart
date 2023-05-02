use std::collections::HashSet;

use lart::*;

sketch_parms! {
    gridw: u16 = 5,
    gridh: u16 = 5,
    rows: u32 = 20,
    cols: u32 = 4,
    minn: u16 = 15,
    maxn: u16 = 40,
    debug: bool = false,
}

fn main() {
    let parms = Parms::from_cli();

    let mut doc = Sketch::new("alien_writings")
        .with_page(Page::A4)
        .with_page(Page(210.0, 270.0));

    let font_bbox = rect!(v(0, 0), v(parms.gridw, parms.gridh));

    for r in doc
        .page_bbox()
        .padded(-20.0)
        .subdivide(parms.cols, parms.rows)
    {
        let n = doc.gen_range(parms.minn..=parms.maxn);
        let xform = Xform::rect_to_rect(&font_bbox, &r);

        let mut seen = HashSet::new();
        let mut p = vec![];
        let mut prevd = (0, 0);
        for _ in 0..100000 {
            if seen.len() >= usize::from(n) {
                break;
            }

            if p.is_empty() {
                let pp = (
                    doc.gen_range(0..i32::from(parms.gridw)),
                    doc.gen_range(0..i32::from(parms.gridh)),
                );
                if seen.insert(pp) {
                    p.push(pp);
                }
                continue;
            }

            let (x, y) = *p.last().unwrap();
            let (dx, dy) = *[
                (1, 0),
                (-1, 0),
                (0, 1),
                (0, -1),
                (1, -1),
                (1, 1),
                (-1, 1),
                (-1, -1),
            ]
            .choose(&mut doc)
            .unwrap();

            if !font_bbox.contains(v(x + dx, y + dy)) {
                continue;
            }
            if !seen.insert((x + dx, y + dy)) {
                continue;
            }

            if (dx, dy) != prevd {
                let path = Path::from(p.clone()) * &xform;
                let o = v(-prevd.1, prevd.0).normalized();
                if doc.gen_bool(0.5) {
                    for t in frange(-0.5, 0.55, 0.2) {
                        doc.geometry(path.clone() * Xform::xlate(o * t));
                    }
                } else {
                    doc.geometry(path.clone());
                }

                p = vec![*p.last().unwrap()];
            }

            p.push((x + dx, y + dy));
            prevd = (dx, dy);
        }

        if parms.debug {
            doc.geometry(r);
        }
    }

    // doc.fit_to_page(20.0);
    doc.save().unwrap();
}
