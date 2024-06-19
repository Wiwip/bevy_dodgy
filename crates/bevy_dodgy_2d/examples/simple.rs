use std::borrow::Cow;
use bevy::prelude::*;
use bevy_dodgy_2d::{Agent, AgentGoal, AgentParameters, AvoidanceOptions, Obstacle};
use log::info;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_agents)
        .add_systems(Update, (rvo_avoidance, update_position, display_agents).chain())
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct AgentColor(Color);

fn spawn_agents(mut commands: Commands) {
    for i in 0..20 {
        commands
            .spawn(AgentRadius(5.0))
            .insert(Velocity(Vec2::ZERO))
            .insert(AgentGoal {
                location: Vec2::new(200.0, 0.0),
            })
            .insert(TransformBundle::from(Transform::from_translation(
                Vec3::new(-100.0, -250.0 + 25.0 * i as f32, 0.0),
            )))
            .insert(AgentColor(Color::rgba(0.8, 0.2, 0.1, 1.0)));

        commands
            .spawn(AgentRadius(5.0))
            .insert(Velocity(Vec2::ZERO))
            .insert(AgentGoal {
                location: Vec2::new(-200.0, 0.0),
            })
            .insert(TransformBundle::from(Transform::from_translation(
                Vec3::new(100.0, -250.0 + 25.0 * i as f32, 0.0),
            )))
            .insert(AgentColor(Color::rgba(0.1, 0.1, 0.9, 1.0)));
    }
}

fn display_agents(query: Query<(&Transform, &AgentRadius, &AgentColor)>, mut gizmos: Gizmos) {
    for (tf, radius, color) in query.iter() {
        gizmos.circle_2d(
            tf.translation.xy(),
            radius.0,
            color.0,
        );
    }
}

#[derive(Component, Clone, PartialEq, Debug)]
pub struct AgentRadius(pub f32);

#[derive(Component, Clone, PartialEq, Debug)]
pub struct Velocity(pub Vec2);

fn update_position(
    mut query: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>,
) {
    for (mut tf, vel) in query.iter_mut() {
        tf.translation = tf.translation + (vel.0 * time.delta_seconds()).extend(0.);
    }
}

fn rvo_avoidance(
    agents: Query<Entity>,
    mut query: Query<(Entity, &Transform, &mut Velocity, &AgentRadius, &AgentGoal)>,
    time: Res<Time>,
    mut gizmos: Gizmos,
) {
    if !(time.delta_seconds() > 0.0) {
        return;
    }

    let time_horizon = 2.0;
    let obstacle_time_horizon = 1.0;

    for entity in agents.iter() {
        let mut other_agents: Vec<Cow<'static, Agent>> = vec![];
        let _ = query.iter().map(|(e, tf, vel, radius, goal)| {
            if entity == e { return; }

            other_agents.push(Cow::Owned(Agent {
                position: tf.translation.xy(),
                velocity: vel.0,
                radius: 5.0,
                avoidance_responsibility: 1.0,
            }));
        }).collect::<Vec<_>>();

        let Ok((e, tf, mut vel, radius, goal)) = query.get_mut(entity) else { continue; };

        let agent_max_speed = 25.0;
        let preferred_velocity = (goal.location - tf.translation.xy()).normalize_or_zero() * agent_max_speed;

        let current_agent = Agent {
            position: tf.translation.xy(),
            velocity: vel.0,
            radius: 5.0,
            avoidance_responsibility: 1.0,
        };

        let obstacles: Vec<Cow<'static, Obstacle>> = vec![];

        let avoidance_velocity = current_agent.compute_avoiding_velocity(
            &other_agents,
            &obstacles,
            preferred_velocity,
            agent_max_speed,
            time.delta_seconds(),
            &AvoidanceOptions {
                obstacle_margin: 0.1,
                time_horizon,
                obstacle_time_horizon,
            });

        gizmos.line_2d(tf.translation.xy(), tf.translation.xy() + avoidance_velocity, Color::linear_rgb(0.5, 0.5, 0.1));
        vel.0 = avoidance_velocity;
    }
}

























