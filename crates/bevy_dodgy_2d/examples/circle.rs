use bevy::app::{App, Startup, Update};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_dodgy_2d::agents::{AgentGoal, AgentInfo};
use bevy_dodgy_2d::{AvoidanceOptions, DodgyPlugin};
use bevy_xpbd_2d::components::{CollisionLayers, LayerMask, RigidBody};
use bevy_xpbd_2d::plugins::{PhysicsDebugPlugin, PhysicsPlugins};
use bevy_xpbd_2d::prelude::{DebugRender, Gravity};

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

    let num_agents = 80;
    for i in 0..num_agents {
        let theta = 2.0 * std::f32::consts::PI * (i as f32) / (num_agents as f32);
        let point = point_on_circle((0., 0.), 400., theta);

        commands
            .spawn(AgentInfo {
                radius: 12.0,
                avoidance_responsibility: 1.0,
                max_speed: 30.0,
            })
            .insert(RigidBody::Dynamic)
            .insert(AgentGoal(Vec2::new(-400.0 + -point.0, -point.1)))
            .insert(TransformBundle::from(Transform::from_translation(
                Vec3::new(-400.0 + point.0, point.1, 0.0),
            )))
            .insert(AvoidanceOptions {
                obstacle_margin: 0.1,
                time_horizon: 6.0,
                obstacle_time_horizon: 1.0,
            })
            .insert(CollisionLayers::new(LayerMask(0b1111), LayerMask(0b0000)))
            .insert(DebugRender::default().with_collider_color(Color::ORANGE));

        // Spawn agents without RVO
        commands
            .spawn(AgentInfo {
                radius: 12.0,
                avoidance_responsibility: 1.0,
                max_speed: 30.0,
            })
            .insert(RigidBody::Dynamic)
            .insert(AgentGoal(Vec2::new(500.0 + -point.0, -point.1)))
            .insert(TransformBundle::from(Transform::from_translation(
                Vec3::new(500.0 + point.0, point.1, 0.0),
            )))
            .insert(AvoidanceOptions {
                obstacle_margin: 0.1,
                time_horizon: 0.0,
                obstacle_time_horizon: 0.0,
            })
            .insert(CollisionLayers::new(LayerMask(0b1111), LayerMask(0b1111)))
            .insert(DebugRender::default().with_collider_color(Color::ORANGE));
    }
}

fn point_on_circle(center: (f32, f32), radius: f32, theta: f32) -> (f32, f32) {
    let x = center.0 + radius * theta.cos();
    let y = center.1 + radius * theta.sin();
    (x, y)
}
