use bevy::{
    math::vec2,
    prelude::{Quat, Vec2},
};

pub fn vec2_z_rot(a: &Vec2, b: &Vec2) -> f32 {
    let ab = *b - *a;
    let len = ab.length();
    let x = if ab.x >= 0.0 { -1.0 } else { 1.0 };
    return x * (ab.y / len).acos();
}

// Get z rotation from a quaternion
pub fn quat_z_rot(q: &Quat) -> f32 {
    return f32::atan2(
        2.0 * (q.w * q.z + q.x * q.y),
        1.0 - 2.0 * (q.y * q.y + q.z * q.z),
    );
}

pub fn quat_to_vec2(q: &Quat) -> Vec2 {
    let rot = f32::atan2(
        2.0 * (q.w * q.z + q.x * q.y),
        1.0 - 2.0 * (q.y * q.y + q.z * q.z),
    );
    return vec2(rot.cos(), rot.sin());
}

pub fn rotate_vec(v: Vec2, z_rot: f32) -> Vec2 {
    let cos_theta = f32::cos(z_rot);
    let sin_theta = f32::sin(z_rot);
    return vec2(v.x * (cos_theta - sin_theta), v.y * (cos_theta + sin_theta));
}

// pub fn angle_between_vec
