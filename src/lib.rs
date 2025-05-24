pub mod agents;
pub mod debug;
pub mod geometry;
mod obstacles;
mod systems;

use crate::systems::rvo_avoidance;
use bevy::app::{App, Plugin, Update};

pub use dodgy_2d::AvoidanceOptions;

pub struct DodgyPlugin;

impl Plugin for DodgyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, rvo_avoidance);
    }
}
