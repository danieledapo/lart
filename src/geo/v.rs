use rand::{distributions::uniform::SampleRange, Rng};

use crate::{Bbox, Rect, PRECISION_2, V};

impl V {
    /// Create a new V with the given x and y coordinates.
    /// ```rust
    /// # use lart::*;
    /// let p = V::new(-12.0, 88.0);
    /// assert_eq!(p.x, -12.0);
    /// assert_eq!(p.y, 88.0);
    /// ```
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Create a new random V inside the given x and y ranges.
    /// ```rust
    /// # use lart::*;
    /// let mut doc = Sketch::new("in_range");
    /// let v = V::in_range(&mut doc, 10.0..=20.0, -2.0..2.0);
    /// assert!(v.x >= 10.0 && v.x <= 20.0);
    /// assert!(v.y >= -2.0 && v.y < 2.0);
    /// ```
    pub fn in_range(
        rng: &mut impl Rng,
        x: impl SampleRange<f64>,
        y: impl SampleRange<f64>,
    ) -> Self {
        Self::new(rng.gen_range(x), rng.gen_range(y))
    }

    /// Create a new random V inside the given Rect.
    /// ```rust
    /// # use lart::*;
    /// let mut doc = Sketch::new("in_rect");
    /// let bbox = doc.page_bbox();
    /// let v = V::in_rect(&mut doc, &bbox);
    /// assert!(bbox.contains(v));
    /// ```
    pub fn in_rect(rng: &mut impl Rng, rect: &Rect) -> V {
        Self::in_range(rng, rect.left()..=rect.right(), rect.top()..=rect.bottom())
    }

    /// Create a new V built from the given angle in radians and radius.
    /// ```rust
    /// # use lart::*;
    /// assert_eq!(V::polar(0.0, 1.0), v(1,0));
    /// assert!(V::polar(TAU/4.0, 4.0).almost_equal(v(0,4)));
    /// ```
    pub fn polar(a: f64, r: f64) -> Self {
        let (s, c) = a.sin_cos();
        Self::new(c * r, s * r)
    }

    /// Return the distance between this point and another one.
    /// ```rust
    /// # use lart::*;
    /// assert_eq!(v(15,0).dist(v(100,0)), 85.0);
    /// assert_eq!(v(-1,2).dist(v(2,-2)), 5.0);
    /// ```
    pub fn dist(self, rhs: Self) -> f64 {
        (rhs - self).norm()
    }

    /// Return the squared distance between this point and another one.
    ///
    /// This is usually to be preferred than dist because it's way faster.
    ///
    /// ```rust
    /// # use lart::*;
    /// assert_eq!(v(15,0).dist2(v(25,0)), 100.0);
    /// assert_eq!(v(-1,2).dist2(v(2,-2)), 25.0);
    /// ```
    pub fn dist2(self, rhs: Self) -> f64 {
        (rhs - self).norm2()
    }

    /// Return the norm (aka the length) of this vector.
    /// ```rust
    /// # use lart::*;
    /// assert_eq!(v(0,10).norm(), 10.0);
    /// assert_eq!(v(3,4).norm(), 5.0);
    /// ```
    pub fn norm(self) -> f64 {
        f64::hypot(self.x, self.y)
    }
    /// Return the squared norm (aka the length) of this vector.
    /// ```rust
    /// # use lart::*;
    /// assert_eq!(v(0,10).norm2(), 100.0);
    /// assert_eq!(v(3,4).norm2(), 25.0);
    /// ```
    pub fn norm2(self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }

    /// Normalize the V so that its norm is 1.
    /// ```rust
    /// # use lart::*;
    /// assert_eq!(v(-4,0).normalized(), v(-1,0));
    /// assert_eq!(v(3,-2).normalized().norm(), 1.0);
    /// ```
    pub fn normalized(self) -> Self {
        self / self.norm()
    }

    /// Return the [dot product][0] between two V.
    ///
    /// The dot product returns:
    /// - a positive value when the two vectors face the same direction
    /// - a negative value when the two vectors face opposite directions
    ///
    /// [0]: https://en.wikipedia.org/wiki/Dot_product
    ///
    /// ```rust
    /// # use lart::*;
    /// assert!(v(3,0).dot(v(1,1)) > 0.0);
    /// assert!(v(3,0).dot(v(-1,1)) < 0.0);
    /// ```
    pub fn dot(self, rhs: V) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }

    /// Return the angle in radians of the vector.
    /// ```rust
    /// # use lart::*;
    /// assert_eq!(V::polar(TAU/3.0, 2.0).angle(), TAU/3.0);
    /// ```
    pub fn angle(self) -> f64 {
        f64::atan2(self.y, self.x)
    }

    /// Return a value representing where the current point is wrt to the
    /// directed line going from point a to b.
    ///
    /// The returned value is:
    /// - positive value if the current point is on the left of the line
    /// - negative value if it's on the right of the line
    /// - 0 if the points are collinear
    ///
    /// ```rust
    /// # use lart::*;
    /// assert!(v(-1,1).orient(v(4,4), v(10,10)) > 0.0);
    /// assert!(v(1,-1).orient(v(4,4), v(10,10)) < 0.0);
    /// assert!(v(6,6).orient(v(4,4), v(10,10)) == 0.0);
    /// ```
    pub fn orient(self, a: Self, b: Self) -> f64 {
        (b.x - a.x) * (self.y - a.y) - (b.y - a.y) * (self.x - a.x)
    }

    /// Create a V with the maximum coordinates between two V.
    /// ```rust
    /// # use lart::*;
    /// assert_eq!(v(-5,120).max(v(-2,99)), v(-2,120));
    /// ```
    pub fn max(self, o: Self) -> Self {
        Self::new(self.x.max(o.x), self.y.max(o.y))
    }

    /// Create a V with the minimum coordinates between two V.
    /// ```rust
    /// # use lart::*;
    /// assert_eq!(v(-5,120).min(v(-2,99)), v(-5,99));
    /// ```
    pub fn min(self, o: Self) -> Self {
        Self::new(self.x.min(o.x), self.y.min(o.y))
    }

    /// Create a V with the absolute value of the coordinates.
    /// ```rust
    /// # use lart::*;
    /// assert_eq!(v(-5,120).abs(), v(5,120));
    /// ```
    pub fn abs(self) -> Self {
        Self::new(self.x.abs(), self.y.abs())
    }

    /// Check that two V are the same considering a very small epsilon to
    /// account for double inaccuracies.
    ///
    /// Pratically speaking for plotter purposes, this method works perfectly
    /// fine.
    ///
    /// ```rust
    /// # use lart::*;
    /// assert!(V::polar(TAU/4.0, 4.0).almost_equal(v(0,4)));
    /// ```
    pub fn almost_equal(self, rhs: Self) -> bool {
        // NOTE: we still first check for exact equality because it's faster and
        // cheaper than computing the actual distance.
        self == rhs || self.dist2(rhs) < PRECISION_2
    }
}

impl Bbox for V {
    fn bbox(&self) -> Option<Rect> {
        Some(Rect::new(*self))
    }
}

macro_rules! impl_num_op {
    ($tr: ident, $name: ident) => {
        impl std::ops::$tr<f64> for V {
            type Output = Self;
            fn $name(self, rhs: f64) -> Self::Output {
                V::new(self.x.$name(rhs), self.y.$name(rhs))
            }
        }

        impl std::ops::$tr<V> for f64 {
            type Output = V;
            fn $name(self, rhs: V) -> Self::Output {
                V::new(self.$name(rhs.x), self.$name(rhs.y))
            }
        }

        impl std::ops::$tr<V> for V {
            type Output = Self;
            fn $name(self, rhs: V) -> Self::Output {
                V::new(self.x.$name(rhs.x), self.y.$name(rhs.y))
            }
        }
    };

    (Assign, $tr: ident, $name: ident) => {
        impl std::ops::$tr<f64> for V {
            fn $name(&mut self, rhs: f64) {
                self.x.$name(rhs);
                self.y.$name(rhs);
            }
        }

        impl std::ops::$tr<V> for V {
            fn $name(&mut self, rhs: V) {
                self.x.$name(rhs.x);
                self.y.$name(rhs.y);
            }
        }
    };
}

impl_num_op!(Add, add);
impl_num_op!(Sub, sub);
impl_num_op!(Mul, mul);
impl_num_op!(Div, div);
impl_num_op!(Rem, rem);

impl_num_op!(Assign, AddAssign, add_assign);
impl_num_op!(Assign, SubAssign, sub_assign);
impl_num_op!(Assign, MulAssign, mul_assign);
impl_num_op!(Assign, DivAssign, div_assign);
impl_num_op!(Assign, RemAssign, rem_assign);

impl std::ops::Neg for V {
    type Output = V;

    fn neg(self) -> Self::Output {
        V::new(-self.x, -self.y)
    }
}

impl<W: Into<f64>> From<(W, W)> for V {
    fn from((x, y): (W, W)) -> Self {
        V::new(x.into(), y.into())
    }
}

impl std::fmt::Debug for V {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("v").field(&self.x).field(&self.y).finish()
    }
}

impl std::iter::Sum for V {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(V::new(0.0, 0.0), |a, b| a + b)
    }
}
