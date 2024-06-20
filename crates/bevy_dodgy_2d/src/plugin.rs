use crate::system::{create_collider, rvo_avoidance};
use bevy::prelude::*;

pub struct DodgyPlugin;

impl Plugin for DodgyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, create_collider)
            .add_systems(Update, rvo_avoidance);
    }
}
