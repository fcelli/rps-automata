use crate::grid::Grid;
use crate::{cell::Cell, config::AppConfig};
use bevy::prelude::*;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_simulation)
            .add_systems(FixedUpdate, step_automata);
    }
}

fn setup_simulation(mut commands: Commands, config: Res<AppConfig>) {
    commands.insert_resource(Time::<Fixed>::from_hz(config.sim_tick_rate_hz));
}

fn predator_of(cell: Cell) -> Cell {
    match cell {
        Cell::Red => Cell::Blue,
        Cell::Green => Cell::Red,
        Cell::Blue => Cell::Green,
    }
}

fn count_predators(grid: &Grid, x: usize, y: usize) -> usize {
    let mut count = 0;
    let current = grid.get(x, y);
    let predator = predator_of(current);

    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx < 0 || nx >= grid.width as isize || ny < 0 || ny >= grid.height as isize {
                continue;
            }
            let cell = grid.get(nx as usize, ny as usize);
            if cell == predator {
                count += 1;
            }
        }
    }
    return count;
}

fn step_automata(mut grid: ResMut<Grid>, config: Res<AppConfig>) {
    let mut next = grid.clone();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let current: Cell = grid.get(x, y);
            let predator = predator_of(current);

            let predators = count_predators(&grid, x, y);
            if predators >= config.predators_threshold {
                next.set(x, y, predator);
            }
        }
    }
    *grid = next;
}
