use crate::{Geometry, Path};

pub trait Chaikin {
    /// Smooth a geometry using the [Chaikin algorithm][0] using the given ratio
    /// in `0..1`. The higher the ratio the higher the smoothness of the final
    /// curve.
    ///
    /// [0]:
    ///     https://www.cs.unc.edu/~dm/UNC/COMP258/LECTURES/Chaikins-Algorithm.pdf
    fn chaikin(&self, ratio: f64) -> Self;
}

impl Chaikin for Path {
    fn chaikin(&self, ratio: f64) -> Self {
        chaikin_impl(self, ratio)
    }
}

impl Chaikin for Geometry {
    fn chaikin(&self, ratio: f64) -> Self {
        let mut g = Geometry::new();
        g.push_paths(self.paths.iter().map(|p| p.chaikin(ratio)));
        g
    }
}

fn chaikin_impl(path: &Path, ratio: f64) -> Path {
    if path.len() <= 2 || ratio == 0.0 || ratio == 1.0 {
        return path.clone();
    }

    let closed = path.is_closed();

    let ratio = if ratio <= 0.5 { ratio } else { 1.0 - ratio };

    let mut new = Path::with_capacity(path.len() * 2 + if closed { 2 } else { 0 });

    // if the path is not closed make sure to preserve the endpoints so that we
    // don't shrunk the path, if it's closed then throw them away as it's
    // expected that the final shape is actually smaller than the original as
    // the corners have been smoothed
    if !closed {
        new.push(path[0]);
    }

    for (s, e) in path.segments() {
        if e == s {
            new.push(s);
            continue;
        }

        let d = e - s;
        new.push(s + ratio * d);
        if ratio != 0.5 {
            new.push(e - ratio * d);
        }
    }

    if !closed {
        new.push(path.last().unwrap());
    } else {
        // if the path is closed be sure to close the smoothed path too
        new.push(new[0]);
    }

    new
}
