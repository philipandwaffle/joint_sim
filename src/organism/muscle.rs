use bevy::{
    math::vec2,
    prelude::{
        default, BuildChildren, Bundle, Color, Commands, Component, Entity, Quat, Transform, Vec2,
    },
};
use bevy_prototype_lyon::{
    prelude::{Fill, GeometryBuilder, ShapeBundle},
    shapes,
};
#[derive(Bundle)]
pub struct MuscleBundle {
    shape_bundle: ShapeBundle,
    fill: Fill,
    muscle: Muscle,
}
impl MuscleBundle {
    pub fn spawn(commands: &mut Commands, bones: [Entity; 2], bone_pos: [Vec2; 2]) -> Entity {
        let ab = bone_pos[1] - bone_pos[0];
        // let dir = ab * 0.5;
        let len = ab.length();
        let x = if ab.x >= 0.0 { 1.0 } else { -1.0 };
        let z_rot = x * f32::acos(ab.y / len);

        let muscle_ent = commands
            .spawn(MuscleBundle::new(len, vec2(0.0, 0.0), z_rot, bones))
            .id();
        commands.get_entity(bones[0]).unwrap().add_child(muscle_ent);

        return muscle_ent;
    }

    pub fn new(len: f32, translation: Vec2, z_rot: f32, bones: [Entity; 2]) -> Self {
        let muscle_width = 2.0;

        let muscle_rect = shapes::Rectangle {
            extents: vec2(muscle_width, len),
            origin: shapes::RectangleOrigin::CustomCenter(vec2(0.0, -len * 0.5)),
        };

        return Self {
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&muscle_rect),
                transform: Transform {
                    translation: translation.extend(-0.2),
                    rotation: Quat::from_rotation_z(z_rot),
                    ..default()
                },
                ..default()
            },
            fill: Fill::color(Color::RED),
            muscle: Muscle {
                bones,
                base_len: len,
                len_modifier: 0.0,
            },
        };
    }
}

#[derive(Component)]
pub struct Muscle {
    pub bones: [Entity; 2],
    pub base_len: f32,
    pub len_modifier: f32,
}
impl Muscle {
    pub fn get_target_len(&self) -> f32 {
        return self.base_len + (self.base_len * self.len_modifier);
    }
}
// // Muscle containing 2 joints and length data
// #[derive(Clone, Debug, Component)]
// pub struct Muscle {
//     pub bones: [Entity; 2],
//     pub base_len: f32,
//     pub len_modifier: f32,
// }
// impl Muscle {
//     // Create a new muscle
//     pub fn new(bones: [Entity; 2], joint_pos: [Vec2; 2]) -> Self {
//         return Self {
//             bones,
//             base_len: (joint_pos[1] - joint_pos[0]).length(),
//             len_modifier: 1.0,
//         };
//     }

//     pub fn spawn() {}

//     // Get the target length of the muscle
//     pub fn get_target_len(&self) -> f32 {
//         return self.base_len + (self.base_len * self.len_modifier);
//     }
// }
