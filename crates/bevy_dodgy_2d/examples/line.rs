use bevy::asset::AssetContainer;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use rand::Rng;
use bevy_dodgy_2d::agents::AgentGoal;
use bevy_dodgy_2d::agents::AgentInfo;
use bevy_dodgy_2d::{AvoidanceOptions, DodgyPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default())
        .add_plugins(DodgyPlugin)
        .add_systems(Startup, setup)
        .insert_resource(Gravity(Vec3::ZERO))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 1200.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    let mut rng = rand::thread_rng();

    let right_x = 400.0;
    for i in 0..20 {
        commands
            .spawn(AgentInfo {
                radius: 8.0,
                avoidance_responsibility: rng.gen_range(1.0..2.0),
                max_speed: 30.0,
            })
            .insert(RigidBody::Dynamic)
            .insert(LockedAxes::new().lock_rotation_x().lock_rotation_z())
            .insert(AgentGoal(Vec3::new(0.0, 0.0, right_x + 200.0)))
            .insert(TransformBundle::from(Transform::from_translation(
                Vec3::new(-250.0 + 20. * i as f32, 0.0, right_x + -100.0),
            )))
            .insert(AvoidanceOptions {
                obstacle_margin: 0.1,
                time_horizon: 3.0,
                obstacle_time_horizon: 1.0,
            })
            .insert(CollisionLayers::new(LayerMask(0b1111), LayerMask(0b1111)))
            .insert(DebugRender::default().with_collider_color(Color::RED));

        commands
            .spawn(AgentInfo {
                radius: 8.0,
                avoidance_responsibility: rng.gen_range(1.0..2.0),
                max_speed: 30.0,
            })
            .insert(RigidBody::Dynamic)
            .insert(LockedAxes::new().lock_rotation_x().lock_rotation_z())
            .insert(AgentGoal(Vec3::new(0.0, 0.0, right_x + -200.0)))
            .insert(TransformBundle::from(Transform::from_translation(
                Vec3::new(-250.0 + 20. * i as f32, 0.0, right_x + 100.0),
            )))
            .insert(AvoidanceOptions {
                obstacle_margin: 0.1,
                time_horizon: 3.0,
                obstacle_time_horizon: 1.0,
            })
            .insert(CollisionLayers::new(LayerMask(0b1111), LayerMask(0b1111)))
            .insert(DebugRender::default().with_collider_color(Color::BLUE));
    }

    // Makes agents that have no prediction at all
    let left_x = -400.0;
    for i in 0..20 {
        commands
            .spawn(AgentInfo {
                radius: 8.0,
                avoidance_responsibility: 1.0,
                max_speed: 30.0,
            })
            .insert(AvoidanceOptions {
                obstacle_margin: 0.1,
                time_horizon: 0.0001,
                obstacle_time_horizon: 1.0,
            })
            .insert(RigidBody::Dynamic)
            .insert(LockedAxes::new().lock_rotation_x().lock_rotation_z())
            .insert(AgentGoal(Vec3::new(0.0, 0.0, left_x + 200.0)))
            .insert(TransformBundle::from(Transform::from_translation(
                Vec3::new(-250.0 + 20.0 * i as f32, 0.0, left_x + -100.0),
            )))

            .insert(CollisionLayers::new(LayerMask(0b1111), LayerMask(0b1111)))
            .insert(DebugRender::default().with_collider_color(Color::RED));

        commands
            .spawn(AgentInfo {
                radius: 8.0,
                avoidance_responsibility: 1.00,
                max_speed: 30.0,
            })
            .insert(RigidBody::Dynamic)
            .insert(LockedAxes::new().lock_rotation_x().lock_rotation_z())
            .insert(AgentGoal(Vec3::new(0.0, 0.0, left_x + -200.0)))
            .insert(TransformBundle::from(Transform::from_translation(
                Vec3::new(-250.0 + 20.0 * i as f32, 0.0, left_x + 100.0),
            )))
            .insert(AvoidanceOptions {
                obstacle_margin: 0.1,
                time_horizon: 0.0001,
                obstacle_time_horizon: 1.0,
            })
            .insert(CollisionLayers::new(LayerMask(0b1111), LayerMask(0b1111)))
            .insert(DebugRender::default().with_collider_color(Color::BLUE));
    }
}
