use crate::cell::Cell;
use bevy::prelude::*;
use rand::RngExt;

#[derive(Resource)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
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

    fn idx(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    pub fn get(&self, x: usize, y: usize) -> Cell {
        self.cells[self.idx(x, y)]
    }

    pub fn set(&mut self, x: usize, y: usize, value: Cell) {
        let idx = self.idx(x, y);
        self.cells[idx] = value;
    }

    pub fn cells(&self) -> &[Cell] {
        &self.cells
    }
}
