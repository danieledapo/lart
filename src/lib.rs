/// # Lart
///
/// Lart is a library for generating 2D vector art in Rust meant to be plotted.
///
/// It provides a set of primitives geometric types and operations on them
/// alongside a "canvas" to draw on.
///
/// Here's an example that showcases:
/// - automatic command line generation and parsing via the `sketch_parms` macro
/// - boolean operations on geometries (union, intersection, difference)
/// - polygon buffering
///
/// ```rust,no_run
/// # use lart::*;
///
/// sketch_parms! {
///     lines: u8 = 2,
///     points: u16 = 10,
/// }
///
/// fn main() {
///     let parms = Parms::from_cli();
///     let mut doc = Sketch::new("example").with_page(Page::A6);
///
///     let bbox = doc.page_bbox();
///
///     let mut drawn = Geometry::new();
///
///     for _ in 0..parms.lines {
///         let mut p = Path::new();
///         for _ in 0..parms.points {
///             p.push(V::in_rect(&mut doc, &bbox));
///         }
///         let g = Geometry::from(p).buffer(-2.0);
///         let g = g - &drawn;
///         drawn = drawn | &g;
///         doc.geometry(g);
///     }
///
///     doc.fit_to_page(20.0);
/// }
/// ```
///
pub mod algo;
pub mod geo;
pub mod sketch;

pub use algo::*;
pub use geo::*;
pub use sketch::*;

pub use std::f64::consts::{PI, TAU};
