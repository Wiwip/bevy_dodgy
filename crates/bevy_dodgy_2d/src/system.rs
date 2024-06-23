use std::borrow::Cow;
use std::time::{Duration, Instant};

use bevy::prelude::*;
use bevy_xpbd_2d::parry::query::PointQuery;
use bevy_xpbd_2d::parry::shape::{SharedShape, TypedShape};
use bevy_xpbd_2d::prelude::*;

use crate::agents::AgentGoal;
use crate::agents::{compute_avoiding_velocity, AgentInfo};
use crate::obstacles::{collider_as_obstacle, Obstacle};
use crate::{
    AgentData, AgentDataItem, AgentDataMut, AgentDataMutReadOnly, AgentDataMutReadOnlyItem,
    AvoidanceOptions,
};

pub fn rvo_avoidance(
    agents: Query<AgentData>,
    mut query: Query<(AgentDataMut, &Collider, &RigidBody)>,
    q_obstacles: Query<(&Transform, &Collider, &RigidBody)>,
    spatial: SpatialQuery,
    time: Res<Time>,
    mut gizmos: Gizmos,
) {
    if !(time.delta_seconds() > 0.0) {
        return;
    }

    for agent in agents.iter() {
        let (agent_data, _, _) = query.get(agent.entity).unwrap();

        let intersections = spatial.shape_intersections(
            &Collider::circle(
                agent.info.radius + (agent.options.time_horizon * agent.info.max_speed / 2.0),
            ), // Shape
            agent.transform.translation.xy(), // Shape position
            0.0,                              // Shape rotation
            SpatialQueryFilter::default().with_excluded_entities([agent.entity]), // Query filter
        );

        // Filter the intersected entities to return only dynamic agents
        let neighbours: Vec<AgentDataMutReadOnlyItem> = intersections
            .clone()
            .into_iter()
            .filter_map(|e| {
                if let Ok((data, _, body)) = query.get(e) {
                    if body.is_dynamic() {
                        return Some(data);
                    }
                };
                None
            })
            .collect();

        let preferred_velocity = (agent.goal.0 - agent.transform.translation.xy())
            .normalize_or_zero()
            * agent.info.max_speed;

        // Compute the obstacles
        let mut obstacles: Vec<Cow<'static, Obstacle>> = vec![];
        for intersect_entity in &intersections {
            let Ok((obstacle_tf, collider, body)) = q_obstacles.get(*intersect_entity) else {
                continue;
            };

            // Only static bodies are considered for obstacles
            match body {
                RigidBody::Dynamic => { /* Ignore rigid bodies*/ }
                RigidBody::Static => {
                    if let Some(obstacle) =
                        collider_as_obstacle(collider.shape_scaled().as_typed_shape(), obstacle_tf)
                    {
                        obstacles.push(Cow::Owned(obstacle));
                    }
                }
                RigidBody::Kinematic => { /* Consider kinematic bodies such as a player? */ }
            }
        }

        let avoidance_velocity = compute_avoiding_velocity(
            &agent_data,
            &neighbours,
            &obstacles,
            preferred_velocity,
            agent.info.max_speed,
            time.delta_seconds(),
            agent.options,
        );

        if let Ok((mut agent, _, _)) = query.get_mut(agent.entity) {
            agent.linvel.0 = avoidance_velocity;
        }

        gizmos.line(
            agent.transform.translation,
            agent.transform.translation + avoidance_velocity.extend(0.),
            Color::SEA_GREEN,
        );
    }
}

pub(crate) fn create_collider(
    mut commands: Commands,
    query: Query<(Entity, &AgentInfo, Option<&Collider>), Added<AgentInfo>>,
) {
    for (e, agent, option_collider) in query.iter() {
        if option_collider.is_none() {
            commands.entity(e).insert(Collider::circle(agent.radius));
        }
    }
}
