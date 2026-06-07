use crate::automata::{Grid, step};
use crate::config::AppConfig;
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

fn step_automata(mut grid: ResMut<Grid>, config: Res<AppConfig>) {
    *grid = step(&grid, config.predators_threshold);
}
