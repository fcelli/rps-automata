mod cell;
mod grid;
mod rendering;
mod simulation;

use crate::rendering::RenderingPlugin;
use crate::simulation::SimulationPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(SimulationPlugin)
        .add_plugins(RenderingPlugin)
        .run();
}
