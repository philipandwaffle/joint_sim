use bevy::{
    math::vec2,
    prelude::{
        default, shape, Assets, Bundle, Color, Commands, Mesh, Res, ResMut, Transform, Vec2,
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle},
};
use bevy_rapier2d::prelude::{Collider, Friction, RigidBody};

use crate::config::structs::GenerationConfig;

pub fn spawn_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    config: Res<GenerationConfig>,
) {
    let num_organisms = config.num_organisms;
    let vertical_sep = config.vertical_sep;
    let width = 4000.0;
    let height = 20.0;

    commands.spawn(Wall::new(
        vec2(-200.0, vertical_sep * num_organisms as f32 * 0.5),
        vec2(height, vertical_sep * num_organisms as f32),
        &mut meshes,
        &mut materials,
    ));
    for i in 0..=num_organisms {
        commands.spawn(Wall::new(
            vec2((width / 2.0) - 200.0, (i as f32) * vertical_sep),
            vec2(width, height),
            &mut meshes,
            &mut materials,
        ));
    }
}

#[derive(Bundle)]
struct Wall {
    material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    friction: Friction,
    rigid_body: RigidBody,
    collider: Collider,
}
impl Wall {
    pub fn new(
        translation: Vec2,
        extents: Vec2,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) -> Self {
        let mesh_handle = meshes.add(shape::Quad::new(extents).into()).into();
        let material_handle = materials.add(ColorMaterial::from(Color::BLACK));

        return Self {
            material_mesh_bundle: MaterialMesh2dBundle {
                mesh: mesh_handle,
                material: material_handle,
                transform: Transform::from_translation(translation.extend(0.0)),
                ..default()
            },
            friction: Friction::coefficient(0.7),
            rigid_body: RigidBody::Fixed,
            collider: Collider::cuboid(extents.x * 0.5, extents.y * 0.5),
        };
    }
}
