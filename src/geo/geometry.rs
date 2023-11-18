use crate::{bbox_union, Bbox, Geometry, Path, Rect};

impl Geometry {
    /// Create an empty Geometry.
    pub const fn new() -> Self {
        Self { paths: vec![] }
    }

    /// Create a Geometry from all the given paths.
    pub const fn from_paths(paths: Vec<Path>) -> Self {
        Self { paths }
    }

    /// Return true when the geometry has no points.
    pub fn is_empty(&self) -> bool {
        self.paths.iter().all(Path::is_empty)
    }

    /// Return all the paths of the Geometry.
    pub fn paths(&self) -> &[Path] {
        &self.paths
    }

    /// Add the given path to the Geometry.
    pub fn push_path(&mut self, p: Path) {
        self.paths.push(p);
    }

    /// Extend the Geometry with all the paths in the iterator.
    pub fn push_paths(&mut self, p: impl IntoIterator<Item = Path>) {
        self.paths.extend(p);
    }

    /// Add all of the paths in the given Geometry into this one.
    pub fn append(&mut self, o: &Self) {
        self.paths.extend_from_slice(&o.paths);
    }
}

impl Bbox for Geometry {
    fn bbox(&self) -> Option<Rect> {
        bbox_union(self.paths.iter())
    }
}

impl From<Path> for Geometry {
    fn from(p: Path) -> Self {
        Geometry { paths: vec![p] }
    }
}

impl From<Rect> for Geometry {
    fn from(r: Rect) -> Self {
        Geometry::from(r.closed_path())
    }
}
