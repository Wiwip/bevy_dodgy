use bevy::asset::AssetContainer;
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use rand::Rng;

use bevy_dodgy_2d::agents::Agent;
use bevy_dodgy_2d::agents::AgentGoal;
use bevy_dodgy_2d::AvoidanceOptions;
use bevy_dodgy_2d::plugin::DodgyPlugin;

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

    let mut rng = rand::thread_rng();
    for i in 0..2000 {
        commands
            .spawn(Agent {
                radius: 8.0,
                avoidance_responsibility: 1.0,
                max_speed: 30.0,
            })
            .insert(RigidBody::Dynamic)
            .insert(AgentGoal(Vec2::new(rng.gen_range(-2000.0..2000.0), rng.gen_range(-2000.0..2000.0))))
            .insert(TransformBundle::from(Transform::from_translation(
                Vec3::new(rng.gen_range(-2000.0..2000.0), rng.gen_range(-2000.0..2000.0), 0.0),
            )))
            .insert(AvoidanceOptions {
                obstacle_margin: 0.1,
                time_horizon: 5.0,
                obstacle_time_horizon: 1.0,
            })
            .insert(CollisionLayers::new(LayerMask(0b1111), LayerMask(0b1111)))
            .insert(DebugRender::default().with_collider_color(Color::RED));

    }
}
