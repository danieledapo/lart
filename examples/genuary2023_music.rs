use lart::*;

pub fn main() {
    let mut doc = Sketch::new("music")
        .with_page(Page::A4)
        .with_page(Page(297.0, 210.0));

    let w = doc.gen_range(0.5..=2.0);

    let rem = 29.0 - w;
    let st = rem * doc.gen_range(0.1..=0.9);

    let mut l = Path::new();

    let a = doc.gen_range(1.0..=2.0) * 5.0;

    l.push(v(-st, 0));
    for t in 0..361 {
        let t = t as f64 / 360.0;
        l.push(v(t * w, f64::sin(TAU * ease(t)) * a));
    }
    l.push(v(29.0 - st, 0));

    doc.layer(1).with_pen_width(1.0);
    doc.geometry(l);

    doc.fit_to_page(20.0);
    doc.save().unwrap();
}

fn ease(x: f64) -> f64 {
    if x < 0.5 {
        2.0 * x * x
    } else {
        1.0 - f64::powf(-2.0 * x + 2.0, 2.0) / 2.0
    }
}
