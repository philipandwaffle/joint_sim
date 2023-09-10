use bevy::prelude::{Commands, Component, Entity, Query, Transform, Vec2, With};
use bevy_prototype_lyon::{prelude::ShapeBundle, shapes};

use super::bone::Bone;

pub struct MuscleBundle {
    shape_bundle: ShapeBundle,
}
impl MuscleBundle {
    pub fn new(
        commands: &mut Commands,
        bones: Query<&Transform, With<Bone>>,
        muscles: [Entity; 2],
    ) -> Entity {
        // commands.get_entity(muscles[0]).unwrap();
        // let bone_rect = shapes::Rectangle {
        //     extents: vec2(bone_width, len),
        //     origin: shapes::RectangleOrigin::Center,
        // };

        todo!();
    }
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
