use bevy::{
    math::vec3,
    prelude::{default, Bundle, Commands, Component, Entity, Handle, Quat, Transform, Vec2},
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Bundle)]
pub struct MuscleBundle {
    material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    muscle: Muscle,
}
impl MuscleBundle {
    pub fn spawn(
        commands: &mut Commands,
        mesh: &Mesh2dHandle,
        material: &Handle<ColorMaterial>,
        bones: [Entity; 2],
        bone_pos: [Vec2; 2],
    ) -> Entity {
        let ab = bone_pos[1] - bone_pos[0];
        // let dir = ab * 0.5;
        let len = ab.length();
        let x = if ab.x >= 0.0 { -1.0 } else { 1.0 };
        let z_rot = x * f32::acos(ab.y / len);

        let muscle_ent = commands
            .spawn(MuscleBundle::new(
                &mesh,
                &material,
                ab * 0.5,
                len,
                z_rot,
                bones,
            ))
            .id();
        // commands.get_entity(bones[0]).unwrap().add_child(muscle_ent);

        return muscle_ent;
    }

    pub fn new(
        mesh: &Mesh2dHandle,
        material: &Handle<ColorMaterial>,
        translation: Vec2,
        len: f32,
        z_rot: f32,
        bones: [Entity; 2],
    ) -> Self {
        let muscle_width = 2.0;

        return Self {
            material_mesh_bundle: MaterialMesh2dBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: Transform {
                    translation: translation.extend(-0.3),
                    // translation: vec3(0.0, -len * 0.5, -0.2),
                    rotation: Quat::from_rotation_z(z_rot),
                    scale: vec3(muscle_width, len, 0.0),
                },
                ..default()
            },
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
