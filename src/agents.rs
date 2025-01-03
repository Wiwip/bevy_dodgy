use avian2d::prelude::LinearVelocity;
use bevy::ecs::query::QueryData;
use bevy::math::Vec3Swizzles;
use bevy::prelude::{Component, Deref, DerefMut, Entity, Transform, Vec2};
use dodgy_2d::{Agent, AvoidanceOptions};

/// A QueryData used by the rvo_avoidance system to simplify queries.
/// This version excludes LinearVelocity due to access restrictions
#[derive(QueryData)]
#[query_data(derive(Debug))]
pub struct AgentQueryData {
    pub entity: Entity,
    pub info: &'static AgentInfo,
    pub transform: &'static Transform,
    pub goal: Option<&'static AgentGoal>,
    pub options: &'static AvoidanceOptionsComponent,
}

///
#[derive(QueryData)]
#[query_data(mutable, derive(Debug))]
pub struct AgentQueryDataMut {
    pub entity: Entity,
    pub info: &'static AgentInfo,
    pub transform: &'static Transform,
    pub linvel: &'static mut LinearVelocity,
    pub goal: Option<&'static AgentGoal>,
    pub options: &'static AvoidanceOptionsComponent,
}

#[derive(Component, Debug)]
pub struct AgentGoal{
    pub dest: Vec2,
    pub tolerance: f32,
}

/// Represents an agent in the simulation
#[derive(Component, Clone, PartialEq, Debug)]
pub struct AgentInfo {
    /// The radius of the agent. Agents will use this to avoid bumping into each
    /// other.
    pub radius: f32,

    /// The amount of responsibility an agent has to avoid other agents. The
    /// amount of avoidance between two agents is then dependent on the ratio of
    /// the responsibility between the agents. Note this does not affect
    /// avoidance of obstacles.
    pub avoidance_responsibility: f32,

    pub max_speed: f32,
}

#[derive(Component, Clone, PartialEq, Debug, Deref, DerefMut)]
pub struct AvoidanceOptionsComponent(pub AvoidanceOptions);

impl AvoidanceOptionsComponent {
    pub fn new(
        obstacle_margin: f32,
        time_horizon: f32,
        obstacle_time_horizon: f32,
    ) -> AvoidanceOptionsComponent {
        AvoidanceOptionsComponent(AvoidanceOptions {
            obstacle_margin,
            time_horizon,
            obstacle_time_horizon,
        })
    }
}

impl From<&AgentQueryDataMutReadOnlyItem<'_>> for Agent {
    fn from(value: &AgentQueryDataMutReadOnlyItem) -> Self {
        Self {
            position: value.transform.translation.xy(),
            velocity: value.linvel.0,
            radius: value.info.radius,
            avoidance_responsibility: value.info.avoidance_responsibility,
        }
    }
}
