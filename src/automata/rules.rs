use crate::automata::Grid;

pub fn step(grid: &Grid, threshold: usize) -> Grid {
    let mut next = grid.clone();

    for (x, y) in grid.positions() {
        let current = grid.get(x, y);
        let predator = current.predator();

        if grid.count_matching_neighbors(x, y, predator) >= threshold {
            next.set(x, y, predator);
        }
    }
    next
}
