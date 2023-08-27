use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    window::WindowMode,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::ImpulseJointSet;
use scrolling_cam::ScrollingCamPlugin;
use world_object::body::{Body, BodyList};
use world_object::bone::Bone;
use world_object::joint::JointBundle;
use world_object::muscle::Muscle;
use world_object::OrganismPlugin;

mod scrolling_cam;
mod world_object;

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
                    resolution: (1900., 1280.).into(),
                    // present_mode: PresentMode::AutoVsync,
                    mode: WindowMode::BorderlessFullscreen,
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
        // OrganismPlugin,
    ))
    .add_systems(Startup, (spawn_ground, spawn_organism_test));

    if profiling_mode {
        app.add_plugins((
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
        ));
    }
    if debug_mode {
        app.add_plugins((
            RapierDebugRenderPlugin::default(),
            WorldInspectorPlugin::new(),
        ));
    }

    app.run();
}

fn spawn_ground(mut commands: Commands) {
    let rectangle = shapes::Rectangle {
        extents: vec2(1000.0, 20.0),
        ..default()
    };
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&rectangle),
            transform: Transform::from_translation(vec3(0.0, -200.0, 0.0)),
            ..default()
        },
        Fill::color(Color::BLACK),
        RigidBody::Fixed,
        Collider::cuboid(500.0, 10.0),
    ));
}

fn spawn_organism_test(mut commands: Commands) {
    let a_pos = vec2(-100.0, 50.0);
    let b_pos = vec2(0.0, 0.0);
    let c_pos = vec2(100.0, 50.0);

    let a_ent = commands.spawn(JointBundle::from_translation(a_pos)).id();
    let b_ent = commands.spawn(JointBundle::from_translation(b_pos)).id();
    let c_ent = commands.spawn(JointBundle::from_translation(c_pos)).id();

    let ba_bone_motor = Bone::new(&mut commands, [b_ent, a_ent], [b_pos, a_pos], None);
    let bc_bone_motor = Bone::new(&mut commands, [b_ent, c_ent], [b_pos, c_pos], None);

    commands.insert_resource(BodyList {
        bodies: vec![Body {
            muscles: vec![Muscle {
                bone_motors: [ba_bone_motor, bc_bone_motor],
            }],
        }],
    })
}

fn spawn_test_scene(mut commands: Commands) {
    let a_pos = vec2(-100.0, 0.0);
    let b_pos = vec2(100.0, 0.0);
    let c_pos = vec2(0.0, 100.0);
    let d_pos = vec2(-200.0, 50.0);
    let e_pos = vec2(200.0, 50.0);

    let a_ent = commands.spawn(JointBundle::from_translation(a_pos)).id();
    let b_ent = commands.spawn(JointBundle::from_translation(b_pos)).id();
    let c_ent = commands.spawn(JointBundle::from_translation(c_pos)).id();
    let d_ent = commands.spawn(JointBundle::from_translation(d_pos)).id();
    let e_ent = commands.spawn(JointBundle::from_translation(e_pos)).id();

    Bone::new(&mut commands, [a_ent, b_ent], [a_pos, b_pos], None);
    Bone::new(&mut commands, [b_ent, c_ent], [b_pos, c_pos], None);
    Bone::new(&mut commands, [c_ent, a_ent], [c_pos, a_pos], None);
    Bone::new(&mut commands, [c_ent, d_ent], [c_pos, d_pos], None);
    Bone::new(&mut commands, [c_ent, e_ent], [c_pos, e_pos], None);
}

fn spawn_performance_test_scene(mut commands: Commands) {
    let mut pos_grid = vec![];
    for y in 0..20 {
        let mut row = vec![];
        for x in 0..20 {
            let pos = vec2(x as f32, y as f32) * 20.0;
            let entity = commands.spawn(JointBundle::new(pos, 5.0, 0.5, 0.5)).id();
            row.push(pos);
        }
        pos_grid.push(row);
    }
}
