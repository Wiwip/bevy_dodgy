use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_dodgy::agents::{AgentGoal, AgentInfo, AvoidanceOptionsComponent};
use bevy_dodgy::DodgyPlugin;
use dodgy_2d::AvoidanceOptions;
use rand::Rng;

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
    commands.spawn(Camera2d);

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
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(AgentGoal {
                dest: Vec2::new(right_x + 200.0, 0.0),
                tolerance: 4.0,
            })
            .insert(TransformBundle::from(Transform::from_translation(
                Vec3::new(right_x + -100.0, -250.0 + 20. * i as f32, 0.0),
            )))
            .insert(AvoidanceOptionsComponent(AvoidanceOptions {
                obstacle_margin: 2.1,
                time_horizon: 3.0,
                obstacle_time_horizon: 1.0,
            }))
            .insert(CollisionLayers::new(LayerMask(0b1111), LayerMask(0b1111)))
            .insert(
                DebugRender::default().with_collider_color(Srgba::hex("#a52c4c").unwrap().into()),
            );

        commands
            .spawn(AgentInfo {
                radius: 8.0,
                avoidance_responsibility: rng.gen_range(1.0..2.0),
                max_speed: 30.0,
            })
            .insert(RigidBody::Dynamic)
            //.insert(LockedAxes::new().lock_rotation_x().lock_rotation_z())
            .insert(AgentGoal {
                dest: Vec2::new(right_x + -200.0, 0.0),
                tolerance: 4.0,
            })
            .insert(TransformBundle::from(Transform::from_translation(
                Vec3::new(right_x + 100.0, -250.0 + 20. * i as f32, 0.0),
            )))
            .insert(AvoidanceOptionsComponent(AvoidanceOptions {
                obstacle_margin: 2.1,
                time_horizon: 3.0,
                obstacle_time_horizon: 1.0,
            }))
            .insert(CollisionLayers::new(LayerMask(0b1111), LayerMask(0b1111)))
            .insert(
                DebugRender::default().with_collider_color(Srgba::hex("#2e41a1").unwrap().into()),
            );
    }

    // Makes agents that have no prediction at all
    /*let left_x = -400.0;
    for i in 0..20 {
        commands
            .spawn(AgentInfo {
                radius: 8.0,
                avoidance_responsibility: 1.0,
                max_speed: 30.0,
            })
            .insert(AvoidanceOptionsComponent(AvoidanceOptions {
                obstacle_margin: 0.1,
                time_horizon: 0.0001,
                obstacle_time_horizon: 1.0,
            }))
            .insert(RigidBody::Dynamic)
            //.insert(LockedAxes::new().lock_rotation_x().lock_rotation_z())
            .insert(AgentGoal(Vec2::new(0.0, left_x + 200.0)))
            .insert(TransformBundle::from(Transform::from_translation(
                Vec3::new(-250.0 + 20.0 * i as f32, left_x + -100.0, 0.0),
            )))
            .insert(CollisionLayers::new(LayerMask(0b1111), LayerMask(0b1111)))
            .insert(
                DebugRender::default().with_collider_color(Srgba::hex("#a52c4c").unwrap().into()),
            );

        commands
            .spawn(AgentInfo {
                radius: 8.0,
                avoidance_responsibility: 1.00,
                max_speed: 30.0,
            })
            .insert(RigidBody::Dynamic)
            //.insert(LockedAxes::new().lock_rotation_x().lock_rotation_z())
            .insert(AgentGoal(Vec2::new(0.0, left_x + -200.0)))
            .insert(TransformBundle::from(Transform::from_translation(
                Vec3::new(-250.0 + 20.0 * i as f32, left_x + 100.0, 0.0),
            )))
            .insert(AvoidanceOptionsComponent(AvoidanceOptions {
                obstacle_margin: 0.1,
                time_horizon: 0.0001,
                obstacle_time_horizon: 1.0,
            }))
            .insert(CollisionLayers::new(LayerMask(0b1111), LayerMask(0b1111)))
            .insert(
                DebugRender::default().with_collider_color(Srgba::hex("#2e41a1").unwrap().into()),
            );

     */
}

