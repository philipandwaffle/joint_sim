use bevy::prelude::{Quat, Vec2};

pub fn vec2_z_rot(a: Vec2, b: Vec2) -> f32 {
    let ab = b - a;
    let len = ab.length();
    let x = if ab.x >= 0.0 { -1.0 } else { 1.0 };
    return x * f32::acos(ab.y / len);
}

// Get z rotation from a quaternion
pub fn quat_z_rot(q: Quat) -> f32 {
    return f32::atan2(
        2.0 * (q.w * q.z + q.x * q.y),
        1.0 - 2.0 * (q.y * q.y + q.z * q.z),
    );
}
