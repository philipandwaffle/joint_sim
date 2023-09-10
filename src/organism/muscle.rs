use bevy::{
    math::vec2,
    prelude::{default, Bundle, Commands, Component, Entity, Quat, Query, Transform, Vec2, With},
};
use bevy_prototype_lyon::{
    prelude::{GeometryBuilder, ShapeBundle},
    shapes,
};

use super::bone::Bone;

#[derive(Bundle)]
pub struct MuscleBundle {
    shape_bundle: ShapeBundle,
    muscle: MuscleTag,
}
impl MuscleBundle {
    pub fn new(len: f32, z_rot: f32) -> Self {
        let muscle_width = 3.0;

        let muscle_rect = shapes::Rectangle {
            extents: vec2(muscle_width, len),
            origin: shapes::RectangleOrigin::Center,
        };

        return Self {
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&muscle_rect),
                transform: Transform::from_rotation(Quat::from_rotation_z(z_rot)),
                ..default()
            },
            muscle: MuscleTag {
                base_len: len,
                len_modifier: 0.0,
            },
        };
    }
}

#[derive(Component)]
pub struct MuscleTag {
    pub base_len: f32,
    pub len_modifier: f32,
}

// Muscle containing 2 joints and length data
#[derive(Clone, Debug, Component)]
pub struct Muscle {
    pub bones: [Entity; 2],
    pub base_len: f32,
    pub len_modifier: f32,
}
impl Muscle {
    // Create a new muscle
    pub fn new(bones: [Entity; 2], joint_pos: [Vec2; 2]) -> Self {
        return Self {
            bones,
            base_len: (joint_pos[1] - joint_pos[0]).length(),
            len_modifier: 1.0,
        };
    }

    pub fn spawn() {}

    // Get the target length of the muscle
    pub fn get_target_len(&self) -> f32 {
        return self.base_len + (self.base_len * self.len_modifier);
    }
}
