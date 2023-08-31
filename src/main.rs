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
use organism::bone::Bone;
use organism::brain::Brain;
use organism::joint::{self, JointBundle};
use organism::muscle::Muscle;
use organism::organism::{Organism, OrganismList};
use organism::OrganismPlugin;
use organism_tester::OrganismTestingPlugin;
use rand::Rng;
use scrolling_cam::ScrollingCamPlugin;

mod organism;
mod organism_tester;
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
        OrganismTestingPlugin,
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

fn bone_testing(mut commands: Commands) {
    let a_pos = vec2(-100.0, 50.0);
    let b_pos = vec2(100.0, 50.0);

    let joint_a = commands.spawn(JointBundle::new(a_pos, 5.0, 0.5, 0.5)).id();
    let joint_b = commands.spawn(JointBundle::new(b_pos, 5.0, 0.5, 0.5)).id();

    let bone_ab_node0 = commands.spawn(RigidBody::Dynamic).id();
    let bone_ab_node1 = commands.spawn(RigidBody::Dynamic).id();

    let joint = RevoluteJointBuilder::new()
        .local_anchor1(a_pos)
        .local_anchor2(b_pos)
        .build();

    // let mut a_child;
    // commands.get_entity(joint_a).unwrap().with_children(|p| {
    //     a_child = p.spawn(RigidBody::Dynamic).id();
    // });
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
