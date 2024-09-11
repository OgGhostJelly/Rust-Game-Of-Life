use std::{
    collections::HashSet,
    fmt::{self, Write},
};

use rand::{rngs::ThreadRng, Rng};

#[derive(Clone)]
pub struct Board<const WIDTH: usize, const HEIGHT: usize> {
    cells: Box<[[Cell; WIDTH]; HEIGHT]>,
    alive_cells: Vec<(usize, usize)>,
}

impl<const WIDTH: usize, const HEIGHT: usize> Board<WIDTH, HEIGHT> {
    pub fn new() -> Self {
        Self {
            cells: Box::new([[Cell::empty(); WIDTH]; HEIGHT]),
            alive_cells: Vec::new(),
        }
    }

    pub fn from_cells(cells: &[&[usize]]) -> Self {
        let mut board = Self::new();

        for (y, row) in cells.into_iter().enumerate() {
            for (x, cell) in row.into_iter().enumerate() {
                if *cell != 0 {
                    board.make_alive(x, y);
                }
            }
        }

        board
    }

    pub fn with_alive_capacity(capacity: usize) -> Self {
        Self {
            cells: Box::new([[Cell::empty(); WIDTH]; HEIGHT]),
            alive_cells: Vec::with_capacity(capacity),
        }
    }

    pub fn full() -> Self {
        Self {
            cells: Box::new([[Cell::new(true, 8); WIDTH]; HEIGHT]),
            alive_cells: (0..HEIGHT)
                .map(|y| (0..WIDTH).map(move |x| (x, y)))
                .flatten()
                .collect(),
        }
    }

    pub fn rand(rng: &mut ThreadRng, p: f64) -> Self {
        let mut board = Self::with_alive_capacity((HEIGHT * WIDTH) / 2);

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if rng.gen_bool(p) {
                    board.make_alive(x, y);
                }
            }
        }

        board
    }

    pub fn cells(&self) -> &[[Cell; WIDTH]; HEIGHT] {
        &self.cells
    }

    pub fn alive_cells(&self) -> &Vec<(usize, usize)> {
        &self.alive_cells
    }

    pub fn tick(&self) -> Self {
        let mut board = Self::with_alive_capacity(self.alive_cells.len());

        for (x, y) in self.alive_cells.iter().cloned() {
            if self.cells[y][x].next_alive_state() {
                board.make_alive(x, y);
            }

            for (x, y) in get_adjacents(x, y) {
                if x >= WIDTH || y >= HEIGHT {
                    continue;
                }

                if self.cells[y][x].is_dead()
                    && board.cells[y][x].is_dead()
                    && self.cells[y][x].next_dead_state()
                {
                    board.make_alive(x, y);
                }
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
            if other_x >= WIDTH || other_y >= HEIGHT {
                continue;
            }

            self.cells[other_y][other_x].0 += 1
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> PartialEq for Board<WIDTH, HEIGHT> {
    fn eq(&self, other: &Self) -> bool {
        if self.alive_cells.len() != other.alive_cells.len() {
            return false;
        }

        let map: HashSet<&(usize, usize)> = HashSet::from_iter(self.alive_cells.iter());

        for key in &other.alive_cells {
            if !map.contains(key) {
                return false;
            }
        }

        if self.cells.len() != other.cells.len() {
            return false;
        }

        self.cells
            .iter()
            .zip(other.cells.iter())
            .map(|(a, b)| {
                if a.len() != b.len() {
                    return false;
                }

                a.iter().zip(b.iter()).map(|(a, b)| a == b).all(|v| v)
            })
            .all(|v| v)
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> fmt::Debug for Board<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.cells.iter() {
            f.write_char('\n')?;

            for cell in row {
                write!(
                    f,
                    "({}, {}), ",
                    if cell.is_alive() { "t" } else { "f" },
                    cell.neighbour_count()
                )?;
            }
        }

        Ok(())
    }
}

fn get_adjacents(to_x: usize, to_y: usize) -> impl Iterator<Item = (usize, usize)> {
    (-1isize..=1)
        .map(move |y| {
            (-1isize..=1).filter_map(move |x| {
                if x == 0 && y == 0 {
                    None
                } else {
                    Some((to_x.checked_add_signed(x)?, to_y.checked_add_signed(y)?))
                }
            })
        })
        .flatten()
}

/// A single game of life cell.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cell(i8);

impl Cell {
    const LIVE: i8 = 0b10000000u8 as i8;
    const NEIGHBOUR: i8 = 0b01111111;

    #[inline]
    pub fn new(is_alive: bool, neighbour_count: i8) -> Self {
        debug_assert!(
            neighbour_count <= 8,
            "integer overflow: neighbour_count (is {}) <= 8",
            neighbour_count
        );
        debug_assert_eq!(
            neighbour_count & !Self::NEIGHBOUR,
            0,
            "integer underflow: neighbour_count (is {}) cannot be negative",
            neighbour_count
        );
        Self(neighbour_count | if is_alive { Self::LIVE } else { 0 })
    }

    pub fn empty() -> Self {
        Self::new(false, 0)
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
        if self.is_alive() {
            self.next_alive_state()
        } else {
            self.next_dead_state()
        }
    }

    #[inline]
    pub fn next_alive_state(self) -> bool {
        let neighbour_count = self.neighbour_count();
        neighbour_count >= 2 && neighbour_count <= 3
    }

    #[inline]
    pub fn next_dead_state(self) -> bool {
        let neighbour_count = self.neighbour_count();
        neighbour_count == 3
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
        assert_eq!(
            get_adjacents(0, 0).collect::<Vec<_>>(),
            &[(1, 0), (0, 1), (1, 1)]
        );

        assert_eq!(
            get_adjacents(1, 0).collect::<Vec<_>>(),
            &[(0, 0), (2, 0), (0, 1), (1, 1), (2, 1)]
        );

        assert_eq!(
            get_adjacents(1, 1).collect::<Vec<_>>(),
            &[
                (0, 0),
                (1, 0),
                (2, 0),
                (0, 1),
                (2, 1),
                (0, 2),
                (1, 2),
                (2, 2)
            ]
        );
    }

    #[test]
    fn test_from_cells() {
        let mut expected = Board::<3, 3>::new();
        expected.make_alive(1, 0);
        expected.make_alive(1, 1);
        expected.make_alive(1, 2);

        let board = Board::from_cells(&[&[0, 1, 0], &[0, 1, 0], &[0, 1, 0]]);

        assert_eq!(expected, board);

        let mut expected = Board::<3, 3>::new();
        expected.make_alive(0, 1);
        expected.make_alive(1, 1);
        expected.make_alive(2, 1);

        let board = Board::from_cells(&[&[0, 0, 0], &[1, 1, 1], &[0, 0, 0]]);

        assert_eq!(expected, board);
    }

    #[test]
    fn test_board_tick() {
        let board: Board<3, 3> = Board::from_cells(&[&[0, 1, 0], &[0, 1, 0], &[0, 1, 0]]);

        let expected = Board::from_cells(&[&[0, 0, 0], &[1, 1, 1], &[0, 0, 0]]);

        assert_eq!(board.tick(), expected)
    }

    #[test]
    fn test_partial_eq() {
        assert_eq!(
            Board::<3, 3>::from_cells(&[&[1, 1, 0], &[0, 1, 1], &[0, 1, 0],]),
            Board::from_cells(&[&[1, 1, 0], &[0, 1, 1], &[0, 1, 0],]),
        );

        assert_ne!(
            Board::<3, 3>::from_cells(&[&[0, 1, 1], &[0, 1, 0], &[1, 1, 1]]),
            Board::from_cells(&[&[0, 1, 1], &[0, 1, 1], &[1, 1, 1]]),
        );

        let board = Board::<3, 3>::from_cells(&[&[0, 1, 1], &[0, 1, 0], &[1, 1, 1]]);

        let expected = {
            let mut board = board.clone();
            board.alive_cells.pop();
            board
        };

        assert_ne!(board, expected);

        let board = Board::<3, 3>::from_cells(&[&[0, 1, 1], &[0, 1, 0], &[1, 1, 1]]);

        let expected = {
            let mut board = board.clone();
            board.alive_cells.pop();
            board.alive_cells.push((0, 0));
            board
        };

        assert_ne!(board, expected);
    }

    fn test_simulation<const W: usize, const H: usize>(steps: &[&[&[usize]]]) {
        let (initial, steps) = (&steps[0], &steps[1..]);
        let mut board = Board::<W, H>::from_cells(initial);

        for next in steps.into_iter() {
            board = board.tick();

            let expected: Board<W, H> = Board::from_cells(next);

            for row in board.cells.iter() {
                print!("[");

                for cell in row {
                    print!(
                        "Cell({}, {})",
                        if cell.is_alive() { "t" } else { "f" },
                        cell.neighbour_count(),
                    );

                    print!(", ");
                }

                println!("]")
            }

            println!("----");

            for row in expected.cells.iter() {
                print!("[");

                for cell in row {
                    print!(
                        "Cell({}, {})",
                        if cell.is_alive() { "t" } else { "f" },
                        cell.neighbour_count(),
                    );

                    print!(", ");
                }

                println!("]")
            }

            println!("cells: {:?}", board.cells == expected.cells);
            println!(
                "alive_cells: {:?}",
                board.alive_cells == expected.alive_cells
            );

            assert_eq!(board, expected)
        }
    }

    #[test]
    fn test_blinker() {
        test_simulation::<3, 3>(&[
            &[&[0, 1, 0], &[0, 1, 0], &[0, 1, 0]],
            &[&[0, 0, 0], &[1, 1, 1], &[0, 0, 0]],
            &[&[0, 1, 0], &[0, 1, 0], &[0, 1, 0]],
            &[&[0, 0, 0], &[1, 1, 1], &[0, 0, 0]],
        ]);
    }

    #[test]
    fn test_glider() {
        test_simulation::<4, 4>(&[
            &[&[1, 0, 0], &[0, 1, 1], &[1, 1, 0]],
            &[&[0, 1, 0], &[0, 0, 1], &[1, 1, 1]],
            &[&[0, 0, 0], &[1, 0, 1], &[0, 1, 1], &[0, 1, 0]],
            &[&[0, 0, 0], &[0, 0, 1], &[1, 0, 1], &[0, 1, 1]],
            &[&[0, 0, 0, 0], &[0, 1, 0, 0], &[0, 0, 1, 1], &[0, 1, 1, 0]],
        ])
    }
}
