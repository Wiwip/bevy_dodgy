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
    mut query: Query<AgentQueryDataMut, With<RigidBody>>,
    q_obstacles: Query<(&GlobalTransform, &Collider, &RigidBody), Without<AgentInfo>>,
    spatial: Option<SpatialQuery>,
    time: Res<Time>,
) {
    if !(time.delta_secs() > 0.0) {
        return;
    }

    let Some(spatial) = spatial else {
        error!("Spatial query does not exist.");
        return;
    };

    query.par_iter_mut().for_each(|mut agent_data| {
        let dodgy_agent = Agent::from(&agent_data);

        // If the agent has no goal, ignore.
        let Some(agent_goal) = agent_data.goal else {
            return;
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
            return;
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

        let mut neighbours: Vec<Cow<'static, Agent>> = Vec::with_capacity(intersections.len());
        let mut obstacles: Vec<Cow<'static, Obstacle>> = Vec::with_capacity(intersections.len());

        for &entity in &intersections {
            if let Ok(data) = agents.get(entity) {
                neighbours.push(Cow::Owned(Agent::from(&data)));
            } else if let Ok((obstacle_tf, collider, body)) = q_obstacles.get(entity) {
                match body {
                    RigidBody::Static => {
                        if let Some(mut obstacle) = collider.to_obstacle() {
                            obstacle.transform_points(obstacle_tf);
                            obstacles.push(Cow::Owned(obstacle));
                        }
                    }
                    RigidBody::Dynamic | RigidBody::Kinematic => {
                        // Keep the warnings from your second snippet if they are useful for debugging
                        warn_once!("Non-static body detected as obstacle; ignoring.");
                    }
                }
            }
        }

        // Check whether the agent is within the goal tolerance.
        #[cfg(feature = "3d")]
        let distance = (agent_goal.dest.xz() - agent_data.transform.translation().xz()).length();

        #[cfg(feature = "2d")]
        let distance = (agent_goal.dest - agent_data.transform.translation().xy()).length();

        #[cfg(feature = "2d")]
        if distance <= agent_goal.tolerance {
            if let Ok(mut agent_data_mut) = query.get_mut(agent_data.entity) {
                agent_data_mut.desired_vel.0 = Vec2::ZERO;
            }
            continue;
        }
        #[cfg(feature = "3d")]
        if distance <= agent_goal.tolerance {
            agent_data.desired_vel.0 = Vec3::ZERO;
            return;
        }

        let ideal_velocity = dodgy_agent.compute_avoiding_velocity(
            &neighbours,
            &obstacles,
            preferred_velocity,
            agent_data.info.max_speed,
            time.delta_secs(),
            agent_data.options,
        );

        agent_data.desired_vel.0 = Vec3::new(ideal_velocity.x, 0.0, ideal_velocity.y);

        #[cfg(feature = "2d")]
        if let Ok(mut agent_data_mut) = query.get_mut(agent_data.entity) {
            agent_data_mut.desired_vel.0 = Vec2::new(ideal_velocity.x, ideal_velocity.y);
        }
    });
}
