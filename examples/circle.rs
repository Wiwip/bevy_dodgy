use avian3d::prelude::*;
use bevy::app::{App, Startup};
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_dodgy::agents::{AgentGoal, AgentInfo, AvoidanceOptionsComponent};
use bevy_dodgy::debug::DodgyDebugPlugin;
use bevy_dodgy::geometry::point_on_circle;
use bevy_dodgy::DodgyPlugin;
use dodgy_2d::AvoidanceOptions;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default())
        .add_plugins(DodgyPlugin)
        .add_plugins(DodgyDebugPlugin)
        .add_systems(Startup, setup)
        .insert_resource(Gravity(Vec3::ZERO))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 1600.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    let num_agents = 60;
    for i in 0..num_agents {
        let theta = 2.0 * std::f32::consts::PI * (i as f32) / (num_agents as f32);
        let point = point_on_circle((0., 0.), 400., theta);

        let mut rng = rand::thread_rng();

        commands
            .spawn(AgentInfo {
                radius: 12.0,
                avoidance_responsibility: rng.gen_range(1.0..2.0),
                max_speed: 30.0,
            })
            .insert(RigidBody::Dynamic)
            .insert(LockedAxes::new().lock_rotation_x().lock_rotation_z())
            .insert(AgentGoal(Vec3::new(-point.y, 0.0, -500.0 + -point.x)))
            .insert(TransformBundle::from(Transform::from_translation(
                Vec3::new(point.y, 0.0, -500.0 + point.x),
            )))
            .insert(AvoidanceOptionsComponent(AvoidanceOptions {
                obstacle_margin: 0.1,
                time_horizon: 6.0,
                obstacle_time_horizon: 1.0,
            }))
            .insert(CollisionLayers::new(LayerMask(0b1111), LayerMask(0b0000)))
            .insert(
                DebugRender::default().with_collider_color(Srgba::hex("#a52c4c").unwrap().into()),
            );

        // Spawn agents without RVO
        commands
            .spawn(AgentInfo {
                radius: 12.0,
                avoidance_responsibility: 1.0,
                max_speed: 30.0,
            })
            .insert(RigidBody::Dynamic)
            .insert(LockedAxes::new().lock_rotation_x().lock_rotation_z())
            .insert(AgentGoal(Vec3::new(-point.y, 0.0, 500.0 + -point.x)))
            .insert(TransformBundle::from(Transform::from_translation(
                Vec3::new(point.y, 0.0, 500.0 + point.x),
            )))
            .insert(AvoidanceOptionsComponent(AvoidanceOptions {
                obstacle_margin: 0.1,
                time_horizon: 0.0001,
                obstacle_time_horizon: 0.0,
            }))
            .insert(CollisionLayers::new(LayerMask(0b1111), LayerMask(0b1111)))
            .insert(
                DebugRender::default().with_collider_color(Srgba::hex("#a52c4c").unwrap().into()),
            );
    }
}
