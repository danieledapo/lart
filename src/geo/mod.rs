pub mod bool_ops;
pub mod geometry;
pub mod path;
pub mod polygon;
pub mod rect;
pub(crate) mod types;
pub mod v;

pub use rect::*;
pub use types::*;

pub trait Transform {
    fn transform(&mut self, f: &mut impl FnMut(V) -> V);
}
