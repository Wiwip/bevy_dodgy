use crate::agents::{AgentData, AgentDataMut, AgentInfo};
use crate::obstacles::{AsObstacle, TransformObstacle};
use avian3d::prelude::*;
use bevy::prelude::*;
use dodgy_2d::{Agent, Obstacle};
use std::borrow::Cow;

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

    for agent in agents.iter() {
        let (agent_data, _) = query.get(agent.entity).unwrap();
        let dodgy_agent = Agent::from(&agent_data);

        let intersections = spatial.shape_intersections(
            &Collider::sphere(
                agent.info.radius + agent.options.time_horizon * agent.info.max_speed,
            ),
            agent.transform.translation,
            Quat::IDENTITY,
            SpatialQueryFilter::default().with_excluded_entities([agent.entity]), // Exclude self
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

        // If the agent has no goal, ignore.
        let Some(agent_goal) = agent.goal else {
            continue;
        };

        let preferred_velocity = (agent_goal.0.xz() - agent.transform.translation.xz())
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
                    if let Some(mut obstacle) = collider.obstacle() {
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
            agent.linvel.0 = Vec3::new(avoidance_velocity.x, 0.0, avoidance_velocity.y)
        }
    }
}

pub(crate) fn on_add_create_collider(
    mut commands: Commands,
    query: Query<(Entity, &AgentInfo, Option<&Collider>), Added<AgentInfo>>,
) {
    for (e, agent, option_collider) in query.iter() {
        if option_collider.is_none() {
            commands
                .entity(e)
                .insert(Collider::capsule(agent.radius, 18.0));
        }
    }
}
