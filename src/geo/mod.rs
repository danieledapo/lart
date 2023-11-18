pub mod bool_ops;
pub mod geometry;
pub mod grid;
pub mod line;
pub mod path;
pub mod rect;
pub(crate) mod types;
pub mod utils;
pub mod v;
pub mod xform;

pub use grid::Grid;
pub use line::*;
pub use rect::*;
pub use types::*;
pub use utils::*;
pub use xform::Xform;

pub const PRECISION: f64 = 1e-6;
pub const PRECISION_2: f64 = PRECISION * PRECISION;

/// Create a 2D V from the given x and y coordinates.
/// ```rust
/// # use lart::*;
/// assert_eq!(v(1, 2) + v(3, 4), v(4, 6));
/// ````
pub fn v(x: impl Into<f64>, y: impl Into<f64>) -> V {
    V::new(x.into(), y.into())
}

/// Macro to create a polygon from a list of expressions evaluating to a V.
/// ```rust
/// # use lart::*;
/// assert_eq!(polygon!((0,0), (1,0), (1,1)), path!((0,0), (1,0), (1,1), (0,0)));
/// ```
#[macro_export]
macro_rules! polygon {
    ($($d: tt)*) => { $crate::path!($($d)*).closed() };
}

/// Macro to create an open path from a list of expressions evaluating to a V.
/// ```rust
/// # use lart::*;
/// let p = path!(v(0, -1) + v(1, 0), v(1, 1), v(0, 1));
/// assert_eq!(p[0], v(1,-1));
/// assert_eq!(p[1], v(1,1));
/// assert_eq!(p[2], v(0,1));
/// ```
#[macro_export]
macro_rules! path {
    ($v0:expr $(, $vv:expr)*) => {
        $crate::Path::from([$v0 $(, $vv)*])
    };

    ($($vv:expr,)*) => {
        $crate::Path::from([$($vv,)*])
    };

    ($size:expr; $init:expr) => {{
        let mut g = $crate::Path::new();
        for _ in 0..$size {
            g.push($init);
        }
        g
    }};
}

/// Macro to create the bounding box of a set of points.
/// ```rust
/// # use lart::*;
/// let r = bbox!(v(0, 0), v(42, 0), v(1, 1) + v(0, 1));
/// assert_eq!(r.min(), v(0,0));
/// assert_eq!(r.max(), v(42, 2));
/// ```
#[macro_export]
macro_rules! bbox {
    ($v0: expr, $($vv: expr ,)*) => {{
        $crate::rect!($v0 $(, $vv)*)
    }};

    ($v0: expr $(, $vv:expr)*) => {{
        let mut b = $crate::geo::Rect::new($v0);
        $( b.expand($vv); )*
        b
    }};
}
