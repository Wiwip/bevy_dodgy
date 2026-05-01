pub mod agents;
pub mod debug;
pub mod geometry;
mod obstacles;
mod systems;

use crate::systems::rvo_avoidance;
use bevy::app::{App, Plugin};
use bevy::prelude::*;
pub use dodgy_2d::AvoidanceOptions;

#[cfg(feature = "2d")]
use avian2d::prelude::PhysicsStepSystems;

use crate::debug::DodgyDebugGizmos;
#[cfg(feature = "3d")]
use avian3d::prelude::PhysicsStepSystems;

pub struct DodgyPlugin;

impl Plugin for DodgyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, rvo_avoidance.in_set(PhysicsStepSystems::Finalize))
            .insert_gizmo_config::<DodgyDebugGizmos>(
                DodgyDebugGizmos,
                GizmoConfig {
                    line: GizmoLineConfig {
                        width: 2.0,
                        ..default()
                    },
                    depth_bias: -1.0,
                    ..default()
                },
            );
    }
}
