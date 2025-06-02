use crate::agents::{AgentInfo, AgentQueryData, AgentQueryDataMut};
use crate::obstacles::{AsObstacle, TransformObstacle};
use bevy::prelude::*;
use dodgy_2d::{Agent, Obstacle};
use std::borrow::Cow;

#[cfg(feature = "2d")]
use avian2d::prelude::*;
#[cfg(feature = "3d")]
use avian3d::prelude::*;

pub fn rvo_avoidance(
    agents: Query<AgentQueryData>,
    mut query: Query<(AgentQueryDataMut, &RigidBody)>,
    q_obstacles: Query<(&GlobalTransform, &Collider, &RigidBody), Without<AgentInfo>>,
    spatial: SpatialQuery,
    time: Res<Time>,
) {
    if !(time.delta_secs() > 0.0) {
        return;
    }

    for agent_data in agents.iter() {
        let Ok((agent_data, _)) = query.get(agent_data.entity) else {
            continue;
        };
        let dodgy_agent = Agent::from(&agent_data);

        // If the agent has no goal, ignore.
        let Some(agent_goal) = agent_data.goal else {
            continue;
        };
        #[cfg(feature = "3d")]
        let preferred_velocity = (agent_goal.dest - agent_data.transform.translation())
            .xz()
            .normalize_or_zero()
            * agent_data.info.max_speed;

        #[cfg(feature = "2d")]
        let preferred_velocity = (agent_goal.dest - agent_data.transform.translation().xy())
            .normalize_or_zero()
            * agent_data.info.max_speed;

        if preferred_velocity.length() < 1e-6 {
            continue;
        }

        #[cfg(feature = "3d")]
        let intersections = spatial.shape_intersections(
            &Collider::sphere(
                agent_data.info.radius
                    + agent_data.options.time_horizon * agent_data.info.max_speed,
            ),
            agent_data.transform.translation(),
            Quat::IDENTITY,
            &SpatialQueryFilter::default().with_excluded_entities([agent_data.entity]), // Exclude self
        );

        #[cfg(feature = "2d")]
        let intersections = spatial.shape_intersections(
            &Collider::circle(
                agent_data.info.radius
                    + agent_data.options.time_horizon * agent_data.info.max_speed,
            ),
            agent_data.transform.translation().xy(),
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

        // Compute the obstacles
        let mut obstacles: Vec<Cow<'static, Obstacle>> = vec![];
        for intersect_entity in &intersections {
            let Ok((obstacle_tf, collider, body)) = q_obstacles.get(*intersect_entity) else {
                continue;
            };

            // Only static bodies are considered for obstacles.
            match body {
                RigidBody::Dynamic => {
                    /* Ignore rigid bodies. */
                    warn_once!("Dynamic bodies obstacles are not supported.");
                }
                RigidBody::Static => {
                    if let Some(mut obstacle) = collider.to_obstacle() {
                        obstacle.transform_points(obstacle_tf);
                        obstacles.push(Cow::Owned(obstacle));
                    }
                }
                RigidBody::Kinematic => {
                    /* Ignore kinematic bodies. */
                    warn_once!("Kinematic bodies obstacles are not supported.");
                }
            }
        }

        // Check whether the agent is within the goal tolerance
        #[cfg(feature = "3d")]
        let distance = (agent_goal.dest - agent_data.transform.translation()).length();

        #[cfg(feature = "2d")]
        let distance = (agent_goal.dest - agent_data.transform.translation().xy()).length();

        #[cfg(feature = "2d")]
        if distance <= agent_goal.tolerance {
            if let Ok((mut agent_data_mut, _)) = query.get_mut(agent_data.entity) {
                agent_data_mut.desired_vel.0 = Vec2::ZERO;
            }
            continue;
        }
        #[cfg(feature = "3d")]
        if distance <= agent_goal.tolerance {
            if let Ok((mut agent_data_mut, _)) = query.get_mut(agent_data.entity) {
                agent_data_mut.desired_vel.0 = Vec3::ZERO;
            }
            continue;
        }

        let ideal_velocity = dodgy_agent.compute_avoiding_velocity(
            &neighbours,
            &obstacles,
            preferred_velocity,
            agent_data.info.max_speed,
            time.delta_secs(),
            agent_data.options,
        );

        #[cfg(feature = "3d")]
        if let Ok((mut agent_data_mut, _)) = query.get_mut(agent_data.entity) {
            agent_data_mut.desired_vel.0 =
                Vec3::new(ideal_velocity.x, 0.0, ideal_velocity.y);
        }
        #[cfg(feature = "2d")]
        if let Ok((mut agent_data_mut, _)) = query.get_mut(agent_data.entity) {
            agent_data_mut.desired_vel.0 = Vec2::new(ideal_velocity.x, ideal_velocity.y);
        }
    }
}
