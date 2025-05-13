use crate::agents::{AgentInfo, AgentQueryData, AgentQueryDataMut};
use crate::obstacles::{AsObstacle, TransformObstacle};
use avian2d::math::RecipOrZero;
use avian2d::prelude::*;
use bevy::prelude::*;
use dodgy_2d::{Agent, Obstacle};
use std::borrow::Cow;
use std::time::Instant;

pub fn rvo_avoidance(
    agents: Query<AgentQueryData>,
    mut query: Query<(AgentQueryDataMut, &RigidBody)>,
    q_obstacles: Query<(&Transform, &Collider, &RigidBody)>,
    spatial: SpatialQuery,
    time: Res<Time>,
) {
    if !(time.delta_secs() > 0.0) {
        return;
    }

    //let instant = Instant::now();

    for agent_data in agents.iter() {
        let Ok((agent_data, _)) = query.get(agent_data.entity) else {
            continue;
        };
        let dodgy_agent = Agent::from(&agent_data);

        // If the agent has no goal, ignore.
        let Some(agent_goal) = agent_data.goal else {
            continue;
        };
        let preferred_velocity = (agent_goal.dest - agent_data.transform.translation.xy())
            .normalize_or_zero()
            * agent_data.info.max_speed;

        if preferred_velocity.length() < 1e-6 {
            warn!("Agent {} has no preferred velocity", agent_data.entity);
            continue;
        }

        let intersections = spatial.shape_intersections(
            &Collider::circle(
                agent_data.info.radius
                    + agent_data.options.time_horizon * agent_data.info.max_speed,
            ),
            agent_data.transform.translation.xy(),
            0.0,
            &SpatialQueryFilter::default().with_excluded_entities([agent_data.entity]), // Exclude self
        );

        // Filter the intersected entities to return only dynamic agents
        let neighbours: Vec<Cow<'static, Agent>> = intersections
            .clone()
            .into_iter()
            .filter_map(|e| {
                if let Ok((data, body)) = query.get(e) {
                    if body.is_dynamic() {
                        return Some(Cow::Owned(Agent::from(&data)));
                    }
                };
                None
            })
            .collect();

        // Check whether the agent is within the goal tolerance
        let distance = (agent_goal.dest - agent_data.transform.translation.xy()).length();
        if distance <= agent_goal.tolerance {
            if let Ok((mut agent_data_mut, _)) = query.get_mut(agent_data.entity) {
                agent_data_mut.linvel.0 = Vec2::ZERO;
            }
            continue;
        }

        // Compute the obstacles
        let mut obstacles: Vec<Cow<'static, Obstacle>> = vec![];
        for intersect_entity in &intersections {
            let Ok((obstacle_tf, collider, body)) = q_obstacles.get(*intersect_entity) else {
                continue;
            };

            // Only static bodies are considered for obstacles.
            match body {
                RigidBody::Dynamic => { /* Ignore rigid bodies. */ }
                RigidBody::Static => {
                    if let Some(mut obstacle) = collider.to_obstacle() {
                        obstacle.transform_points(obstacle_tf);
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
            agent_data.info.max_speed,
            time.delta_secs(),
            agent_data.options,
        );

        if let Ok((mut agent_data_mut, _)) = query.get_mut(agent_data.entity) {
            agent_data_mut.linvel.0 = avoidance_velocity;
        }
    }

    // println!("RVO avoidance took {:?}", instant.elapsed());
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
