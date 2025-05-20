use crate::agents::{AgentGoal, AgentQueryData};
use avian3d::prelude::*;
use bevy::app::{App, Plugin};
use bevy::color::palettes::basic::BLUE;
use bevy::color::palettes::css::PURPLE;
use bevy::prelude::*;
use std::f32::consts::PI;

pub struct DodgyDebugPlugin;

impl Plugin for DodgyDebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<DodgyDebugGizmos>()
            .add_systems(Startup, setup_debug_gizmos)
            .add_systems(PostUpdate, (display_dodgy_agents, display_agent_velocity));
    }
}

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct DodgyDebugGizmos {}

fn setup_debug_gizmos(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<DodgyDebugGizmos>();
    config.line.style = GizmoLineStyle::Dotted;
}

fn display_dodgy_agents(
    query: Query<AgentQueryData>,
    mut gizmos: Gizmos,
) {
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

fn display_agent_velocity(
    query: Query<(&GlobalTransform, &LinearVelocity, &AgentGoal)>,
    mut gizmos: Gizmos,
) {
    for (tf, linvel, goal) in query.iter() {
        gizmos.line(tf.translation(), tf.translation() + linvel.0, BLUE);
        gizmos.line(tf.translation(), goal.dest, PURPLE);
    }
}
