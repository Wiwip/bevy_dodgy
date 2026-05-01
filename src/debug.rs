use crate::agents::{AgentInfo, AgentQueryData, IdealVelocity, PreferredVelocity};
#[cfg(feature = "2d")]
use avian2d::prelude::*;
#[cfg(feature = "3d")]
use avian3d::prelude::*;
use std::f32::consts::PI;

use crate::obstacles::{AsObstacle, TransformObstacle};
use bevy::app::{App, Plugin};
use bevy::color::palettes::basic::BLUE;
use bevy::color::palettes::css::PURPLE;
use bevy::prelude::*;
use dodgy_2d::Obstacle;

pub struct DodgyDebugPlugin;

impl Plugin for DodgyDebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<DodgyDebugGizmos>()
            .add_systems(Startup, setup_debug_gizmos)
            .add_systems(PostUpdate, (display_dodgy_agents, display_agent_velocity));

        app.add_systems(PostUpdate, display_obstacles);
    }
}

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct DodgyDebugGizmos;

fn setup_debug_gizmos(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<DodgyDebugGizmos>();
    config.line.style = GizmoLineStyle::Dotted;
}

#[cfg(feature = "3d")]
fn display_dodgy_agents(query: Query<AgentQueryData>, mut gizmos: Gizmos) {
    for agent in query.iter() {
        gizmos.circle(
            Isometry3d::new(
                agent.transform.translation(),
                Quat::from_rotation_x(PI / 2.0),
            ),
            agent.info.radius,
            BLUE,
        );
    }
}

#[cfg(feature = "2d")]
fn display_dodgy_agents(query: Query<AgentQueryData>, mut gizmos: Gizmos) {
    for agent in query.iter() {
        gizmos.circle_2d(
            Isometry2d::new(agent.transform.translation().xy(), Rot2::IDENTITY),
            agent.info.radius,
            BLUE,
        );
    }
}

#[cfg(feature = "3d")]
fn display_agent_velocity(query: Query<(&GlobalTransform, &PreferredVelocity, &IdealVelocity)>, mut gizmos: Gizmos) {
    for (tf, pref, ideal) in query.iter() {
        gizmos.line(tf.translation(), tf.translation() + ideal.0, BLUE);
        gizmos.line(
            tf.translation(),
            tf.translation() + pref.0,
            PURPLE.with_alpha(0.50),
        );
    }
}
#[cfg(feature = "2d")]
fn display_agent_velocity(
    query: Query<(&GlobalTransform, &LinearVelocity, &DodgyVelocity)>,
    mut gizmos: Gizmos,
) {
    for (tf, linvel, goal) in query.iter() {
        gizmos.line_2d(
            tf.translation().xy(),
            tf.translation().xy() + linvel.0,
            BLUE,
        );
        gizmos.line_2d(tf.translation().xy(), goal.desired.xy(), PURPLE);
    }
}

#[cfg(feature = "2d")]
fn display_obstacles(
    obstacles: Query<(&GlobalTransform, &Collider, &RigidBody), Without<AgentInfo>>,
    mut gizmos: Gizmos,
) {
    for (tf, collider, _body) in obstacles {
        if let Some(mut obstacle) = collider.to_obstacle() {
            obstacle.transform_points(tf);

            match obstacle {
                Obstacle::Closed { vertices } => {
                    gizmos.linestrip_2d(vertices, BLUE);
                }
                Obstacle::Open { vertices } => {
                    gizmos.linestrip_2d(vertices, BLUE);
                }
            }
        }
    }
}

#[cfg(feature = "3d")]
fn display_obstacles(
    obstacles: Query<(&GlobalTransform, &Collider, &RigidBody), Without<AgentInfo>>,
    mut gizmos: Gizmos<DodgyDebugGizmos>,
) {
    for (tf, collider, _body) in obstacles {
        if let Some(mut obstacle) = collider.to_obstacle() {
            obstacle.transform_points(tf);

            match obstacle {
                Obstacle::Closed { vertices } => {
                    let vertices = vertices.iter().map(|vec| Vec3::new(vec.x, 0.0, vec.y));
                    gizmos.linestrip(vertices, BLUE);
                }
                Obstacle::Open { vertices } => {
                    let vertices = vertices.iter().map(|vec| Vec3::new(vec.x, 0.0, vec.y));
                    gizmos.linestrip(vertices, BLUE);
                }
            }
        }
    }
}
