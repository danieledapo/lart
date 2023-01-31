pub mod bool_ops;
pub mod geometry;
pub mod path;
pub mod polygon;
pub mod rect;
pub(crate) mod types;
pub mod utils;
pub mod v;
pub mod xform;

pub use rect::*;
pub use types::*;
pub use utils::*;
pub use xform::Xform;

pub fn v(x: impl Into<f64>, y: impl Into<f64>) -> V {
    V::new(x.into(), y.into())
}

#[macro_export]
macro_rules! polygon {
    ($($d: tt)*) => { $crate::_build_path_like!($crate::geo::Polygon, $($d)*) };
}

#[macro_export]
macro_rules! path {
    ($($d: tt)*) => { $crate::_build_path_like!($crate::geo::Path, $($d)*) };
}

#[macro_export]
macro_rules! rect {
    ($v0: expr, $($vv: expr ,)*) => {{
        $crate::rect!($v0 $(, $vv)*)
    }};

    ($v0: expr $(, $vv:expr)*) => {{
        let mut b = $crate::geo::Rect::new($v0);
        $( b.expand($vv); )*
        b
    }};
}

#[macro_export]
macro_rules! _build_path_like {
    ($t: ty, $v0:expr $(, $vv:expr)*) => {
        <$t as From<_>>::from([$v0 $(, $vv)*])
    };

    ($t: ty, $($vv:expr,)*) => {
        <$t as From<_>>::from([$($vv,)*])
    };

    ($t: ty, $size:expr; $init:expr) => {{
        let mut g = <$t>::new();
        for _ in 0..$size {
            g.push($init);
        }
        g
    }};
}
