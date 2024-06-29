use crate::obstacles::{Obstacle};
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

pub struct DodgyDebugPlugin;

impl Plugin for DodgyDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, (display_collider, display_agent_velocity));
    }
}

fn display_collider(query: Query<(&Transform, &RigidBody, &Collider)>, mut gizmos: Gizmos) {
    for (tf, body, collider) in query.iter() {
        if body.is_dynamic() || body.is_kinematic() {
            continue;
        }

        if let Ok(mut obstacle) = Obstacle::try_from(collider) {
            obstacle.transform(tf);

            match obstacle {
                Obstacle::Closed { vertices } => {
                    gizmos.linestrip_2d(vertices, Color::PURPLE);
                }
                Obstacle::Open { vertices } => {
                    gizmos.linestrip_2d(vertices, Color::PURPLE);
                }
            }
        }
    }
}

fn display_agent_velocity(query: Query<(&Transform, &LinearVelocity)>, mut gizmos: Gizmos) {
    for (tf, linvel) in query.iter() {
        gizmos.line_2d(tf.translation.xy(), tf.translation.xy() + linvel.0, Color::SEA_GREEN)
    }
}