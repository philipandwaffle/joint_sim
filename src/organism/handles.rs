use bevy::{
    math::vec2,
    prelude::{shape, Assets, Color, Commands, Handle, Mesh, ResMut, Resource},
    sprite::{ColorMaterial, Mesh2dHandle},
};

#[derive(Resource)]
pub struct Handles {
    pub joint_mesh: Mesh2dHandle,
    pub joint_material: Handle<ColorMaterial>,
    pub bone_mesh: Mesh2dHandle,
    pub bone_material: Handle<ColorMaterial>,
    pub muscle_mesh: Mesh2dHandle,
    pub muscle_contract_material: Handle<ColorMaterial>,
    pub muscle_expand_material: Handle<ColorMaterial>,
    pub muscle_neutral_material: Handle<ColorMaterial>,
}

pub fn setup_handles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(Handles {
        joint_mesh: meshes.add(shape::Circle::new(5.0).into()).into(),
        joint_material: materials.add(ColorMaterial::from(Color::hsl(115.0, 0.60, 0.35))),
        bone_mesh: meshes.add(shape::Quad::new(vec2(1.0, 1.0)).into()).into(),
        bone_material: materials.add(ColorMaterial::from(Color::hsl(0.0, 0.50, 0.90))),
        muscle_mesh: meshes.add(shape::Quad::new(vec2(1.0, 1.0)).into()).into(),
        muscle_contract_material: materials.add(ColorMaterial::from(Color::hsl(0.0, 0.60, 0.45))),
        muscle_expand_material: materials.add(ColorMaterial::from(Color::hsl(240.0, 0.60, 0.45))),
        muscle_neutral_material: materials.add(ColorMaterial::from(Color::hsl(300.0, 0.60, 0.45))),
    });
}
