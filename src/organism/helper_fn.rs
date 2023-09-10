use bevy::prelude::Vec2;

pub fn vec2_z_rot(a: Vec2, b: Vec2) -> f32 {
    let ab = b - a;
    let dir = ab * 0.5;
    let len = ab.length();
    let x = if ab.x >= 0.0 { -1.0 } else { 1.0 };
    return x * f32::acos(ab.y / len);
}
