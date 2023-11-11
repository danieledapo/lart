use crate::{bbox_union, Bbox, Geometry, Path, Polygon, Rect};

impl Geometry {
    pub const fn new() -> Self {
        Self { paths: vec![] }
    }

    pub fn from_polygons(polygons: Vec<Polygon>) -> Self {
        Self {
            paths: polygons.into_iter().map(|p| p.into_boundary()).collect(),
        }
    }

    pub const fn from_paths(paths: Vec<Path>) -> Self {
        Self { paths }
    }

    pub fn is_empty(&self) -> bool {
        self.paths.iter().all(Path::is_empty)
    }

    pub fn lines(&self) -> &[Path] {
        &self.paths
    }

    pub fn push_polygon(&mut self, p: Polygon) {
        self.paths.push(p.into_boundary());
    }

    pub fn push_polygons(&mut self, p: impl IntoIterator<Item = Polygon>) {
        self.paths.extend(p.into_iter().map(Polygon::into_boundary));
    }

    pub fn push_path(&mut self, p: Path) {
        self.paths.push(p);
    }

    pub fn push_paths(&mut self, p: impl IntoIterator<Item = Path>) {
        self.paths.extend(p);
    }

    pub fn paths(&self) -> &[Path] {
        &self.paths
    }

    pub fn extend(&mut self, o: &Self) {
        self.paths.extend_from_slice(&o.paths);
    }
}

impl Bbox for Geometry {
    fn bbox(&self) -> Option<Rect> {
        bbox_union(self.paths.iter())
    }
}

impl From<Polygon> for Geometry {
    fn from(p: Polygon) -> Self {
        Geometry {
            paths: vec![p.into_boundary()],
        }
    }
}

impl From<Path> for Geometry {
    fn from(p: Path) -> Self {
        Geometry { paths: vec![p] }
    }
}

impl From<Rect> for Geometry {
    fn from(r: Rect) -> Self {
        Geometry::from(Polygon::from(r))
    }
}
