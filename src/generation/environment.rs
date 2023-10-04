use bevy::{
    math::vec2,
    prelude::{default, Bundle, Commands, Entity, Handle, Resource, Transform, Vec2},
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::{Collider, Friction, RigidBody};

use crate::config::structs::GenerationConfig;

#[derive(Resource)]
pub struct Environment {
    env_ents: Vec<Entity>,
}
impl Environment {
    pub fn new() -> Self {
        return Self { env_ents: vec![] };
    }

    pub fn spawn(
        &mut self,
        commands: &mut Commands,
        mesh: &Mesh2dHandle,
        material: &Handle<ColorMaterial>,
        gc: &GenerationConfig,
    ) {
        let num_organisms = gc.num_organisms;
        let vertical_sep = gc.vertical_sep;
        let width = 4000.0;
        let height = 20.0;

        let wall = commands
            .spawn(Block::new(
                vec2(-200.0, vertical_sep * num_organisms as f32 * 0.5),
                vec2(height, vertical_sep * num_organisms as f32),
                mesh,
                material,
            ))
            .id();
        self.env_ents.push(wall);

        for i in 0..=num_organisms {
            let floor = commands
                .spawn(Block::new(
                    vec2((width / 2.0) - 200.0, (i as f32) * vertical_sep),
                    vec2(width, height),
                    &mesh,
                    &material,
                ))
                .id();
            self.env_ents.push(floor);
        }
    }
    pub fn despawn(&self, commands: &mut Commands) {
        for e in self.env_ents.iter() {
            commands.entity(*e).despawn();
        }
    }
}

#[derive(Bundle)]
struct Block {
    material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    friction: Friction,
    rigid_body: RigidBody,
    collider: Collider,
}
impl Block {
    pub fn new(
        translation: Vec2,
        extents: Vec2,
        mesh: &Mesh2dHandle,
        material: &Handle<ColorMaterial>,
    ) -> Self {
        return Self {
            material_mesh_bundle: MaterialMesh2dBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: Transform {
                    translation: translation.extend(0.0),
                    scale: extents.extend(0.0),
                    ..default()
                },
                ..default()
            },
            friction: Friction::coefficient(0.7),
            rigid_body: RigidBody::Fixed,
            collider: Collider::cuboid(0.5, 0.5),
        };
    }
}
