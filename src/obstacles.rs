use crate::geometry::{point_on_circle, rect_inner};
use avian3d::parry::shape::TypedShape;
use avian3d::prelude::*;
use bevy::prelude::*;
use dodgy_2d::Obstacle;

pub trait AsObstacle {
    fn to_obstacle(&self) -> Option<Obstacle>;
}

impl AsObstacle for Collider {
    fn to_obstacle(&self) -> Option<Obstacle> {
        let shape = self.shape_scaled().as_typed_shape();
        match shape {
            TypedShape::Cuboid(cuboid) => {
                let [tr, tl, bl, br] = rect_inner(Vec3::from(cuboid.half_extents * 2.0));

                Some(Obstacle::Closed {
                    vertices: vec![tr, tl, bl, br],
                })
            }

            TypedShape::Triangle(tri) => Some(Obstacle::Closed {
                vertices: vec![tri.a.xz().into(), tri.b.xz().into(), tri.c.xz().into()],
            }),

            TypedShape::Cylinder(cylinder) => {
                let num_agents = 12; // TODO configuration resource
                Some(Obstacle::Closed {
                    vertices: (0..num_agents)
                        .map(|i| {
                            let theta =
                                2.0 * std::f32::consts::PI * (i as f32) / (num_agents as f32);
                            point_on_circle((0., 0.), cylinder.radius, theta)
                        })
                        .collect(),
                })
            }

            TypedShape::Capsule(capsule) => {
                let num_agents = 12; // TODO configuration resource
                Some(Obstacle::Closed {
                    vertices: (0..num_agents)
                        .map(|i| {
                            let theta =
                                2.0 * std::f32::consts::PI * (i as f32) / (num_agents as f32);
                            point_on_circle((0., 0.), capsule.radius, theta)
                        })
                        .collect(),
                })
            }

            TypedShape::Ball(ball) => {
                let num_agents = 12; // TODO configuration resource
                Some(Obstacle::Closed {
                    vertices: (0..num_agents)
                        .map(|i| {
                            let theta =
                                2.0 * std::f32::consts::PI * (i as f32) / (num_agents as f32);
                            point_on_circle((0., 0.), ball.radius, theta)
                        })
                        .collect(),
                })
            }

            TypedShape::Cone(cone) => {
                let num_agents = 12; // TODO configuration resource
                Some(Obstacle::Closed {
                    vertices: (0..num_agents)
                        .map(|i| {
                            let theta =
                                2.0 * std::f32::consts::PI * (i as f32) / (num_agents as f32);
                            point_on_circle((0., 0.), cone.radius, theta)
                        })
                        .collect(),
                })
            }

            _ => {
                warn_once!("The shape isn't supported.");
                None
            }
        }
    }
}

pub trait TransformObstacle {
    fn transform_points(&mut self, tf: &Transform);
}

impl TransformObstacle for Obstacle {
    fn transform_points(&mut self, tf: &Transform) {
        match self {
            Obstacle::Closed { vertices } => {
                vertices.iter_mut().for_each(|vec2| {
                    let pt = Vec3::new(vec2.x, 0., vec2.y);
                    *vec2 = tf.transform_point(pt).xz();
                });
            }
            Obstacle::Open { vertices } => {
                vertices.iter_mut().for_each(|vec2| {
                    let pt = Vec3::new(vec2.x, 0., vec2.y);
                    *vec2 = tf.transform_point(pt).xz();
                });
            }
        }
    }
}
