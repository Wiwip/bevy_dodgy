use crate::obstacles::Obstacle;
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

pub struct DodgyDebugPlugin;

impl Plugin for DodgyDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, (display_dodgy_obstacles, display_agent_velocity));
    }
}

fn display_dodgy_obstacles(query: Query<(&Transform, &RigidBody, &Collider)>, mut gizmos: Gizmos) {
    for (tf, body, collider) in query.iter() {
        if body.is_dynamic() || body.is_kinematic() {
            continue;
        }

        if let Ok(mut obstacle) = Obstacle::try_from(collider) {
            obstacle.transform(tf);

            match obstacle {
                Obstacle::Closed { vertices } => {
                    let mut vertices_3d: Vec<Vec3> = vertices.iter().map(|v|{
                        Vec3::new(v.x, 1., v.y)
                    }).collect();

                    if !vertices_3d.is_empty() {
                        vertices_3d.push(vertices_3d[0]); // Adds a line to close the shape
                    }

                    gizmos.linestrip(vertices_3d, Color::PURPLE);
                }
                Obstacle::Open { vertices } => {
                    let vertices_3d: Vec<Vec3> = vertices.iter().map(|v|{
                        Vec3::new(v.x, 1., v.y)
                    }).collect();


                    gizmos.linestrip(vertices_3d, Color::DARK_GREEN);
                }
            }
        }
    }
}

fn display_agent_velocity(query: Query<(&Transform, &LinearVelocity)>, mut gizmos: Gizmos) {
    for (tf, linvel) in query.iter() {
        gizmos.line(tf.translation, tf.translation + linvel.0, Color::SEA_GREEN)
    }
}
