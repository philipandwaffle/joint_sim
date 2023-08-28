use bevy::prelude::*;

pub struct Muscle {
    pub joints: [Entity; 2],
    pub impulse_scale: Vec2,
}
impl Muscle {
    pub fn new(joints: [Entity; 2]) -> Self {
        return Self {
            joints: joints,
            impulse_scale: Vec2::ZERO,
        };
    }
}
