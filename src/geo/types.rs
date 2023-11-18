#[cxx::bridge]
pub mod ffi {
    /// A 2D vector with an x and y coordinate.
    ///
    /// ```rust
    /// # use lart::*;
    /// assert_eq!(v(3, 4).norm2(), 25.0);
    /// assert_eq!((v(3, 7) + v(12,3)).dist2(v(16,10)), 1.0);
    /// ```
    #[derive(Default, Clone, Copy, PartialEq)]
    pub struct V {
        x: f64,
        y: f64,
    }

    /// A Path is a sequence of points, if the last point is equal to the first
    /// it is a polygon.
    ///
    /// ```rust
    /// # use lart::*;
    /// let mut p = path!(v(0,0), v(1,2), v(3,4));
    /// p.reverse();
    /// p.pop();
    /// p.push(v(-2, 2));
    /// assert_eq!(p.points(), &[v(3,4), v(1,2), v(-2,2)]);
    /// assert_eq!(p.first(), Some(v(3,4)));
    /// assert_eq!(p.last(), Some(v(-2,2)));
    /// ```
    #[derive(Clone, PartialEq)]
    pub struct Path {
        points: Vec<V>,
    }

    /// A Geometry is a collection of paths that represent both open lines and
    /// polygons.
    ///
    /// Geometry objects are used primarily for efficient boolean operations and
    /// buffering operations.
    ///
    /// ```rust,no_run
    /// # use lart::*;
    /// let mut doc = Sketch::new("geo");
    /// let mut g1 = Geometry::from(polygon!(v(0,0), v(10, 0), v(5, 8)));
    /// let mut g2 = g1.clone().buffer(-2.0);
    /// doc.geometry(&g1 - &g2);
    /// doc.geometry(&g1 & &g2);
    /// doc.geometry(&g1 | &g2);
    /// ```
    #[derive(Debug, Clone)]
    pub struct Geometry {
        paths: Vec<Path>,
    }

    unsafe extern "C++" {
        include!("lart/include/lart.h");

        type Clipper;

        fn new_clipper() -> UniquePtr<Clipper>;
        fn add_subject(self: Pin<&mut Clipper>, polygon: &Path);
        fn add_clip(self: Pin<&mut Clipper>, polygon: &Path);

        fn union_(self: Pin<&mut Clipper>) -> Geometry;
        fn intersection(self: Pin<&mut Clipper>) -> Geometry;
        fn difference(self: Pin<&mut Clipper>) -> Geometry;
        fn symmetric_difference(self: Pin<&mut Clipper>) -> Geometry;

        fn buffer(geo: &Geometry, delta: f64) -> Geometry;
    }
}

pub use ffi::*;
