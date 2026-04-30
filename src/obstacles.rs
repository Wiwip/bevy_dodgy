use crate::geometry::{point_on_circle, rect_inner};
use bevy::prelude::*;
use dodgy_2d::Obstacle;

#[cfg(feature = "2d")]
use avian2d::prelude::*;
#[cfg(feature = "2d")]
use avian2d::parry::shape::TypedShape;

#[cfg(feature = "3d")]
use avian3d::prelude::*;
#[cfg(feature = "3d")]
use avian3d::parry::shape::TypedShape;

pub trait AsObstacle {
    fn to_obstacle(&self) -> Option<Obstacle>;
}

impl AsObstacle for Collider {
    fn to_obstacle(&self) -> Option<Obstacle> {
        let shape = self.shape_scaled().as_typed_shape();
        match shape {
            TypedShape::Cuboid(cuboid) => {
                let [tr, tl, bl, br] = rect_inner(Vec2::from(cuboid.half_extents.xz() * 2.0));

                Some(Obstacle::Closed {
                    vertices: vec![tr, tl, bl, br],
                })
            }
            #[cfg(feature = "2d")]
            TypedShape::Cuboid(cuboid) => {
                let [tr, tl, bl, br] = rect_inner(Vec2::from(cuboid.half_extents * 2.0));

                Some(Obstacle::Closed {
                    vertices: vec![tr, tl, bl, br],
                })
            }
            #[cfg(feature = "2d")]
            TypedShape::Triangle(tri) => Some(Obstacle::Closed {
                vertices: vec![tri.a.xy().into(), tri.b.xy().into(), tri.c.xy().into()],
            }),
            #[cfg(feature = "2d")]
            TypedShape::Ball(ball) => {
                let num_vertices = 12; // TODO configuration resource
                Some(Obstacle::Closed {
                    vertices: (0..num_vertices)
                        .map(|i| {
                            let theta =
                                2.0 * std::f32::consts::PI * (i as f32) / (num_vertices as f32);
                            point_on_circle((0., 0.), ball.radius, theta)
                        })
                        .collect(),
                })
            }
            _ => {
                warn_once!("This shape isn't supported.");
                None
            }
        }
    }
}

pub trait TransformObstacle {
    fn transform_points(&mut self, tf: &GlobalTransform);
}

impl TransformObstacle for Obstacle {
    #[cfg(feature = "2d")]
    fn transform_points(&mut self, tf: &GlobalTransform) {
        match self {
            Obstacle::Closed { vertices } => {
                vertices.iter_mut().for_each(|vec2| {
                    let pt = Vec3::new(vec2.x, vec2.y, 0.0);
                    *vec2 = tf.transform_point(pt).xy();
                });
            }
            Obstacle::Open { vertices } => {
                vertices.iter_mut().for_each(|vec2| {
                    let pt = Vec3::new(vec2.x, vec2.y, 0.0);
                    *vec2 = tf.transform_point(pt).xy();
                });
            }
        }
    }

    #[cfg(feature = "3d")]
    fn transform_points(&mut self, tf: &GlobalTransform) {
        match self {
            Obstacle::Closed { vertices } => {
                vertices.iter_mut().for_each(|vec2| {
                    let pt = Vec3::new(vec2.x, 0.0, vec2.y);
                    *vec2 = tf.transform_point(pt).xz();
                });
            }
            Obstacle::Open { vertices } => {
                vertices.iter_mut().for_each(|vec2| {
                    let pt = Vec3::new(vec2.x, 0.0, vec2.y);
                    *vec2 = tf.transform_point(pt).xz();
                });
            }
        }
    }
}
