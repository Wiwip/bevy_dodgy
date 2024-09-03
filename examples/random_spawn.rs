use avian3d::math::Vector;
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_dodgy::agents::{AgentGoal, AgentInfo, AvoidanceOptionsComponent};
use bevy_dodgy::debug::DodgyDebugPlugin;
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
        transform: Transform::from_xyz(0.0, 1200.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    let mut rng = rand::thread_rng();
    for _ in 0..2000 {
        commands
            .spawn(AgentInfo {
                radius: 8.0,
                avoidance_responsibility: 1.0,
                max_speed: 30.0,
            })
            .insert(AvoidanceOptionsComponent(AvoidanceOptions {
                obstacle_margin: 8.1,
                time_horizon: 5.0,
                obstacle_time_horizon: 3.0,
            }))
            .insert(RigidBody::Dynamic)
            .insert(LockedAxes::new().lock_rotation_x().lock_rotation_z())
            .insert(AgentGoal(Vec3::new(
                rng.gen_range(-2000.0..2000.0),
                0.0,
                rng.gen_range(-2000.0..2000.0),
            )))
            .insert(TransformBundle::from(Transform::from_translation(
                Vec3::new(
                    rng.gen_range(-2000.0..2000.0),
                    0.0,
                    rng.gen_range(-2000.0..2000.0),
                ),
            )))
            .insert(CollisionLayers::new(LayerMask(0b1111), LayerMask(0b1111)))
            .insert(
                DebugRender::default().with_collider_color(Srgba::hex("#a52c4c").unwrap().into()),
            );
    }

    commands.spawn((
        RigidBody::Static,
        TransformBundle::from(Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))),
        Collider::cuboid(150.0, 1.0, 150.0),
        CollisionLayers::new(LayerMask(0b1111), LayerMask(0b1111)),
        DebugRender::default().with_collider_color(Srgba::hex("#b86830").unwrap().into()),
    ));

    commands.spawn((
        RigidBody::Static,
        TransformBundle::from(Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))),
        Collider::triangle(
            Vector::new(300.0, 0.0, 400.0),
            Vector::new(200.0, 0.0, 300.0),
            Vector::new(400.0, 0.0, 300.0),
        ),
        CollisionLayers::new(LayerMask(0b1111), LayerMask(0b1111)),
        DebugRender::default().with_collider_color(Srgba::hex("#b86830").unwrap().into()),
    ));
}
