use std::ops::{Index, IndexMut, RangeBounds};

use crate::{bbox_union, path, polar_angles, sample_seg, v, Bbox, Path, Rect, V};

impl Path {
    pub const fn new() -> Self {
        Self { points: vec![] }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            points: Vec::with_capacity(cap),
        }
    }

    /// Return the `n` angles diving the circle in `n` arcs.
    pub fn circle(c: V, r: f64, steps: u16) -> Self {
        polar_angles(steps)
            .map(|a| c + V::polar(a, r))
            .collect::<Self>()
            .closed()
    }

    pub fn push(&mut self, a: impl Into<V>) {
        self.points.push(a.into())
    }

    pub fn pop(&mut self) -> Option<V> {
        self.points.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn clear(&mut self) {
        self.points.clear();
    }

    pub fn close(&mut self) {
        if let Some(p) = self.first() {
            if p != self.last().unwrap() {
                self.push(p);
            }
        }
    }

    pub fn closed(mut self) -> Self {
        self.close();
        self
    }

    pub fn is_closed(&self) -> bool {
        self.first() == self.last()
    }

    pub fn first(&self) -> Option<V> {
        self.points.first().cloned()
    }

    pub fn last(&self) -> Option<V> {
        self.points.last().cloned()
    }

    pub fn iter(&self) -> impl Iterator<Item = V> + '_ {
        self.points.iter().copied()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut V> + '_ {
        self.points.iter_mut()
    }

    pub fn points(&self) -> &[V] {
        &self.points
    }

    pub fn segment(&self, i: usize) -> Self {
        Self::from([self.points[i], self.points[(i + 1) % self.points.len()]])
    }

    pub fn segments(&self) -> impl Iterator<Item = (V, V)> + '_ {
        self.iter().zip(self.iter().skip(1))
    }

    pub fn closed_segments(&self) -> impl Iterator<Item = (V, V)> + '_ {
        self.iter().zip(self.points.iter().copied().cycle().skip(1))
    }

    pub fn norm(&self) -> f64 {
        self.segments().map(|(a, b)| a.dist(b)).sum()
    }

    pub fn norm2(&self) -> f64 {
        self.segments().map(|(a, b)| a.dist2(b)).sum()
    }

    pub fn centroid(&self) -> V {
        let mut c = v(0, 0);
        for p in self.iter() {
            c += p
        }
        c / (self.points.len() as f64)
    }

    pub fn slice(&self, r: impl RangeBounds<usize>) -> Path {
        let mut p = Path::new();
        p.points
            .extend_from_slice(&self.points[(r.start_bound().cloned(), r.end_bound().cloned())]);
        p
    }

    pub fn reverse(&mut self) {
        self.points.reverse();
    }

    pub fn dedup(&mut self) {
        self.points.dedup();
    }

    pub fn sampled(&self, max_dist: f64) -> Self {
        Self::from_iter(
            self.segments()
                .flat_map(|(a, b)| sample_seg(a, b, max_dist, false).map(|s| s.point)),
        )
    }

    pub fn area(&self) -> f64 {
        self.sarea().abs()
    }

    fn sarea(&self) -> f64 {
        if self.points.len() < 3 {
            return 0.0;
        }

        let mut a = 0.0;
        let mut p1 = *self.points.last().unwrap();
        for p2 in self.iter() {
            a += p1.x * p2.y - p2.x * p1.y;
            p1 = p2;
        }
        a * 0.5
    }
}

impl Bbox for Path {
    fn bbox(&self) -> Option<Rect> {
        bbox_union(&self.points)
    }
}

impl Index<usize> for Path {
    type Output = V;

    fn index(&self, index: usize) -> &Self::Output {
        self.points.index(index)
    }
}

impl IndexMut<usize> for Path {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.points.index_mut(index)
    }
}

impl<W: Into<V>, I: IntoIterator<Item = W>> From<I> for Path {
    fn from(it: I) -> Self {
        Self {
            points: it.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<Rect> for Path {
    fn from(r: Rect) -> Self {
        path!(
            r.min(),
            V::new(r.right(), r.top()),
            r.max(),
            V::new(r.left(), r.bottom()),
        )
    }
}

impl FromIterator<V> for Path {
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        Self {
            points: iter.into_iter().collect(),
        }
    }
}

impl Extend<V> for Path {
    fn extend<T: IntoIterator<Item = V>>(&mut self, iter: T) {
        self.points.extend(iter);
    }
}

impl std::fmt::Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("path!")?;
        f.debug_list().entries(&self.points).finish()
    }
}
