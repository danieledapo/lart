use rand::Rng;

use crate::{Rect, V};

/// A CirclePacker allows to pack circles inside a Rect following a set of
/// parameters without ever creating overlapping circles.
///
/// It is possible to customize the minimum and maximum radius of the genrated
/// circles and whether it's allowed to generate circles inside other circles,
/// but always without overlapping circles.
pub struct CirclePacker {
    circles: Vec<(V, f64)>,
    pub bbox: Rect,
    pub margin: f64,
    pub min_radius: f64,
    pub max_radius: f64,
    pub allow_nested: bool,
}

impl CirclePacker {
    /// Create a new CirclePacker meant to pack circles in the given Rect.
    pub fn new(bbox: Rect) -> Self {
        let max_radius = f64::min(bbox.width(), bbox.height()) / 2.0;
        Self {
            circles: vec![],
            bbox,
            margin: 0.0,
            min_radius: 0.0,
            max_radius,
            allow_nested: false,
        }
    }

    /// Return all the circles packed so far.
    ///
    /// The returned slice contains a tuple of circle center and radius.
    pub fn circles(&self) -> &[(V, f64)] {
        &self.circles
    }

    /// Try to packa another circle in the current solution.
    ///
    /// Does not guarantee that a new circle is actually added to the solution.
    pub fn generate(&mut self, rng: &mut impl Rng) {
        let mut bbox = self.bbox.clone();
        bbox.pad(-self.margin);

        let c = V::in_rect(rng, &bbox);
        let mut r = f64::min(
            f64::min(self.bbox.right() - c.x, c.x - self.bbox.left()),
            f64::min(self.bbox.bottom() - c.y, c.y - self.bbox.top()),
        );
        if r < self.min_radius {
            return;
        }

        for &(cc, rr) in &self.circles {
            let mut d = cc.dist(c) - rr;
            if self.allow_nested {
                d = d.abs();
            }
            d -= self.margin;
            if d < r {
                r = d;
                if r < self.min_radius {
                    return;
                }
            }
        }

        r = r.min(self.max_radius - self.margin);
        self.circles.push((c, r));
    }
}
