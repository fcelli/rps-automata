use bevy::prelude::*;
use rand::{self, RngExt};

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Grid::new_random(100, 100));
    }
}

#[derive(Clone, Copy)]
pub enum Cell {
    Red,
    Green,
    Blue,
}

impl Cell {
    pub fn color(self) -> [u8; 4] {
        match self {
            Cell::Red => [255, 0, 0, 255],
            Cell::Green => [0, 255, 0, 255],
            Cell::Blue => [0, 0, 255, 255],
        }
    }

    pub fn beats(self, other: Cell) -> bool {
        matches!(
            (self, other),
            (Cell::Red, Cell::Green) | (Cell::Green, Cell::Blue) | (Cell::Blue, Cell::Red),
        )
    }
}

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
}
