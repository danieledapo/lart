use std::ops::{Mul, MulAssign};

use crate::{v, Geometry, Path, Rect, V};

/// An Xform is 2D transformation to scale, rotate, mirror, translate, etc... a
/// V, Path and Geometry.
///
/// Xform can be combined via the \* operator and `xform1 * xform2` can be
/// thought of applying `xform1` first and then `xform2`.
///
/// ```rust
/// # use lart::*;
/// let xform = Xform::xlate(v(10, -2)) * Xform::scale(v(2.0,1.0));
/// assert_eq!(v(12,-2) * &xform, v(44,-4));
/// assert_eq!(path!(v(0,0), v(10, 2), v(-2,5)) * xform, path!(v(20,-2), v(40,0), v(16,3)));
/// ```
#[derive(Debug, Clone)]
pub struct Xform {
    pub a: V,
    pub b: V,
    pub c: V,
}

impl Xform {
    /// Create a new Xform that does not change the input entity.
    ///
    /// ```rust
    /// # use lart::*;
    /// assert_eq!(v(42,10) * Xform::identity(), v(42,10));
    /// ```
    pub fn identity() -> Self {
        Self {
            a: v(1, 0),
            b: v(0, 1),
            c: v(0, 0),
        }
    }

    /// Create an Xform that translates the input entity by the given vector.
    ///
    /// ```rust
    /// # use lart::*;
    /// assert_eq!(v(42,10) * Xform::xlate(v(8,10)), v(50,20));
    /// ```
    pub fn xlate(d: V) -> Self {
        Self {
            a: v(1, 0),
            b: v(0, 1),
            c: d,
        }
    }

    /// Create an Xform that scales the input geometry by the given factor in
    /// both axes. The center of the scale is 0,0.
    ///
    /// ```rust
    /// # use lart::*;
    ///
    /// assert_eq!(path!(v(0,0), v(10, 2)) * Xform::scale(v(3, -1)), path!(v(0,0), v(30,-2)));
    /// ```
    pub fn scale(s: V) -> Self {
        Self {
            a: v(s.x, 0.0),
            b: v(0.0, s.y),
            c: v(0.0, 0.0),
        }
    }

    /// Create an Xform that rotates the input geometry by the given angle in
    /// radians.
    ///
    /// The center of the rotation is 0,0 which could mean that this operation
    /// ends up in also doing a translation of the input geometry.
    ///
    /// ```rust
    /// # use lart::*;
    /// let p = path!(v(15,0), v(20, 0)) * Xform::rot(TAU/4.0);
    /// assert!(p[0].almost_equal(v(0,15)));
    /// assert!(p[1].almost_equal(v(0,20)));
    /// ```
    pub fn rot(a: f64) -> Self {
        let (sa, ca) = a.sin_cos();
        Self {
            a: v(ca, sa),
            b: v(-sa, ca),
            c: v(0.0, 0.0),
        }
    }

    /// Create an Xform that rotates the input geometry by the given angle in
    /// radians pivoting on the given point.
    ///
    /// ```rust
    /// # use lart::*;
    /// let p = path!(v(10,0), v(20, 0)) * Xform::rot_on(v(15,0), TAU/4.0);
    /// assert!(p[0].almost_equal(v(15, -5)));
    /// assert!(p[1].almost_equal(v(15, 5)));
    /// ```
    pub fn rot_on(p: V, a: f64) -> Self {
        Self::xlate(-p) * Self::rot(a) * Self::xlate(p)
    }

    /// Create an Xform that scales the input geometry by the given factor in
    /// both the x and y coordinate centering on the given point.
    ///
    /// ```rust
    /// # use lart::*;
    /// let p = path!(v(10,5), v(20, -2)) * Xform::scale_on(v(10,5), v(2, -1));
    /// assert_eq!(p, path!(v(10,5), v(30,12)));
    /// ```
    pub fn scale_on(p: V, s: V) -> Self {
        Self::xlate(-p) * Self::scale(s) * Self::xlate(p)
    }

    /// Create a rigid Xform that scales and translates so that the src Rect is
    /// transformed into the dst Rect.
    ///
    /// This transformation is rigid which means that the aspect ratio of the
    /// two Rect is preserved.
    ///
    /// ```rust
    /// # use lart::*;
    /// let src = Rect::with_dimensions(v(2, 4), 8.0, 16.0);
    /// let dst = Rect::with_dimensions(v(8, 10), 4.0, 4.0);
    /// assert_eq!(src.closed_path() * Xform::rect_to_rect(&src, &dst), polygon!(v(9,10), v(11,10), v(11,14), v(9,14)));
    /// ```
    pub fn rect_to_rect(src: &Rect, dst: &Rect) -> Self {
        let sf = f64::min(dst.width() / src.width(), dst.height() / src.height());

        Self::xlate(-src.center()) * Self::scale(v(sf, sf)) * Self::xlate(dst.center())
    }

    /// Create an Xform that scales and translates so that the src Rect is
    /// transformed into the dst Rect.
    ///
    /// This transformation does not keep the aspect ratio of the two Rect, but
    /// it stretches and scales the src rect to be exactly the dst Rect.
    ///
    /// ```rust
    /// # use lart::*;
    /// let src = Rect::with_dimensions(v(2, 4), 8.0, 16.0);
    /// let dst = Rect::with_dimensions(v(8, 10), 4.0, 4.0);
    /// assert_eq!(src.closed_path() * Xform::stretched_rect_to_rect(&src, &dst), dst.closed_path());
    pub fn stretched_rect_to_rect(src: &Rect, dst: &Rect) -> Self {
        Self::xlate(-src.center())
            * Self::scale(v(dst.width() / src.width(), dst.height() / src.height()))
            * Self::xlate(dst.center())
    }
}

impl<'a> MulAssign<&'a Xform> for Xform {
    fn mul_assign(&mut self, m: &'a Xform) {
        self.a = self.a.x * m.a + self.a.y * m.b;
        self.b = self.b.x * m.a + self.b.y * m.b;
        self.c = self.c * m;
    }
}

impl<'a> MulAssign<&'a Xform> for V {
    fn mul_assign(&mut self, rhs: &'a Xform) {
        *self = self.x * rhs.a + self.y * rhs.b + rhs.c;
    }
}

impl<'a> MulAssign<&'a Xform> for Path {
    fn mul_assign(&mut self, rhs: &'a Xform) {
        for p in self.iter_mut() {
            *p *= rhs;
        }
    }
}

impl<'a> MulAssign<&'a Xform> for Geometry {
    fn mul_assign(&mut self, rhs: &'a Xform) {
        for p in &mut self.paths {
            *p *= rhs;
        }
    }
}

macro_rules! impl_trivial_xform_helpers {
    ($t: ident) => {
        impl MulAssign<Xform> for $t {
            fn mul_assign(&mut self, rhs: Xform) {
                *self *= &rhs;
            }
        }

        impl Mul<Xform> for $t {
            type Output = Self;

            fn mul(self, rhs: Xform) -> Self::Output {
                self * &rhs
            }
        }

        impl<'a> Mul<&'a Xform> for $t {
            type Output = Self;

            fn mul(mut self, rhs: &'a Xform) -> Self::Output {
                self *= rhs;
                self
            }
        }
    };
}

impl_trivial_xform_helpers!(Xform);
impl_trivial_xform_helpers!(V);
impl_trivial_xform_helpers!(Path);
impl_trivial_xform_helpers!(Geometry);
