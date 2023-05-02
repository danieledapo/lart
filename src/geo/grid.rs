use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Clone> Grid<T> {
    pub fn new(def: T, width: usize, height: usize) -> Self {
        Self {
            data: vec![def; width * height],
            width,
            height,
        }
    }
}

impl<T> Grid<T> {
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.data.get(y * self.width + x)
    }
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.data.get_mut(y * self.width + x)
    }

    pub fn set(&mut self, x: usize, y: usize, t: T) {
        self[(x, y)] = t;
    }

    pub fn cells(&self) -> impl Iterator<Item = &T> + '_ {
        self.data.iter()
    }

    pub fn cells_mut(&mut self) -> impl Iterator<Item = &mut T> + '_ {
        self.data.iter_mut()
    }

    pub fn enum_cells(&self) -> impl Iterator<Item = (usize, usize, &T)> + '_ {
        self.data.iter().enumerate().map(|(i, c)| {
            let x = i % self.width;
            let y = i / self.width;
            (x, y, c)
        })
    }

    pub fn indices(&self) -> impl Iterator<Item = (usize, usize)> {
        let w = self.width;
        let h = self.height;
        (0..h).flat_map(move |y| (0..w).map(move |x| (x, y)))
    }

    pub fn neighbors4(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        self.left(x, y)
            .into_iter()
            .chain(self.up(x, y))
            .chain(self.right(x, y))
            .chain(self.down(x, y))
    }

    pub fn up(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        (y > 0 && self.height > 0 && x < self.width).then_some((x, y - 1))
    }

    pub fn down(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        (y + 1 < self.height && x < self.width).then_some((x, y + 1))
    }

    pub fn left(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        (x > 0 && self.width > 0 && y < self.height).then_some((x - 1, y))
    }

    pub fn right(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        (x + 1 < self.width && y < self.height).then_some((x + 1, y))
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.data[y * self.width + x]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.data[y * self.width + x]
    }
}
