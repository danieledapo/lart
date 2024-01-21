use std::{collections::HashSet, f64::INFINITY};

use lart::*;

sketch_parms! {
    size: u32 = 4,
    single_line: bool = false,
}

pub fn main() {
    let parms = Parms::from_cli();
    let mut doc = Sketch::new("vera_molnar").with_page(Page::A4);

    let initial_rect = doc.page_bbox().padded(-30.0);

    let padding = 0.3;

    let precision = 1.0 / 0.5;
    for (x, y, r) in initial_rect.enumerate_subdivide(parms.size, parms.size) {
        let r = r.padded(-2.5);

        let mut paths = vec![];

        let mut seen = HashSet::new();
        for _ in 0..=y {
            loop {
                let yy = doc.gen_range(r.top()..=r.bottom());
                let yy = f64::floor(yy * precision);
                if seen.insert(yy as u32) {
                    let yy = yy / precision;
                    let x0 = linterp(r.left(), r.right(), doc.gen_range(0.0..=padding));
                    let x1 = linterp(
                        r.left(),
                        r.right(),
                        1.0 - padding + doc.gen_range(0.0..=padding),
                    );
                    paths.push(path!(v(x0, yy), v(x1, yy)));
                    break;
                }
            }
        }

        seen.clear();
        for _ in 0..=x {
            loop {
                let xx = doc.gen_range(r.left()..=r.right());
                let xx = f64::floor(xx * precision);
                if seen.insert(xx as u32) {
                    let xx = xx / precision;
                    let y0 = linterp(r.top(), r.bottom(), doc.gen_range(0.0..=padding));
                    let y1 = linterp(
                        r.top(),
                        r.bottom(),
                        1.0 - padding + doc.gen_range(0.0..=padding),
                    );
                    paths.push(path!(v(xx, y0), v(xx, y1)));
                    break;
                }
            }
        }

        if !parms.single_line {
            for p in paths {
                doc.geometry(p);
            }
            continue;
        }

        let mut final_path = Path::new();
        while let Some(p) = paths.pop() {
            final_path.extend(p.iter());

            if paths.is_empty() {
                break;
            }

            let lp = p.last().unwrap();

            let mut mind = INFINITY;
            let mut closest = 0;
            let mut rev = false;
            for (i, pp) in paths.iter().enumerate() {
                let d = lp.dist2(pp.first().unwrap());
                if d < mind {
                    mind = d;
                    rev = false;
                    closest = i;
                }

                let d = lp.dist2(pp.last().unwrap());
                if d < mind {
                    mind = d;
                    rev = true;
                    closest = i;
                }
            }

            if rev {
                paths[closest].reverse();
            }

            let n = paths.len();
            paths.swap(closest, n - 1);
        }

        doc.geometry(final_path);
    }

    doc.save().unwrap();
}
