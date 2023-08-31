use bevy::prelude::{Entity, Vec2};

#[derive(Clone)]
pub struct Muscle {
    pub joints: [Entity; 2],
    pub base_len: f32,
    pub len_modifier: f32,
}
impl Muscle {
    pub fn new(joints: [Entity; 2], joint_pos: [Vec2; 2]) -> Self {
        return Self {
            joints: joints,
            base_len: (joint_pos[1] - joint_pos[0]).length(),
            len_modifier: 1.0,
        };
    }

    pub fn get_target_len(&self) -> f32 {
        return self.base_len + (self.base_len * self.len_modifier);
    }
}
