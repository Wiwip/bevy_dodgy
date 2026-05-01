#[cfg(feature = "3d")]
use avian3d::prelude::*;
#[cfg(feature = "2d")]
use avian2d::prelude::*;

use bevy::ecs::query::QueryData;
use bevy::ecs::system::lifetimeless::{Read, Write};
use bevy::prelude::*;
use dodgy_2d::{Agent, AvoidanceOptions};

/// A QueryData used by the rvo_avoidance system to simplify queries.
/// This version excludes LinearVelocity due to access restrictions
#[derive(QueryData)]
#[query_data(derive(Debug))]
pub struct AgentQueryData {
    pub entity: Entity,
    pub info: Read<AgentInfo>,
    pub transform: Read<GlobalTransform>,
    pub linvel: Read<LinearVelocity>,
    pub pref_vel: Read<PreferredVelocity>,
    pub options: Read<AvoidanceOptionsComponent>,
}

///
#[derive(QueryData)]
#[query_data(mutable, derive(Debug))]
pub struct AgentQueryDataMut {
    pub entity: Entity,
    pub info: Read<AgentInfo>,
    pub transform: Read<GlobalTransform>,
    pub linvel: Read<LinearVelocity>,
    pub pref_vel: Read<PreferredVelocity>,
    pub ideal_vel: Write<IdealVelocity>,
    pub options: Read<AvoidanceOptionsComponent>,
}

#[cfg(feature = "3d")]
#[derive(Component, Default, Clone, Copy, Debug, Deref, DerefMut)]
pub struct PreferredVelocity(pub Vec3);

#[cfg(feature = "3d")]
#[derive(Component, Default, Clone, Copy, Debug, Deref, DerefMut)]
pub struct IdealVelocity(pub Vec3);

#[cfg(feature = "2d")]
#[derive(Component, Clone, Copy, Debug)]
pub struct AgentGoal {
    pub dest: Vec2,
    pub tolerance: f32,
}

/// Represents an agent in the simulation
#[derive(Component, Clone, PartialEq, Debug)]
#[require(AvoidanceOptionsComponent, PreferredVelocity, IdealVelocity)]
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
#[cfg(feature = "2d")]
#[derive(Component, Default, Clone, Copy, Debug)]
pub struct AgentIdealVelocity(pub Vec2);

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

impl From<&AgentQueryDataMutReadOnlyItem<'_, '_>> for Agent {
    fn from(value: &AgentQueryDataMutReadOnlyItem) -> Self {
        Self {
            #[cfg(feature = "3d")]
            position: value.transform.translation().xz(),
            #[cfg(feature = "2d")]
            position: value.transform.translation().xy(),
            #[cfg(feature = "3d")]
            velocity: value.linvel.0.xz(),
            #[cfg(feature = "2d")]
            velocity: value.linvel.0.xy(),
            radius: value.info.radius,
            avoidance_responsibility: value.info.avoidance_responsibility,
        }
    }
}

impl From<&AgentQueryDataMutItem<'_, '_>> for Agent {
    fn from(value: &AgentQueryDataMutItem) -> Self {
        Self {
            #[cfg(feature = "3d")]
            position: value.transform.translation().xz(),
            #[cfg(feature = "2d")]
            position: value.transform.translation().xy(),
            #[cfg(feature = "3d")]
            velocity: value.linvel.0.xz(),
            #[cfg(feature = "2d")]
            velocity: value.linvel.0.xy(),
            radius: value.info.radius,
            avoidance_responsibility: value.info.avoidance_responsibility,
        }
    }
}

impl From<&AgentQueryDataItem<'_, '_>> for Agent {
    fn from(value: &AgentQueryDataItem) -> Self {
        Self {
            #[cfg(feature = "3d")]
            position: value.transform.translation().xz(),
            #[cfg(feature = "2d")]
            position: value.transform.translation().xy(),
            #[cfg(feature = "3d")]
            velocity: value.linvel.0.xz(),
            #[cfg(feature = "2d")]
            velocity: value.linvel.0.xy(),
            radius: value.info.radius,
            avoidance_responsibility: value.info.avoidance_responsibility,
        }
    }
}
