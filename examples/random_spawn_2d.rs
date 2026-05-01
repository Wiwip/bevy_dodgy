
use avian2d::prelude::*;
use bevy::camera::{Exposure, PhysicalCameraParameters};
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;
use bevy_dodgy_2d::agents::{DodgyVelocity, AgentIdealVelocity, AgentInfo, AvoidanceOptionsComponent};
use bevy_dodgy_2d::DodgyPlugin;
use bevy_top_down_camera::{CameraController, TopDownCameraPlugin};
use dodgy_2d::AvoidanceOptions;
use rand::RngExt;

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
        .add_systems(Update, apply_velocity)
        .insert_resource(Gravity(Vec2::ZERO))
        .run();
}

fn spawn_entities(mut commands: Commands) {
    let mut rng = rand::rng();
    for _ in 0..600 {
        commands
            .spawn(AgentInfo {
                radius: 5.0,
                avoidance_responsibility: 1.0,
                max_speed: 40.0,
            })
            .insert(AvoidanceOptionsComponent(AvoidanceOptions {
                obstacle_margin: 1.0,
                time_horizon: 1.0,
                obstacle_time_horizon: 1.0,
            }))
            .insert(DodgyVelocity {
                desired: Vec2::new(
                    rng.random_range(-1000.0..1000.0),
                    rng.random_range(-1000.0..1000.0),
                ),
                tolerance: 1.0,
            })
            .insert(RigidBody::Dynamic)
            .insert(LockedAxes::new())
            .insert(Transform::from_translation(Vec3::new(
                rng.random_range(-1000.0..1000.0),
                rng.random_range(-1000.0..1000.0),
                0.0,
            )))
            .insert(Collider::rectangle(5.0, 5.0)) // cuboid(1.0, 1.0, 1.0))
            .insert(CollisionLayers::new(LayerMask(0b1111), LayerMask(0b1111)))
            .insert(
                DebugRender::default().with_collider_color(Srgba::hex("#a52c4c").unwrap().into()),
            );
    }
}

pub fn find_new_destination(mut query: Query<(&GlobalTransform, &mut DodgyVelocity)>) {
    let mut rng = rand::rng();

    for (tf, mut agent) in query.iter_mut() {
        if tf.translation().xz().distance_squared(agent.desired) < agent.tolerance * agent.tolerance {
            agent.desired = Vec2::new(
                rng.random_range(-2000.0..2000.0),
                rng.random_range(-2000.0..2000.0),
            );
        }
    }
}

pub fn spawn_camera(mut commands: Commands) {
    println!("Spawn camera");
    // camera
    commands.spawn((
        Camera2d::default(),
        Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
        Bloom::NATURAL,             // 3. Enable bloom for the camera
        //Transform::from_xyz(0.0, 0.0, 250.0).looking_at(Vec3::ZERO, Vec3::Y),
        Exposure::from_physical_camera(PhysicalCameraParameters::default()),
        CameraController::default(),
    ));

    // directional 'sun' light
    /*commands.spawn((
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
    ));*/
}


pub fn apply_velocity(mut query: Query<(&mut LinearVelocity, &AgentIdealVelocity)>) {
    for (mut vel, ideal_vel) in query.iter_mut() {
        vel.0 = ideal_vel.0;
    }
}
