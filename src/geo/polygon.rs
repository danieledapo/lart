use crate::{bbox_union, Bbox, Path, Polygon, Rect, V};

impl Polygon {
    pub const fn new() -> Self {
        Self { areas: vec![] }
    }

    pub fn is_empty(&self) -> bool {
        self.areas.iter().all(Path::is_empty)
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
            areas: vec![p.into()],
        }
    }
}

impl From<Rect> for Polygon {
    fn from(r: Rect) -> Self {
        Self {
            areas: vec![Path::from([
                r.min(),
                V::new(r.right(), r.top()),
                r.max(),
                V::new(r.left(), r.bottom()),
            ])],
        }
    }
}
