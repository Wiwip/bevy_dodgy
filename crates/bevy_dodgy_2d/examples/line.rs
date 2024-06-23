use bevy::asset::AssetContainer;
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

use bevy_dodgy_2d::agents::AgentGoal;
use bevy_dodgy_2d::agents::AgentInfo;
use bevy_dodgy_2d::plugin::DodgyPlugin;
use bevy_dodgy_2d::AvoidanceOptions;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default())
        .add_plugins(DodgyPlugin)
        .add_systems(Startup, setup)
        .insert_resource(Gravity(Vec2::ZERO))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let right_x = 400.0;
    for i in 0..20 {
        commands
            .spawn(AgentInfo {
                radius: 8.0,
                avoidance_responsibility: 1.0,
                max_speed: 30.0,
            })
            .insert(RigidBody::Dynamic)
            .insert(AgentGoal(Vec2::new(right_x + 200.0, 0.0)))
            .insert(TransformBundle::from(Transform::from_translation(
                Vec3::new(right_x + -100.0, -250.0 + 20. * i as f32, 0.0),
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
                avoidance_responsibility: 1.0,
                max_speed: 30.0,
            })
            .insert(RigidBody::Dynamic)
            .insert(AgentGoal(Vec2::new(right_x + -200.0, 0.0)))
            .insert(TransformBundle::from(Transform::from_translation(
                Vec3::new(right_x + 100.0, -250.0 + 20. * i as f32, 0.0),
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
            .insert(RigidBody::Dynamic)
            .insert(AgentGoal(Vec2::new(left_x + 200.0, 0.0)))
            .insert(TransformBundle::from(Transform::from_translation(
                Vec3::new(left_x + -100.0, -250.0 + 20.0 * i as f32, 0.0),
            )))
            .insert(AvoidanceOptions {
                obstacle_margin: 0.1,
                time_horizon: 0.0,
                obstacle_time_horizon: 1.0,
            })
            .insert(CollisionLayers::new(LayerMask(0b1111), LayerMask(0b1111)))
            .insert(DebugRender::default().with_collider_color(Color::RED));

        commands
            .spawn(AgentInfo {
                radius: 8.0,
                avoidance_responsibility: 1.00,
                max_speed: 30.0,
            })
            .insert(RigidBody::Dynamic)
            .insert(AgentGoal(Vec2::new(left_x + -200.0, 0.0)))
            .insert(TransformBundle::from(Transform::from_translation(
                Vec3::new(left_x + 100.0, -250.0 + 20.0 * i as f32, 0.0),
            )))
            .insert(AvoidanceOptions {
                obstacle_margin: 0.1,
                time_horizon: 0.0,
                obstacle_time_horizon: 1.0,
            })
            .insert(CollisionLayers::new(LayerMask(0b1111), LayerMask(0b1111)))
            .insert(DebugRender::default().with_collider_color(Color::BLUE));
    }
}
