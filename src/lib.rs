pub mod agents;
pub mod debug;
pub mod geometry;
mod obstacles;
mod systems;

use crate::systems::{on_add_create_collider, rvo_avoidance};
use bevy::app::{App, Plugin, PreUpdate, Update};

pub struct DodgyPlugin;

impl Plugin for DodgyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, on_add_create_collider)
            .add_systems(Update, rvo_avoidance);
    }
}
