#[derive(Debug)]
pub struct Grid<const Y: usize, const X: usize, T>([[T; X]; Y]);

impl<const Y: usize, const X: usize, T> Grid<Y, X, T> {
    pub fn new(value: [[T; X]; Y]) -> Self {
        Self(value)
    }

    pub fn iter(&self) -> std::slice::Iter<[T; X]> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<[T; X]> {
        self.into_iter()
    }

    pub fn contains(&self, y: usize, x: usize) -> bool {
        match self.get(y, x) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn get_8_neighbours(&self, y: usize, x: usize) -> [((isize, isize), Option<&T>); 8] {
        let mut neighbours = [((0isize, 0isize), None); 8];

        const NEIGHBOURS: [(isize, isize); 8] = [
            (-1, -1), (-1,  0), (-1,  1),
            ( 0, -1),           ( 0,  1),
            ( 1, -1), ( 1,  0), ( 1,  1),
        ];

        for (i, pos) in NEIGHBOURS.into_iter().enumerate() {
            neighbours[i] = (pos, match y.checked_add_signed(pos.0) {
                Some(y) => match x.checked_add_signed(pos.1) {
                    Some(x) => self.get(y, x),
                    None => None,
                },
                None => None,
            });
        }

        neighbours
    }

    pub fn get(&self, y: usize, x: usize) -> Option<&T> {
        match self.0.get(y) {
            Some(row) => match row.get(x) {
                Some(tile) => Some(tile),
                None => None,
            },
            None => None,
        }
    }

    pub fn grid_mut(&mut self) -> &mut [[T; X]; Y] {
        &mut self.0
    }

    pub fn grid(&self) -> &[[T; X]; Y] {
        &self.0
    }
}

impl<const Y: usize, const X: usize, T> IntoIterator for Grid<Y, X, T> {
    type Item = [T; X];

    type IntoIter = std::array::IntoIter<[T; X], Y>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, const Y: usize, const X: usize, T> std::ops::Index<usize> for Grid<Y, X, T> {
    type Output = [T; X];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<'a, const Y: usize, const X: usize, T> std::ops::IndexMut<usize> for Grid<Y, X, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<'a, const Y: usize, const X: usize, T> IntoIterator for &'a Grid<Y, X, T> {
    type Item = &'a [T; X];

    type IntoIter = std::slice::Iter<'a, [T; X]>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, const Y: usize, const X: usize, T> IntoIterator for &'a mut Grid<Y, X, T> {
    type Item = &'a mut [T; X];

    type IntoIter = std::slice::IterMut<'a, [T; X]>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<T, const Y: usize, const X: usize> From<[[T; X]; Y]> for Grid<Y, X, T> {
    fn from(value: [[T; X]; Y]) -> Self {
        Self::new(value)
    }
}
