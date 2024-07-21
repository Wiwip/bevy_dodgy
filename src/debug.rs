use bevy::app::{App, Plugin};
use bevy::prelude::*;
use avian3d::prelude::*;
use dodgy_2d::Obstacle;
use crate::obstacles::{AsObstacle, TransformObstacle};

pub struct DodgyDebugPlugin;

impl Plugin for DodgyDebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<DodgyDebugGizmos>()
            .add_systems(Startup, setup_debug_gizmos)
            .add_systems(
            PostUpdate,
            (display_dodgy_obstacles, display_agent_velocity),
        );
    }
}

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct DodgyDebugGizmos {}

fn setup_debug_gizmos(mut config_store: ResMut<GizmoConfigStore>,){
    let (config, _) = config_store.config_mut::<DodgyDebugGizmos>();
    config.line_style = GizmoLineStyle::Dotted;
}

fn display_dodgy_obstacles(
    query: Query<(&Transform, &RigidBody, &Collider)>,
    mut gizmos: Gizmos<DodgyDebugGizmos>
) {
    for (tf, body, collider) in query.iter() {
        if body.is_dynamic() || body.is_kinematic() {
            continue;
        }

        if let Some(mut obstacle) = collider.to_obstacle() {
            obstacle.transform_points(tf);

            match obstacle {
                Obstacle::Closed { vertices } => {
                    let mut vertices_3d: Vec<Vec3> =
                        vertices.iter().map(|v| Vec3::new(v.x, 1., v.y)).collect();

                    if !vertices_3d.is_empty() {
                        vertices_3d.push(vertices_3d[0]); // Adds a line to close the shape
                    }

                    gizmos.linestrip(vertices_3d, Srgba::hex("#9F2B68").unwrap());
                }
                Obstacle::Open { vertices } => {
                    let vertices_3d: Vec<Vec3> =
                        vertices.iter().map(|v| Vec3::new(v.x, 1., v.y)).collect();

                    gizmos.linestrip(vertices_3d, Srgba::hex("#301934").unwrap());
                }
            }
        }
    }
}

fn display_agent_velocity(query: Query<(&Transform, &LinearVelocity)>, mut gizmos: Gizmos) {
    for (tf, linvel) in query.iter() {
        gizmos.line(tf.translation, tf.translation + linvel.0, Srgba::hex("#6b8e2c").unwrap())
    }
}
