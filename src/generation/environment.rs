use bevy::{
    math::{vec2, vec3},
    prelude::{default, Color, Commands, Res, Transform},
};
use bevy_prototype_lyon::{
    prelude::{Fill, GeometryBuilder, ShapeBundle},
    shapes,
};
use bevy_rapier2d::prelude::{Collider, RigidBody};

use crate::config::structs::GenerationConfig;

pub fn spawn_environment(mut commands: Commands, config: Res<GenerationConfig>) {
    let num_organisms = config.num_organisms;
    let vertical_sep = config.vertical_sep;
    let width = 4000.0;
    let height = 20.0;

    let platform = shapes::Rectangle {
        extents: vec2(width, height),
        ..default()
    };
    let wall = shapes::Rectangle {
        extents: vec2(height, num_organisms as f32 * vertical_sep),
        ..default()
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&wall),
            transform: Transform::from_translation(vec3(
                -200.0,
                vertical_sep * num_organisms as f32 * 0.5,
                0.0,
            )),
            ..default()
        },
        Fill::color(Color::BLACK),
        RigidBody::Fixed,
        Collider::cuboid(height * 0.5, vertical_sep * num_organisms as f32 * 0.5),
    ));
    for i in 0..=num_organisms {
        println!("{:?}", i as f32 * vertical_sep);
        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&platform),
                transform: Transform::from_translation(vec3(
                    (width / 2.0) - 200.0,
                    (i as f32) * vertical_sep,
                    0.0,
                )),
                ..default()
            },
            Fill::color(Color::BLACK),
            RigidBody::Fixed,
            Collider::cuboid(width * 0.5, height * 0.5),
        ));
    }
}
