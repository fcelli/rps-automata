mod rendering;
mod simulation;
use crate::rendering::RenderingPlugin;
use crate::simulation::SimulationPlugin;
use bevy::prelude::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SimulationPlugin)
        .add_plugins(RenderingPlugin)
        .run();
}
