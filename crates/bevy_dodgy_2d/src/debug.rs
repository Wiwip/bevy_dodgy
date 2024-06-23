use crate::obstacles::{collider_as_obstacle, Obstacle};
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::{Collider, RigidBody};

pub struct DodgyDebugPlugin;

impl Plugin for DodgyDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, display_collider);
    }
}

fn display_collider(query: Query<(&Transform, &RigidBody, &Collider)>, mut gizmos: Gizmos) {
    for (tf, body, collider) in query.iter() {
        if body.is_dynamic() || body.is_kinematic() {
            continue;
        }

        if let Some(obstacle) = collider_as_obstacle(collider.shape_scaled().as_typed_shape(), tf) {
            match obstacle {
                Obstacle::Closed { vertices } => {
                    //info!("{:?}", vertices.clone());
                    gizmos.linestrip_2d(vertices, Color::PURPLE);
                }
                Obstacle::Open { vertices } => {
                    gizmos.linestrip_2d(vertices, Color::PURPLE);
                }
            }
        }
    }
}
