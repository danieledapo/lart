use lart::*;

sketch_parms! {
    regular_polygon: bool = true,
    points: u16 = 11,
    occult: bool = true,
    shading: bool = true,
    dashed_lines: bool = true,
    dash_len: f64 = 2.0,
}

fn main() {
    let parms = Parms::from_cli();
    let mut doc = Sketch::new("oned").with_page(Page::A6);
    let bbox = doc.page_bbox();

    let poly_bbox = bbox.scaled(0.8);
    let front_face = if parms.regular_polygon {
        Path::circle(
            v(0, 0),
            doc.gen_range(0.2..=0.4) * poly_bbox.radius(),
            doc.gen_range(3..=9),
        ) * Xform::rot(doc.gen_range(0.0..=TAU))
    } else {
        let pts = (0..parms.points)
            .map(|_| V::in_rect(&mut doc, &poly_bbox))
            .collect::<Vec<_>>();
        convex_hull(&pts)
    };

    let scale = doc.gen_range(0.2..=0.6);
    let back_face = front_face.clone()
        * (Xform::scale(v(scale, scale))
            * Xform::xlate(V::polar(
                doc.gen_range(0.0..=TAU),
                [1.0, -1.0].choose(&mut doc).unwrap() * doc.gen_range(0.3..=0.5) * bbox.width(),
            )));

    let xform = Xform::rect_to_rect(
        &bbox_union(&[back_face.bbox().unwrap(), front_face.bbox().unwrap()]).unwrap(),
        &bbox.padded(-20.0),
    );
    let front_face = front_face * &xform;
    let back_face = back_face * &xform;

    let mut side_faces = vec![];
    for ((s0, e0), (s1, e1)) in front_face.segments().zip(back_face.segments()) {
        side_faces.push(polygon!(s0, e0, e1, s1));
    }

    // this is not correct for non-convex polygons, but hey, it's a sketch
    let visible_faces = side_faces
        .iter()
        .filter(|f| {
            !parms.occult
                || (Geometry::from((**f).clone()) & Geometry::from(front_face.clone())).is_empty()
        })
        .cloned()
        .collect::<Vec<_>>();

    if parms.dashed_lines {
        for (s, e) in front_face
            .iter()
            .zip(back_face.iter())
            .chain(back_face.segments())
        {
            let visible = visible_faces.iter().any(|f| {
                f.segments()
                    .position(|seg| seg == (s, e) || seg == (e, s))
                    .is_some()
            });

            if visible {
                continue;
            }

            let d = (e - s).normalized();
            let mut l = 0.0;
            while l <= s.dist(e) {
                let s0 = s + d * l;
                let e0 = s0 + d * parms.dash_len.min(s.dist(e) - l);
                doc.geometry(path!(s0, e0));

                l += parms.dash_len * 2.0;
            }
        }
    }

    doc.geometry(front_face.clone().closed());

    for f in visible_faces {
        if parms.shading {
            let tex = parallel_hatch(
                &Geometry::from(f.clone()),
                doc.gen_range(0.0..=TAU),
                doc.gen_range(0.8..=1.2),
            );
            doc.geometry(tex);
        }
        doc.geometry(f);
    }

    doc.save().unwrap();
}
