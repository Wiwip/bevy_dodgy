use crate::agents::{AgentGoal, AgentIdealVelocity, AgentInfo, AgentQueryData};
#[cfg(feature = "2d")]
use avian2d::prelude::*;
#[cfg(feature = "3d")]
use avian3d::prelude::*;
use std::borrow::Cow;

use crate::obstacles::{AsObstacle, TransformObstacle};
use bevy::app::{App, Plugin};
use bevy::color::palettes::basic::BLUE;
use bevy::color::palettes::css::PURPLE;
use bevy::prelude::*;
use dodgy_2d::Obstacle;
use std::f32::consts::PI;

pub struct DodgyDebugPlugin;

impl Plugin for DodgyDebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<DodgyDebugGizmos>()
            .add_systems(Startup, setup_debug_gizmos)
            .add_systems(PostUpdate, (display_dodgy_agents, display_agent_velocity));

        #[cfg(feature = "2d")]
        app.add_systems(PostUpdate, display_obstacles);
    }
}

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct DodgyDebugGizmos {}

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
fn display_agent_velocity(
    query: Query<(&GlobalTransform, &AgentIdealVelocity, &AgentGoal)>,
    mut gizmos: Gizmos,
) {
    for (tf, ideal_velocity, goal) in query.iter() {
        gizmos.line(tf.translation(), tf.translation() + ideal_velocity.0, BLUE);
        gizmos.line(tf.translation(), goal.dest, PURPLE.with_alpha(0.1));
    }
}
#[cfg(feature = "2d")]
fn display_agent_velocity(
    query: Query<(&GlobalTransform, &LinearVelocity, &AgentGoal)>,
    mut gizmos: Gizmos,
) {
    for (tf, linvel, goal) in query.iter() {
        gizmos.line_2d(
            tf.translation().xy(),
            tf.translation().xy() + linvel.0,
            BLUE,
        );
        gizmos.line_2d(tf.translation().xy(), goal.dest.xy(), PURPLE);
    }
}

#[cfg(feature = "2d")]
fn display_obstacles(
    obstacles: Query<(&GlobalTransform, &Collider, &RigidBody), Without<AgentInfo>>,
    mut gizmos: Gizmos,
) {
    for (tf, collider, body) in obstacles {
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
