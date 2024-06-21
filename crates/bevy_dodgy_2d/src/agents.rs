use crate::common::determinant;
use crate::linear_programming::{solve_linear_program, Line};
use crate::{AvoidanceOptions, Obstacle};
use bevy::ecs::system::QueryLens;
use bevy::prelude::*;
use bevy_xpbd_2d::components::LinearVelocity;
use std::borrow::Cow;
//use crate::obstacles::get_lines_for_agent_to_obstacle;

#[derive(Component)]
pub struct AgentGoal(pub Vec2);
/// Represents an agent in the simulation
#[derive(Component, Clone, PartialEq, Debug)]
pub struct Agent {
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

pub fn compute_avoiding_velocity(
    agent: Entity,
    mut neighbours: Vec<Entity>,
    query: &mut QueryLens<(&Agent, &Transform, &LinearVelocity, &AvoidanceOptions)>,
    obstacles: &[Cow<'_, Obstacle>],
    preferred_velocity: Vec2,
    max_speed: f32,
    time_step: f32,
    avoidance_options: &AvoidanceOptions,
) -> Vec2 {
    assert!(
        time_step > 0.0,
        "time_step must be positive, was {}",
        time_step
    );

    let lines = obstacles
        .iter()
        .flat_map(|o| {
            /*self.get_lines_for_agent_to_obstacle(
                self,
                o,
                avoidance_options.obstacle_margin,
                avoidance_options.obstacle_time_horizon,
            )*/
            None
        })
        .chain(neighbours.iter().map(|&neighbour| {
            get_line_for_neighbour(
                agent,
                neighbour,
                query,
                avoidance_options.time_horizon,
                time_step,
            )
        }))
        .collect::<Vec<Line>>();

    // Since each neighbour generates one line, the number of obstacle lines is
    // just the other lines.
    let obstacle_line_count = lines.len() - neighbours.len();

    solve_linear_program(&lines, obstacle_line_count, max_speed, preferred_velocity)
}

fn get_line_for_neighbour(
    agent: Entity,
    neighbour: Entity,
    query_lens: &mut QueryLens<(&Agent, &Transform, &LinearVelocity, &AvoidanceOptions)>,
    time_horizon: f32,
    time_step: f32,
) -> Line {
    // There are two parts to the velocity obstacle induced by `neighbour`.
    // 1) The cut-off circle. This is where the agent collides with `neighbour`
    // after some time (either `time_horizon` or `time_step`).
    // 2) The cut-off shadow. Any velocity that is just scaled up from a
    // velocity in the cut-off circle will also hit `neighbour`.
    //
    // If the relative position and velocity is used, the cut-off for the shadow
    // will be directed toward the origin.

    let binding = query_lens.query();
    let (agent_info, agent_position, agent_velocity, agent_option) = binding.get(agent).unwrap();
    let (nb_info, nb_transform, nb_velocity, nb_option) = binding.get(neighbour).unwrap();

    let relative_neighbour_position =
        nb_transform.translation.xy() - agent_position.translation.xy();
    let relative_agent_velocity = agent_velocity.0 - nb_velocity.0;

    let distance_squared = relative_neighbour_position.length_squared();

    let sum_radius = agent_info.radius + nb_info.radius;
    let sum_radius_squared = sum_radius * sum_radius;

    let vo_normal;
    let relative_velocity_projected_to_vo;
    let inside_vo;

    // Find out if the agent is inside the cut-off circle. Note: since both the
    // distance to the cut-off circle and the radius of the cut-off circle is
    // scaled by `time_horizon` (or `time_step` depending on the situation),
    // factoring out those terms and cancelling yields this simpler expression.
    if distance_squared > sum_radius_squared {
        // No collision, so either project on to the cut-off circle, or the
        // cut-off shadow.
        //
        // The edges of the cut-off shadow lies along the tangents of the circle
        // that intersects the origin (since the tangents are the lines that just
        // graze the cut-off circle and so these line divide the "shadowed"
        // velocities from the "unshadowed" velocities).
        //
        // Since the shadows are caused by the tangent lines, velocities should be
        // projected to the cut-off circle when they are on one-side of the
        // tangent points, and should be projected to the shadow when on the
        // other-side of the tangent points.

        let cutoff_circle_center = relative_neighbour_position / time_horizon;
        let cutoff_circle_center_to_relative_velocity =
            relative_agent_velocity - cutoff_circle_center;
        let cutoff_circle_center_to_relative_velocity_length_squared =
            cutoff_circle_center_to_relative_velocity.length_squared();

        let dot = cutoff_circle_center_to_relative_velocity.dot(relative_neighbour_position);

        // TODO: Figure out why this works. Something to do with circle tangents,
        // right triangles with those tangents, and the angle between
        // `cutoff_circle_center_to_relative_velocity` and
        // `relative_neighbour_position`.
        if dot < 0.0
            && dot * dot
            > sum_radius_squared * cutoff_circle_center_to_relative_velocity_length_squared
        {
            // The relative velocity has not gone past the cut-off circle tangent
            // points yet, so project onto the cut-off circle.

            let cutoff_circle_radius = sum_radius / time_horizon;

            vo_normal = cutoff_circle_center_to_relative_velocity.normalize_or_zero();
            relative_velocity_projected_to_vo =
                vo_normal * cutoff_circle_radius + cutoff_circle_center;
            inside_vo = cutoff_circle_center_to_relative_velocity_length_squared
                < cutoff_circle_radius * cutoff_circle_radius;
        } else {
            // The relative velocity is past the cut-off circle tangent points, so
            // project onto the shadow.

            let tangent_triangle_leg = (distance_squared - sum_radius_squared).sqrt();

            // Consider the right-triangle describing the tangent point (one side
            // has length `sum_radius`, hypotenuse has side length
            // `cutoff_circle_center_to_relative_velocity_length_squared`). The last
            // side will have length `tangent_triangle_leg`. A similar triangle can
            // then be created using the same triangle leg lengths, but oriented
            // such that the hypotenuse is in the direction of the tangent and
            // composed of directions `relative_position` and the perpendicular of
            // `relative_position`.

            // Determine whether the relative velocity is nearer the left or right
            // side of the shadow.
            let tangent_side = determinant(
                relative_neighbour_position,
                cutoff_circle_center_to_relative_velocity,
            )
                .signum();

            // Compute the shadow direction using the tangent triangle legs, and
            // make sure to use the correct orientation of that direction (the
            // correct side of the line is invalid).
            let shadow_direction =
                relative_neighbour_position * tangent_triangle_leg * tangent_side
                    + relative_neighbour_position.perp() * sum_radius;

            // Renormalize the shadow direction.
            let shadow_direction = shadow_direction / distance_squared;

            vo_normal = shadow_direction.perp();
            // Project onto the shadow.
            relative_velocity_projected_to_vo =
                relative_agent_velocity.project_onto_normalized(shadow_direction);
            // The velocity is inside the VO if it is to the left of the left
            // shadow, or the right of the right shadow.
            inside_vo = determinant(relative_agent_velocity, shadow_direction) >= 0.0;
        }
    } else {
        // Collision. Project on cut-off circle at time `time_step`.

        // Find the velocity such that after `time_step` the agent would be at the
        // neighbours position.
        let cutoff_circle_center = relative_neighbour_position / time_step;
        let cutoff_circle_radius = sum_radius / time_step;

        // The direction of the velocity from `cutoff_circle_center` is therefore
        // the normal to the velocity obstacle.
        vo_normal = (relative_agent_velocity - cutoff_circle_center).normalize_or_zero();
        // Get the point on the cut-off circle in that direction (which is the
        // agent's velocity projected to the circle).
        relative_velocity_projected_to_vo = vo_normal * cutoff_circle_radius + cutoff_circle_center;
        inside_vo = true;
    }

    // As in the paper, `u` is the vector from the relative velocity to the
    // nearest point outside the velocity obstacle.
    let u = relative_velocity_projected_to_vo - relative_agent_velocity;

    let responsibility = if inside_vo {
        agent_info.avoidance_responsibility
            / (agent_info.avoidance_responsibility + nb_info.avoidance_responsibility)
    } else {
        1.0
    };

    Line {
        point: agent_velocity.0 + u * responsibility,
        direction: -vo_normal.perp(),
    }
}
