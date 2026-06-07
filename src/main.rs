mod cell;
mod config;
mod grid;
mod rendering;
mod simulation;

use crate::config::AppConfig;
use crate::grid::Grid;
use crate::rendering::RenderingPlugin;
use crate::simulation::SimulationPlugin;
use bevy::prelude::*;

fn main() {
    let config = AppConfig {
        grid_width: 200,
        grid_height: 200,
        predators_threshold: 3,
        sim_tick_rate_hz: 20.0,
        pixel_scale: 4.0,
    };

    let grid = Grid::new_random(config.grid_width, config.grid_height);

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(config)
        .insert_resource(grid)
        .add_plugins(SimulationPlugin)
        .add_plugins(RenderingPlugin)
        .run();
}
