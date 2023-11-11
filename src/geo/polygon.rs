use crate::{bbox_union, polar_angles, Bbox, Path, Polygon, Rect, V};

impl Polygon {
    pub const fn new() -> Self {
        Self { areas: vec![] }
    }

    pub fn circle(c: V, r: f64, points: u16) -> Self {
        Self::from(polar_angles(points).map(|a| c + V::polar(a, r)))
    }

    pub fn is_empty(&self) -> bool {
        self.areas.iter().all(Path::is_empty)
    }

    pub fn boundary(&self) -> &Path {
        debug_assert!(self.areas.len() == 1);
        &self.areas[0]
    }

    pub fn into_boundary(mut self) -> Path {
        debug_assert!(self.areas.len() == 1);
        self.areas.swap_remove(0)
    }
}

impl Bbox for Polygon {
    fn bbox(&self) -> Option<Rect> {
        bbox_union(&self.areas)
    }
}

impl<P: Into<Path>> From<P> for Polygon {
    fn from(p: P) -> Self {
        Self {
            areas: vec![p.into().closed()],
        }
    }
}

impl<P: Into<Path>> FromIterator<P> for Polygon {
    fn from_iter<T: IntoIterator<Item = P>>(iter: T) -> Self {
        Polygon {
            areas: iter.into_iter().map(|p| p.into().closed()).collect(),
        }
    }
}
