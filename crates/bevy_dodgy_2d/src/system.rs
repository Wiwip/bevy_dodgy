use std::borrow::Cow;
use std::time::{Duration, Instant};

use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

use crate::agents::{Agent, compute_avoiding_velocity};
use crate::agents::AgentGoal;
use crate::{AvoidanceOptions, Obstacle};

pub fn rvo_avoidance(
    agents: Query<(Entity, &Agent, &Transform, &AgentGoal, &AvoidanceOptions)>,
    mut query: Query<(
        Entity,
        &Agent,
        &Transform,
        &mut LinearVelocity,
        &AgentGoal,
        &AvoidanceOptions,
    )>,
    spatial: SpatialQuery,
    time: Res<Time>,
    mut gizmos: Gizmos,
) {
    if !(time.delta_seconds() > 0.0) {
        return;
    }

    for (entity, agent, tf, goal, options) in agents.iter() {
        let intersections = spatial.shape_intersections(
            &Collider::circle(agent.radius + (options.time_horizon * agent.max_speed / 2.0)), // Shape
            tf.translation.xy(),                 // Shape position
            0.0,                    // Shape rotation
            SpatialQueryFilter::default().with_excluded_entities([entity]), // Query filter
        );

        let preferred_velocity =
            (goal.0 - tf.translation.xy()).normalize_or_zero() * agent.max_speed;

        let obstacles: Vec<Cow<'static, Obstacle>> = vec![];

        let mut lens =
            query.transmute_lens::<(&Agent, &Transform, &LinearVelocity, &AvoidanceOptions)>();

        let avoidance_velocity = compute_avoiding_velocity(
            entity,
            intersections,
            &mut lens,
            &obstacles,
            preferred_velocity,
            agent.max_speed,
            time.delta_seconds(),
            options,
        );

        let (_, _, _, mut vel, _, _) = query.get_mut(entity).unwrap();
        vel.0 = avoidance_velocity;

        gizmos.line(
            tf.translation,
            tf.translation + avoidance_velocity.extend(0.),
            Color::GREEN,
        );
    }
}

pub(crate) fn create_collider(
    mut commands: Commands,
    query: Query<(Entity, &Agent, Option<&Collider>), Added<Agent>>,
) {
    for (e, agent, option_collider) in query.iter() {
        if option_collider.is_none() {
            commands.entity(e).insert(Collider::circle(agent.radius));
        }
    }
}
