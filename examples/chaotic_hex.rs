use std::collections::HashSet;

use lart::*;

sketch_parms! {
    page: Page = Page::A4,
    iterations: usize = 15,
    birth_rule: u64 = 0,
    dying_rule: u64 = 0,
    fill: bool = false,
    seed_square_size: i32 = 50,
    smoothness: f64 = 0.5,
}

fn main() {
    let mut parms = Parms::from_cli();
    let mut doc = Sketch::new("chaotic_hex").with_page(parms.page);

    let mut world: HashSet<(i32, i32)> = HashSet::new();

    for _ in 0..parms.seed_square_size * parms.seed_square_size * 2 / 2 {
        world.insert((
            doc.gen_range(-parms.seed_square_size..=parms.seed_square_size),
            doc.gen_range(-parms.seed_square_size..=parms.seed_square_size),
        ));
    }

    if parms.birth_rule == 0 {
        parms.birth_rule = doc.gen();
    }
    if parms.dying_rule == 0 {
        parms.dying_rule = doc.gen();
    }

    let mut new_world: HashSet<(i32, i32)> = HashSet::new();
    for _ in 0..parms.iterations {
        let bbox =
            bbox_union(world.iter().map(|(x, y)| v(*x, *y))).unwrap_or_else(|| bbox!(v(0, 0)));

        new_world.clear();

        for y in bbox.top() as i32 - 1..=bbox.bottom() as i32 + 1 {
            for x in bbox.left() as i32 - 1..=bbox.right() as i32 + 1 {
                let alive = world.get(&(x, y)).is_some();

                let bit = [
                    (x - 1, y),
                    (x + 1, y),
                    (x, y - 1),
                    (x, y + 1),
                    (x + 1, y - 1),
                    (x - 1, y + 1),
                ]
                .into_iter()
                .enumerate()
                .map(|(bi, p)| if world.contains(&p) { 1 << bi } else { 0 })
                .sum::<u64>();

                let alive = if alive {
                    (parms.dying_rule & bit) == 0
                } else {
                    (parms.birth_rule & bit) != 0
                };

                if alive {
                    new_world.insert((x, y));
                }
            }
        }

        std::mem::swap(&mut world, &mut new_world);
    }

    let radius = 0.5;
    let hex_dir =
        v(0.5, f64::sqrt(3.0) / 2.0) * f64::hypot(f64::sqrt(3.0) * radius / 2.0, 1.5 * radius);

    let project = |i, j| {
        let (i, j) = (f64::from(i), f64::from(j));
        v(i - j, i + j) * hex_dir
    };

    let mut g = Geometry::new();
    for &(i, j) in &world {
        let c = project(i, j);

        g.push_path(Path::circle(c, radius, 6) * Xform::rot_on(c, TAU / 12.0));
    }

    let mut g = g.union_all();
    for _ in 0..2 {
        g = g.chaikin(parms.smoothness).simplify(0.0);
    }
    let g = g.clone() * Xform::rect_to_rect(&g.bbox().unwrap(), &doc.page_bbox().padded(-20.0));

    if parms.fill {
        doc.geometry(parallel_hatch(&g, TAU / 8.0, 0.5));
    }

    doc.geometry(g);

    doc.save().unwrap();
}
