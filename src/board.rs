use rand::{rngs::ThreadRng, Rng};

pub const W: usize = 250;
pub const H: usize = 250;

#[derive(PartialEq, Eq)]
#[derive(Debug)]
pub struct Board {
    cells: [[Cell; W]; H],
    alive_cells: Vec<(usize, usize)>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: [[Cell::new(false, 0); W]; H],
            alive_cells: Vec::new(),
        }
    }

    pub fn full() -> Self {
        Self {
            cells: [[Cell::new(true, 8); W]; H],
            alive_cells: (0..H).map(|y| (0..W).map(move |x| (x, y))).flatten().collect(),
        }
    }

    pub fn rand(rng: &mut ThreadRng, p: f64) -> Self {
        let mut board = Board::new();

        for y in 0..H {
            for x in 0..W {
                if rng.gen_bool(p) {
                    board.make_alive(x, y);
                }
            }
        }

        board
    }

    pub fn cells(&self) -> &[[Cell; W]; H] {
        &self.cells
    }
    
    pub fn tick(self) -> Self {
        let mut board = Self::new();

        for (x, y) in self.alive_cells.into_iter() {
            if self.cells[y][x].next_state() {
                board.make_alive(x, y);
            }
        }

        board
    }

    fn make_alive(&mut self, x: usize, y: usize) {
        self.alive_cells.push((x, y));
        self.propagate_neighbours(x, y);
        self.cells[y][x].make_alive();
    }

    fn propagate_neighbours(&mut self, x: usize, y: usize) {
        for (other_x, other_y) in get_adjacents(x, y) {
            if other_x >= W || other_y >= H {
                continue;
            }

            self.cells[other_y][other_x].0 += 1
        }
    }
}

fn get_adjacents(to_x: usize, to_y: usize) -> impl Iterator<Item = (usize, usize)> {
    (-1isize..=1).map(move |y|
        (-1isize..=1).filter_map(move |x| {
            if x == 0 && y == 0 {
                None
            } else {
                Some((to_x.checked_add_signed(x)?, to_y.checked_add_signed(y)?))
            }
        })).flatten()
}

/// A single game of life cell.
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(Debug)]
pub struct Cell(i8);

impl Cell {
    const LIVE: i8 = 0b10000000u8 as i8;
    const NEIGHBOUR: i8 = 0b01111111;

    #[inline]
    pub fn new(is_alive: bool, neighbour_count: i8) -> Self {
        debug_assert!(neighbour_count <= 8, "integer overflow: neighbour_count (is {}) <= 8", neighbour_count);
        debug_assert_eq!(neighbour_count & !Self::NEIGHBOUR, 0, "integer underflow: neighbour_count (is {}) cannot be negative", neighbour_count);
        Self(neighbour_count | if is_alive { Self::LIVE } else { 0 })
    }

    #[inline]
    pub fn is_alive(self) -> bool {
        self.0 & Self::LIVE != 0
    }

    #[inline]
    pub fn is_dead(self) -> bool {
        !self.is_alive()
    }

    #[inline]
    pub fn make_alive(&mut self) {
        self.0 |= Self::LIVE;
    }

    #[inline]
    pub fn make_dead(&mut self) {
        self.0 &= !Self::LIVE;
    }

    #[inline]
    pub fn neighbour_count(self) -> i8 {
        self.0 & Self::NEIGHBOUR
    }

    #[inline]
    pub fn next_state(self) -> bool {
        let neighbour_count = self.neighbour_count();
        if self.is_alive() {
            neighbour_count >= 2 && neighbour_count <= 3
        } else {
            neighbour_count == 3
        }
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn test_neighbours_for_dead() {
        assert!(!Cell::new(false, 8).next_state());
        assert!(!Cell::new(false, 7).next_state());
        assert!(!Cell::new(false, 6).next_state());
        assert!(!Cell::new(false, 5).next_state());
        assert!(!Cell::new(false, 4).next_state());
        
        assert!(Cell::new(false, 3).next_state());

        assert!(!Cell::new(false, 2).next_state());
        assert!(!Cell::new(false, 1).next_state());
        assert!(!Cell::new(false, 0).next_state());
    }

    #[test]
    fn test_neighbours_for_alive() {
        assert!(!Cell::new(true, 8).next_state());
        assert!(!Cell::new(true, 7).next_state());
        assert!(!Cell::new(true, 6).next_state());
        assert!(!Cell::new(true, 5).next_state());
        assert!(!Cell::new(true, 4).next_state());
        
        assert!(Cell::new(true, 3).next_state());
        assert!(Cell::new(true, 2).next_state());

        assert!(!Cell::new(true, 1).next_state());
        assert!(!Cell::new(true, 0).next_state());
    }

    #[test]
    fn test_cell_constructor() {
        for i in 0..=8 {
            let cell = Cell::new(true, i);
            assert!(cell.is_alive());
            assert_eq!(cell.neighbour_count(), i);
        }

        for i in 0..=8 {
            let cell = Cell::new(false, i);
            assert!(!cell.is_alive());
            assert_eq!(cell.neighbour_count(), i);
        }
    }
    
    #[test]
    #[should_panic]
    fn test_overflow_cell_constructor() {
        Cell::new(false, 9);
    }
    
    #[test]
    #[should_panic]
    fn test_underflow_cell_constructor() {
        Cell::new(false, -1);
    }

    #[test]
    fn test_adjacents() {
        assert_eq!(get_adjacents(0, 0).collect::<Vec<_>>(), &[        (1, 0),
                                                              (0, 1), (1, 1)]);
        
        assert_eq!(get_adjacents(1, 0).collect::<Vec<_>>(), &[(0, 0),         (2, 0),
                                                              (0, 1), (1, 1), (2, 1)]);
        
        assert_eq!(get_adjacents(1, 1).collect::<Vec<_>>(), &[(0, 0), (1, 0), (2, 0),
                                                              (0, 1),         (2, 1),
                                                              (0, 2), (1, 2), (2, 2)]);
    }

    #[test]
    fn test_blinker() {
        // . # .
        // . # .
        // . # .
        let mut board = Board::new();
        board.make_alive(1, 0);
        board.make_alive(1, 1);
        board.make_alive(1, 2);

        // . . .
        // # # #
        // . . .
        let expected = Board::new();
        board.make_alive(0, 1);
        board.make_alive(1, 1);
        board.make_alive(2, 1);

        let board = board.tick();
        assert_eq!(board, expected);
    }
}