use bevy::prelude::*;

#[derive(Resource)]
pub struct AppConfig {
    pub grid_width: usize,
    pub grid_height: usize,
    pub predators_threshold: usize,
    pub sim_tick_rate_hz: f64,
}
