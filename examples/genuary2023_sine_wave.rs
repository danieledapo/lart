use lart::*;

sketch_parms! {
    octaves: u16 = 3,
    revs: u16 = 360,
}

fn main() {
    let mut doc = Sketch::new("sine_wave").with_page(Page::A4);
    let bbox = doc.page_bbox();

    let parms = Parms::from_cli();

    let mut paths = vec![];
    for o in 0..parms.octaves {
        let freq = (1 << o) as f64;
        let ampli = 100.0 / freq;
        let f = doc.gen_range(0.0..=TAU);

        let mut p = Path::new();
        for y in frange(bbox.top(), bbox.bottom(), 1.0) {
            let t = mapu(y, bbox.top(), bbox.bottom());
            p.push(v((TAU * t * freq + f).sin() * ampli, y));
        }

        paths.push(p);
    }

    let mut path = Path::new();
    for i in 0..paths[0].len() {
        let mut p = v(0, 0);
        for v in &paths {
            p += v[i];
        }
        path.push(p);
    }
    doc.geometry(path.clone());

    let center = V::in_rect(&mut doc, &path.bbox().unwrap());
    for _ in 0..parms.revs {
        // let center = path.bbox().unwrap().center();
        path *= Xform::xlate(-center)
            * Xform::rot(TAU / parms.revs as f64)
            * Xform::scale(v(0.99, 0.99))
            * Xform::xlate(center);
        doc.geometry(path.clone());
    }

    doc.fit_to_page(20.0);
    doc.save().unwrap();
}
