use rand::{distributions::WeightedIndex, prelude::*};

use crate::{F64Key, Grid};

pub type Border = u8;

#[derive(Debug, Clone)]
pub struct Tile {
    pub left: Border,
    pub top: Border,
    pub right: Border,
    pub bottom: Border,
    pub weight: f64,
}

impl Tile {
    pub const fn new(left: Border, top: Border, right: Border, bottom: Border) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
            weight: 1.0,
        }
    }

    pub const fn with_weight(mut self, w: f64) -> Self {
        self.weight = w;
        self
    }
}

pub fn solve(
    rng: &mut impl Rng,
    tiles: &[Tile],
    width: usize,
    height: usize,
) -> Option<Grid<usize>> {
    if width == 0 || height == 0 || tiles.is_empty() {
        return None;
    }

    let mut state = State::new(tiles, rng, width, height);
    let pt = state.rand_pt();
    state.entropy[pt] = 0.0;
    state.solve()
}

struct State<'t, R> {
    tiles: &'t [Tile],
    wave: Grid<usize>,
    entropy: Grid<f64>,
    rng: &'t mut R,
}

impl<'t, R: Rng> State<'t, R> {
    fn new(tiles: &'t [Tile], rng: &'t mut R, width: usize, height: usize) -> Self {
        Self {
            tiles,
            rng,
            wave: Grid::new(tiles.len(), width, height),
            entropy: Grid::new(f64::MAX, width, height),
        }
    }

    fn rand_pt(&mut self) -> (usize, usize) {
        (
            self.rng.gen_range(0..self.wave.width()),
            self.rng.gen_range(0..self.wave.height()),
        )
    }

    fn solve(&mut self) -> Option<Grid<usize>> {
        let (x, y, e) = self
            .entropy
            .enum_cells()
            .min_by_key(|(_, _, e)| F64Key(**e))
            .unwrap();

        if !e.is_finite() {
            let mut tmp = Grid::new(0, 0, 0);
            std::mem::swap(&mut tmp, &mut self.wave);
            return Some(tmp);
        }

        let mut weights: Vec<f64> = Vec::with_capacity(self.tiles.len());
        let mut tile_ixs = Vec::with_capacity(self.tiles.len());
        for (i, cand) in self.tiles.iter().enumerate() {
            if self.border_match(self.wave.left(x, y), |t| t.right != cand.left) {
                continue;
            }

            if self.border_match(self.wave.right(x, y), |t| t.left != cand.right) {
                continue;
            }

            if self.border_match(self.wave.up(x, y), |t| t.bottom != cand.top) {
                continue;
            }

            if self.border_match(self.wave.down(x, y), |t| t.top != cand.bottom) {
                continue;
            }

            weights.push(cand.weight);
            tile_ixs.push(i);
        }

        self.entropy[(x, y)] = f64::INFINITY;
        while !tile_ixs.is_empty() {
            let wi = WeightedIndex::new(&weights).unwrap();
            let candi = wi.sample(self.rng);

            self.wave[(x, y)] = tile_ixs[candi];

            for (nx, ny) in self.entropy.neighbors4(x, y) {
                self.update_entropy(nx, ny);
            }

            if let Some(sol) = self.solve() {
                return Some(sol);
            }

            tile_ixs.swap_remove(candi);
            weights.swap_remove(candi);
        }

        // undo the move, this allows to avoid copies and allocations when
        // exploring the space and it's sensibly faster
        self.wave[(x, y)] = self.tiles.len();
        self.update_entropy(x, y);
        for (nx, ny) in self.entropy.neighbors4(x, y) {
            self.update_entropy(nx, ny);
        }

        None
    }

    fn border_match(
        &self,
        c: Option<(usize, usize)>,
        border_match: impl Fn(&Tile) -> bool,
    ) -> bool {
        c.and_then(|ix| self.tiles.get(self.wave[ix]))
            .map_or(false, border_match)
    }

    fn update_entropy(&mut self, x: usize, y: usize) {
        if self.wave[(x, y)] < self.tiles.len() {
            self.entropy[(x, y)] = f64::INFINITY;
            return;
        }

        self.entropy[(x, y)] = std::f64::MAX
            - self
                .entropy
                .neighbors4(x, y)
                .filter_map(|ix| Some(self.tiles.get(self.wave[ix])?.weight))
                .sum::<f64>();
    }
}
