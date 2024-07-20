use avian3d::parry::shape::TypedShape;
use avian3d::prelude::*;
use bevy::prelude::*;
use dodgy_2d::Obstacle;

pub trait AsObstacle {
    fn obstacle(&self) -> Option<Obstacle>;
}

impl AsObstacle for Collider {
    fn obstacle(&self) -> Option<Obstacle> {
        let shape = self.shape_scaled().as_typed_shape();
        match shape {
            TypedShape::Cuboid(cuboid) => {
                let [tr, tl, bl, br] = rect_inner(Vec3::from(cuboid.half_extents * 2.0));

                Some(Obstacle::Closed {
                    vertices: vec![tr, tl, bl, br],
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
    fn transform(&mut self, tf: &Transform);
}

impl TransformObstacle for Obstacle {
    fn transform(&mut self, tf: &Transform) {
        match self {
            Obstacle::Closed { vertices } => {
                vertices.iter_mut().for_each(|mut vec2| {
                    let pt = Vec3::new(vec2.x, 0., vec2.y);
                    let _ = tf.transform_point(pt).xz();
                });
            }
            Obstacle::Open { vertices } => {
                vertices.iter_mut().for_each(|mut vec2| {
                    let pt = Vec3::new(vec2.x, 0., vec2.y);
                    let _ = tf.transform_point(pt).xz();
                });
            }
        }
    }
}

fn rect_inner(size: Vec3) -> [Vec2; 4] {
    let half_size = size / 2.;
    let tl = Vec2::new(-half_size.x, half_size.z);
    let tr = Vec2::new(half_size.x, half_size.z);
    let bl = Vec2::new(-half_size.x, -half_size.z);
    let br = Vec2::new(half_size.x, -half_size.z);
    [tr, tl, bl, br]
}
