use crate::automata::Cell;
use bevy::prelude::*;
use rand::RngExt;

#[derive(Resource, Clone)]
pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Grid {
    pub fn new_random(width: usize, height: usize) -> Self {
        let mut rng = rand::rng();
        let cells = (0..width * height)
            .map(|_| match rng.random_range(0..3) {
                0 => Cell::Red,
                1 => Cell::Green,
                _ => Cell::Blue,
            })
            .collect();
        Grid {
            width,
            height,
            cells,
        }
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn positions(&self) -> impl Iterator<Item = (usize, usize)> {
        (0..self.height).flat_map(move |y| (0..self.width).map(move |x| (x, y)))
    }

    pub fn cells(&self) -> impl Iterator<Item = &Cell> {
        self.cells.iter()
    }

    pub fn get(&self, x: usize, y: usize) -> Cell {
        self.cells[self.idx(x, y)]
    }

    pub fn get_checked(&self, x: isize, y: isize) -> Option<Cell> {
        if self.in_bounds(x, y) {
            Some(self.get(x as usize, y as usize))
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: Cell) {
        let idx = self.idx(x, y);
        self.cells[idx] = value;
    }

    fn in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize
    }

    pub fn count_matching_neighbors(&self, x: usize, y: usize, target: Cell) -> usize {
        self.neighbors(x, y)
            .filter(|&neighbor| neighbor == target)
            .count()
    }

    fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = Cell> {
        (-1..=1)
            .flat_map(move |dy| (-1..=1).map(move |dx| (dx, dy)))
            .filter(|&(dx, dy)| !(dx == 0 && dy == 0))
            .filter_map(move |(dx, dy)| {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                self.get_checked(nx, ny)
            })
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Grid {
        pub fn new_filled(width: usize, height: usize, cell: Cell) -> Self {
            let cells = vec![cell; width * height];
            Self {
                width,
                height,
                cells,
            }
        }
    }

    #[test]
    fn dimensions_are_correct() {
        let cases = [(10, 10), (10, 5), (1, 1), (0, 0)];

        for (width, height) in cases {
            let grid = Grid::new_random(width, height);

            assert_eq!(grid.dimensions(), (width, height));
        }
    }

    #[test]
    fn set_and_get_cell() {
        let mut grid = Grid::new_random(2, 2);
        let target = Cell::Red;

        grid.set(0, 0, target);

        assert_eq!(grid.get(0, 0), target);
    }

    #[test]
    fn positions_cover_entire_grid() {
        let grid = Grid::new_random(2, 2);

        let positions: Vec<(usize, usize)> = grid.positions().collect();

        assert_eq!(positions, vec![(0, 0), (1, 0), (0, 1), (1, 1)]);
    }

    #[test]
    fn cells_iter_matches_storage_length() {
        let grid = Grid::new_random(4, 3);

        assert_eq!(grid.cells().count(), 12);
    }

    #[test]
    fn get_checked_returns_none_out_of_bounds() {
        let grid = Grid::new_random(2, 2);

        assert_eq!(grid.get_checked(-1, 0), None);
        assert_eq!(grid.get_checked(0, -1), None);
        assert_eq!(grid.get_checked(2, 2), None);
    }

    #[test]
    fn correct_number_of_neighbors() {
        let mut grid = Grid::new_filled(2, 2, Cell::Red);
        grid.set(0, 0, Cell::Blue);

        assert_eq!(grid.count_matching_neighbors(0, 0, Cell::Red), 3);
        assert_eq!(grid.count_matching_neighbors(1, 0, Cell::Red), 2);
        assert_eq!(grid.count_matching_neighbors(0, 0, Cell::Blue), 0);
        assert_eq!(grid.count_matching_neighbors(0, 1, Cell::Blue), 1);
    }

    #[test]
    fn neighbors_exclude_center() {
        let grid = Grid::new_filled(3, 3, Cell::Red);

        assert_eq!(grid.count_matching_neighbors(1, 1, Cell::Red), 8);
    }
}
