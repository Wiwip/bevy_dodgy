use avian3d::prelude::*;

use bevy::camera::{Exposure, PhysicalCameraParameters};
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::light::CascadeShadowConfigBuilder;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;
use bevy_dodgy_2d::agents::{AgentInfo, AvoidanceOptionsComponent, IdealVelocity, PreferredVelocity};
use bevy_dodgy_2d::DodgyPlugin;
use bevy_top_down_camera::{CameraController, TopDownCameraPlugin};
use dodgy_2d::AvoidanceOptions;
use rand::RngExt;
use std::f32::consts::PI;
use bevy_dodgy_2d::debug::DodgyDebugPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default())
        .add_plugins(DodgyPlugin)
        //.add_plugins(DodgyDebugPlugin)
        .add_plugins(TopDownCameraPlugin)
        .add_systems(Startup, spawn_entities)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, find_new_destination)
        .add_systems(Update, (set_desired_velocity, apply_ideal_velocity).chain())
        .insert_resource(Gravity(Vec3::ZERO))
        .run();
}

#[derive(Component)]
pub struct Goal(Vec3);

impl Goal {
    fn random() -> Self {
        let mut rng = rand::rng();
        let x = rng.random_range(-400.0..400.0);
        let z = rng.random_range(-400.0..400.0);
        Self(Vec3::new(x, 0.0, z))
    }
}

fn spawn_entities(mut commands: Commands) {
    let mut rng = rand::rng();
    for _ in 0..2000 {
        commands
            .spawn(AgentInfo {
                radius: 2.1,
                avoidance_responsibility: 1.0,
                max_speed: 12.0,
            })
            .insert(AvoidanceOptionsComponent(AvoidanceOptions {
                obstacle_margin: 1.0,
                time_horizon: 1.0,
                obstacle_time_horizon: 1.0,
            }))
            .insert(Goal::random())
            .insert(RigidBody::Dynamic)
            .insert(LockedAxes::new().lock_rotation_x().lock_rotation_z())
            .insert(Transform::from_translation(Vec3::new(
                rng.random_range(-400.0..400.0),
                0.0,
                rng.random_range(-400.0..400.0),
            )))
            .insert(Collider::capsule(2.0, 4.25))
            .insert(CollisionLayers::new(LayerMask(0b1111), LayerMask(0b1111)))
            .insert(
                DebugRender::default().with_collider_color(Srgba::hex("#a52c4c").unwrap().into()),
            );
    }
}

pub fn find_new_destination(mut query: Query<(&GlobalTransform, &mut Goal)>) {
    let mut rng = rand::rng();

    for (tf, mut goal) in query.iter_mut() {
        if tf.translation().distance_squared(goal.0) < 0.1 {
            goal.0 = Vec3::new(
                rng.random_range(-200.0..200.0),
                0.0,
                rng.random_range(-200.0..200.0),
            );
        }
    }
}

pub fn set_desired_velocity(
    mut query: Query<(&mut Transform, &Goal, &mut PreferredVelocity, &AgentInfo)>,
) {
    for (tf, goal, mut pref_vel, agent) in query.iter_mut() {
        let direction = (goal.0 - tf.translation).normalize_or_zero() * agent.max_speed;
        pref_vel.0 = direction;
    }
}

pub fn apply_ideal_velocity(mut query: Query<(&mut LinearVelocity, &IdealVelocity)>) {
    for (mut vel, ideal) in query.iter_mut() {
        vel.0 = ideal.0;
    }
}

pub fn spawn_camera(mut commands: Commands) {
    println!("Spawn camera");
    // camera
    commands.spawn((
        Camera3d::default(),
        Camera {
            //hdr: true, // 1. HDR is required for bloom
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
        Bloom::NATURAL,             // 3. Enable bloom for the camera
        Transform::from_xyz(0.0, 100.0, 42.0).looking_at(Vec3::ZERO, Vec3::Y),
        Exposure::from_physical_camera(PhysicalCameraParameters::default()),
        CameraController::default(),
    ));

    // directional 'sun' light
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(10.0, 2.0, 5.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .build(),
    ));
}
