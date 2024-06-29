use std::borrow::Cow;
use std::time::Instant;
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

use crate::agents::{AgentInfo, AgentData, AgentDataMut, AgentDataMutReadOnlyItem, Agent};
use crate::obstacles::{Obstacle};


pub fn rvo_avoidance(
    agents: Query<AgentData>,
    mut query: Query<(AgentDataMut, &RigidBody)>,
    q_obstacles: Query<(&Transform, &Collider, &RigidBody)>,
    spatial: SpatialQuery,
    time: Res<Time>,
) {
    if !(time.delta_seconds() > 0.0) {
        return;
    }

    let now = Instant::now();

    for agent in agents.iter() {
        let (agent_data, _) = query.get(agent.entity).unwrap();
        let dodgy_agent = Agent::from(&agent_data);

        let intersections = spatial.shape_intersections(
            &Collider::circle(
                agent.info.radius + (agent.options.time_horizon * agent.info.max_speed / 2.0),
            ), // Shape
            agent.transform.translation.xy(),
            0.0,
            SpatialQueryFilter::default().with_excluded_entities([agent.entity]), // Exclude self
        );

        // Filter the intersected entities to return only dynamic agents
        let neighbours: Vec<Agent> = intersections
            .clone()
            .into_iter()
            .filter_map(|e| {
                if let Ok((data, body)) = query.get(e) {
                    if body.is_dynamic() {
                        return Some(Agent::from(&data));
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
                RigidBody::Dynamic => { /* Ignore rigid bodies. */ }
                RigidBody::Static => {
                    if let Ok(mut obstacle) = Obstacle::try_from(collider) {
                        obstacle.transform(obstacle_tf);
                        obstacles.push(Cow::Owned(obstacle));
                    }
                }
                RigidBody::Kinematic => { /* Ignore kinematic bodies. */ }
            }
        }

        let avoidance_velocity = dodgy_agent.compute_avoiding_velocity(
            &neighbours,
            &obstacles,
            preferred_velocity,
            agent.info.max_speed,
            time.delta_seconds(),
            agent.options,
        );

        if let Ok((mut agent, _)) = query.get_mut(agent.entity) {
            agent.linvel.0 = avoidance_velocity;
        }
    }

    info!("Elapsed: {:?}", now.elapsed());
}

pub(crate) fn on_add_create_collider(
    mut commands: Commands,
    query: Query<(Entity, &AgentInfo, Option<&Collider>), Added<AgentInfo>>,
) {
    for (e, agent, option_collider) in query.iter() {
        if option_collider.is_none() {
            commands.entity(e).insert(Collider::circle(agent.radius));
        }
    }
}
