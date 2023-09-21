use bevy::{
    prelude::{default, Bundle, Component, Handle, Transform, Vec2},
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Component)]
pub struct Icon;

#[derive(Bundle)]
pub struct JointIcon {
    icon: Icon,
    material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
}
impl JointIcon {
    pub fn new(translation: Vec2, mesh: &Mesh2dHandle, material: &Handle<ColorMaterial>) -> Self {
        return Self {
            icon: Icon,
            material_mesh_bundle: MaterialMesh2dBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: Transform::from_translation(translation.extend(0.3)),
                ..default()
            },
        };
    }
}
