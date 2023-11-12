#[cxx::bridge]
pub mod ffi {
    #[derive(Default, Clone, Copy, PartialEq)]
    pub struct V {
        x: f64,
        y: f64,
    }

    #[derive(Clone, PartialEq)]
    pub struct Path {
        points: Vec<V>,
    }

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
