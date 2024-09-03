use bevy::math::{Vec2, Vec3};

pub fn rect_inner(size: Vec3) -> [Vec2; 4] {
    let half_size = size / 2.;
    let tl = Vec2::new(-half_size.x, half_size.z);
    let tr = Vec2::new(half_size.x, half_size.z);
    let bl = Vec2::new(-half_size.x, -half_size.z);
    let br = Vec2::new(half_size.x, -half_size.z);
    [tr, tl, bl, br]
}

pub fn point_on_circle(center: (f32, f32), radius: f32, theta: f32) -> Vec2 {
    let x = center.0 + radius * theta.cos();
    let y = center.1 + radius * theta.sin();
    Vec2::new(x, y)
}
