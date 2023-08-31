use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::ui::debug::print_ui_layout_tree;
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    window::WindowMode,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use generation::GenerationPlugin;
use organism::bone::Bone;
use organism::brain::Brain;
use organism::joint::{self, JointBundle};
use organism::muscle::Muscle;
use organism::organism::Organism;
use organism::OrganismPlugin;
use rand::Rng;
use scrolling_cam::ScrollingCamPlugin;

mod generation;
mod organism;
mod scrolling_cam;
fn main() {
    let profiling_mode = false;
    let debug_mode = true;

    let mut app = App::new();
    app.insert_resource(RapierConfiguration {
        gravity: Vec2::NEG_Y * 100.0,
        ..default()
    })
    .insert_resource(Msaa::Sample4)
    .add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Particle Sim".into(),
                    resolution: (1900. / 4.0, 1280. / 4.0).into(),
                    // present_mode: PresentMode::AutoVsync,
                    mode: WindowMode::Windowed,
                    // Tells wasm to resize the window according to the available canvas
                    fit_canvas_to_parent: true,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            })
            // don't use linear sampling as image textures will be blurry
            .set(ImagePlugin::default_nearest()),
    )
    .add_plugins((
        ShapePlugin,
        RapierPhysicsPlugin::<NoUserData>::default(),
        // RegisterTraitPlugin,
        ScrollingCamPlugin,
        GenerationPlugin,
        OrganismPlugin,
    ))
    .add_systems(Startup, spawn_ground);

    if profiling_mode {
        app.add_plugins((
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
        ));
    }
    if debug_mode {
        app.add_plugins((
            RapierDebugRenderPlugin::default(),
            // WorldInspectorPlugin::new(),
        ));
    }

    app.run();
}

fn spawn_ground(mut commands: Commands) {
    let num_creatures = 500;
    let vertical_sep = 200.0;
    let width = 2000.0;
    let height = 20.0;

    let platform = shapes::Rectangle {
        extents: vec2(width, height),
        ..default()
    };
    let wall = shapes::Rectangle {
        extents: vec2(height, num_creatures as f32 * vertical_sep),
        ..default()
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&wall),
            transform: Transform::from_translation(vec3(
                -200.0,
                (vertical_sep * num_creatures as f32 * 0.5) - 20.0,
                0.0,
            )),
            ..default()
        },
        Fill::color(Color::BLACK),
        RigidBody::Fixed,
        Collider::cuboid(height * 0.5, vertical_sep * num_creatures as f32 * 0.5),
    ));
    for i in 0..=num_creatures {
        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&platform),
                transform: Transform::from_translation(vec3(
                    (width / 2.0) - 200.0,
                    (i as f32 * 200.0) - 20.0,
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
