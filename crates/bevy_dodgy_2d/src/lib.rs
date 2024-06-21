#![doc = include_str!("../README.md")]

// The contents of this file were primarily ported from Agent.cc from RVO2 with
// significant alterations. As per the Apache-2.0 license, the original
// copyright notice has been included, excluding those notices that do not
// pertain to the derivate work:
//
// Agent.cc
// RVO2 Library
//
// SPDX-FileCopyrightText: 2008 University of North Carolina at Chapel Hill
//
// The authors may be contacted via:
//
// Jur van den Berg, Stephen J. Guy, Jamie Snape, Ming C. Lin, Dinesh Manocha
// Dept. of Computer Science
// 201 S. Columbia St.
// Frederick P. Brooks, Jr. Computer Science Bldg.
// Chapel Hill, N.C. 27599-3175
// United States of America
//
// <https://gamma.cs.unc.edu/RVO2/>
pub mod agents;
pub mod common;
mod linear_programming;
mod obstacles;
pub mod plugin;
pub mod system;
mod visibility_set;

//use obstacles::get_lines_for_agent_to_obstacle;

/// Parameters for computing the avoidance vector.
#[derive(Component, Clone, PartialEq, Debug)]
pub struct AvoidanceOptions {
    /// The distance that the agent must be from any obstacle. This is commonly
    /// the agent's radius to ensure the agent never intersects the obstacle (for
    /// example a wall). An alternative is to set this to a small value to treat
    /// obstacles as the edge of something (like a cliff).
    pub obstacle_margin: f32,
    /// How long in the future should collisions be considered between agents.
    pub time_horizon: f32,
    /// How long in the future should collisions be considered for obstacles.
    pub obstacle_time_horizon: f32,
}
/*
#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;

    mod get_line_for_neighbour_tests {
        use bevy::prelude::Vec2;
        use super::{Agent, Line};

        macro_rules! assert_line_eq {
            ($a: expr, $b: expr) => {{
                let a = $a;
                let b = $b;

                assert!(
                    a.point.distance_squared(b.point) < 1e-5,
                    "\n  left: {:?}\n right: {:?}",
                    a,
                    b
                );
                assert!(
                    a.direction.distance_squared(b.direction) < 1e-5,
                    "\n  left: {:?}\n right: {:?}",
                    a,
                    b
                );
            }};
        }

        #[test]
        fn velocity_projects_on_cutoff_circle() {
            let position = Vec2::new(1.0, 2.0);
            let radius = 2.0;

            let agent = Agent {
                position: Vec2::ZERO,
                velocity: Vec2::ZERO,
                radius: radius - 1.0,
                avoidance_responsibility: 1.0,
            };

            let neighbour = Agent {
                position: position,
                velocity: Vec2::ZERO,
                radius: 1.0,
                avoidance_responsibility: 1.0,
            };

            let actual_line = agent.get_line_for_neighbour(
                &neighbour, /* time_horizon= */ 1.0, /* time_step= */ 1.0,
            );
            // The agent's velocity projects directly onto the cut-off circle.
            assert_line_eq!(
                actual_line,
                Line {
                    point: position.normalize() * (position.length() - radius),
                    direction: position.perp().normalize(),
                }
            );
        }

        #[test]
        fn velocity_projects_to_shadow() {
            let mut agent = Agent {
                position: Vec2::ZERO,
                velocity: Vec2::new(1.0, 3.0),
                radius: 1.0,
                avoidance_responsibility: 1.0,
            };

            let neighbour = Agent {
                position: Vec2::new(2.0, 2.0),
                velocity: Vec2::ZERO,
                radius: 1.0,
                avoidance_responsibility: 1.0,
            };

            let inside_shadow_line = agent.get_line_for_neighbour(
                &neighbour, /* time_horizon= */ 1.0, /* time_step= */ 1.0,
            );
            assert_line_eq!(
                inside_shadow_line,
                Line {
                    point: Vec2::new(0.5, 3.0),
                    direction: Vec2::new(0.0, 1.0)
                }
            );

            agent.velocity = Vec2::new(10.0, -1.0);

            let outside_shadow_line = agent.get_line_for_neighbour(
                &neighbour, /* time_horizon= */ 1.0, /* time_step= */ 1.0,
            );
            assert_line_eq!(
                outside_shadow_line,
                Line {
                    point: Vec2::new(10.0, 0.0),
                    direction: Vec2::new(-1.0, 0.0)
                }
            );
        }

        #[test]
        fn collision_uses_time_step() {
            let agent = Agent {
                position: Vec2::ZERO,
                velocity: Vec2::new(0.0, 0.0),
                radius: 2.0,
                avoidance_responsibility: 1.0,
            };

            let neighbour = Agent {
                position: Vec2::new(2.0, 2.0),
                velocity: Vec2::ZERO,
                radius: 2.0,
                avoidance_responsibility: 1.0,
            };

            let collision_line = agent.get_line_for_neighbour(
                &neighbour, /* time_horizon= */ 1.0, /* time_step= */ 0.5,
            );
            assert_line_eq!(
                collision_line,
                Line {
                    point: (Vec2::ONE.normalize() * -8.0 + Vec2::new(4.0, 4.0)) * 0.5,
                    direction: Vec2::new(-1.0, 1.0).normalize(),
                }
            );
        }

        #[test]
        fn no_collision_uses_time_horizon() {
            let agent = Agent {
                position: Vec2::ZERO,
                velocity: Vec2::new(0.0, 0.0),
                radius: 1.0,
                avoidance_responsibility: 1.0,
            };

            let neighbour = Agent {
                position: Vec2::new(2.0, 2.0),
                velocity: Vec2::ZERO,
                radius: 1.0,
                avoidance_responsibility: 1.0,
            };

            let collision_line = agent.get_line_for_neighbour(
                &neighbour, /* time_horizon= */ 2.0, /* time_step= */ 0.5,
            );
            assert_line_eq!(
                collision_line,
                Line {
                    point: -Vec2::ONE.normalize() + Vec2::new(1.0, 1.0),
                    direction: Vec2::new(-1.0, 1.0).normalize(),
                }
            );
        }

        #[test]
        fn uses_avoidance_responsibility() {
            let agent = Agent {
                position: Vec2::ZERO,
                velocity: Vec2::new(1.5, 0.0),
                radius: 1.0,
                avoidance_responsibility: 1.0,
            };

            let neighbour = Agent {
                position: Vec2::new(4.0, 0.0),
                velocity: Vec2::ZERO,
                radius: 1.0,
                avoidance_responsibility: 3.0,
            };

            let actual_line = agent.get_line_for_neighbour(
                &neighbour, /* time_horizon= */ 2.0, /* time_step= */ 0.5,
            );
            assert_line_eq!(
                actual_line,
                Line {
                    point: Vec2::new(1.375, 0.0),
                    direction: Vec2::new(0.0, 1.0)
                }
            );
        }

        #[test]
        fn uses_avoidance_responsibility_only_when_inside_vo() {
            let agent = Agent {
                position: Vec2::ZERO,
                velocity: Vec2::new(0.5, 0.0),
                radius: 1.0,
                avoidance_responsibility: 1.0,
            };

            let neighbour = Agent {
                position: Vec2::new(4.0, 0.0),
                velocity: Vec2::ZERO,
                radius: 1.0,
                avoidance_responsibility: 3.0,
            };

            let actual_line = agent.get_line_for_neighbour(
                &neighbour, /* time_horizon= */ 2.0, /* time_step= */ 0.5,
            );
            assert_line_eq!(
                actual_line,
                Line {
                    point: Vec2::new(1.0, 0.0),
                    direction: Vec2::new(0.0, 1.0)
                }
            );
        }
    }
}
