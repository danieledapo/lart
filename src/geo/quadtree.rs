use crate::{Rect, V};

#[derive(Debug, Clone)]
pub struct QuadTree<T: QuadTreeElem> {
    root: Box<Node<T>>,
}

pub trait QuadTreeElem: std::fmt::Debug {
    fn reference(&self) -> V;
}

#[derive(Debug, Clone)]

enum Node<T: QuadTreeElem> {
    Leaf(Vec<T>, Rect),
    Branch([Box<Node<T>>; 4]),
}

impl<T: QuadTreeElem> QuadTree<T> {
    pub fn new(bbox: Rect, elements: Vec<T>) -> Self {
        Self {
            root: Box::new(Node::new(bbox, elements)),
        }
    }

    pub fn in_range(&self, p: V, eps: f64) -> Vec<&T> {
        let eps2 = eps.powi(2);

        let mut res = vec![];
        let mut stack = vec![&self.root];
        while let Some(n) = stack.pop() {
            match &**n {
                Node::Branch(sub) => stack.extend(sub.iter()),
                Node::Leaf(elements, bbox) => {
                    if !bbox.contains(p) && bbox.dist2(p) > eps {
                        continue;
                    }

                    res.extend(elements.iter().filter(|e| e.reference().dist2(p) <= eps2));
                }
            }
        }

        res
    }
}

impl<T: QuadTreeElem> Node<T> {
    pub const LEAF_CAPACITY: usize = 64;

    pub fn new(bbox: Rect, elements: Vec<T>) -> Self {
        if elements.len() <= Self::LEAF_CAPACITY {
            return Self::Leaf(elements, bbox);
        }

        let sub: [Rect; 4] = TryFrom::try_from(bbox.subdivide(2, 2).collect::<Vec<_>>()).unwrap();
        let mut sub = sub.map(|b| (b, Vec::with_capacity(Self::LEAF_CAPACITY)));

        let c = bbox.center();
        for e in elements {
            let p = e.reference();
            let mut i = 0;
            i += if p.x < c.x { 0 } else { 1 };
            i += if p.y < c.y { 0 } else { 2 };
            sub[i].1.push(e);
        }

        Self::Branch(sub.map(|(b, els)| Box::new(Self::new(b, els))))
    }
}
