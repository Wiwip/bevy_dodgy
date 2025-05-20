use avian3d::prelude::*;
use bevy::ecs::query::QueryData;
use bevy::prelude::*;
use dodgy_2d::{Agent, AvoidanceOptions};

/// A QueryData used by the rvo_avoidance system to simplify queries.
/// This version excludes LinearVelocity due to access restrictions
#[derive(QueryData)]
#[query_data(derive(Debug))]
pub struct AgentQueryData {
    pub entity: Entity,
    pub info: &'static AgentInfo,
    pub transform: &'static GlobalTransform,
    pub goal: Option<&'static AgentGoal>,
    pub options: &'static AvoidanceOptionsComponent,
}

///
#[derive(QueryData)]
#[query_data(mutable, derive(Debug))]
pub struct AgentQueryDataMut {
    pub entity: Entity,
    pub info: &'static AgentInfo,
    pub transform: &'static GlobalTransform,
    pub linvel: &'static mut LinearVelocity,
    pub goal: Option<&'static AgentGoal>,
    pub options: &'static AvoidanceOptionsComponent,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct AgentGoal {
    pub dest: Vec3,
    pub tolerance: f32,
}

/// Represents an agent in the simulation
#[derive(Component, Clone, PartialEq, Debug)]
#[require(AvoidanceOptionsComponent)]
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
    pub fn new(time_horizon: f32) -> AvoidanceOptionsComponent {
        AvoidanceOptionsComponent(AvoidanceOptions {
            obstacle_margin: 1.0,
            time_horizon,
            obstacle_time_horizon: 2.0,
        })
    }
}

impl Default for AvoidanceOptionsComponent {
    fn default() -> Self {
        Self::new(0.0)
    }
}

impl From<&AgentQueryDataMutReadOnlyItem<'_>> for Agent {
    fn from(value: &AgentQueryDataMutReadOnlyItem) -> Self {
        Self {
            position: value.transform.translation().xz(),
            velocity: value.linvel.0.xz(),
            radius: value.info.radius,
            avoidance_responsibility: value.info.avoidance_responsibility,
        }
    }
}
