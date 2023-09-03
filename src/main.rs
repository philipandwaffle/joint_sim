use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    window::WindowMode,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use generation::GenerationPlugin;
use organism::brain::{Brain, Matrix, MxNMatrix};
use organism::joint::JointBundle;
use organism::{brain, OrganismPlugin};
use scrolling_cam::ScrollingCamPlugin;

mod generation;
mod organism;
mod scrolling_cam;
fn main() {
    let matrix = Matrix::from_vec(6, 1, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    println!("in {:?}", matrix);
    let m_string = serde_json::to_string(&MxNMatrix(matrix)).unwrap();
    println!("matrix, {:?}", &m_string);
    let m = serde_json::from_str::<MxNMatrix>(&m_string).unwrap();
    println!("result {:?}", m.0);
    // match serde_json::from_str::<MxMMatrix>(&m_string) {
    //     Ok(m) => {
    //         println!("worked, {:?}", m.0);
    //     }
    //     Err(err) => println!("{:?}", err),
    // }

    return;

    // let brain = Brain::new(vec![5, 5]);
    // let foo = serde_json::to_string(&brain).unwrap();
    // println!("{:?}", foo);

    // match serde_json::from_str::<Brain>(&foo) {
    //     Ok(b) => {
    //         println!("Hello");
    //     }
    //     Err(err) => println!("{:?}", err),
    // }

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
                    position: WindowPosition::At(IVec2::ZERO),
                    resolution: (1900. / 6.0, 1000.).into(),
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
        ScrollingCamPlugin,
        GenerationPlugin,
        OrganismPlugin,
    ));

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

    // app.add_systems(Startup, spawn_test_bone);

    app.run();
}
fn spawn_test_bone(mut commands: Commands) {
    let a_pos = vec2(-100.0, 50.0);
    let b_pos = vec2(100.0, 50.0);
    let rect = shapes::Rectangle {
        extents: vec2(100.0, 10.0),
        ..default()
    };

    let bone = commands
        .spawn((
            RigidBody::Dynamic,
            ShapeBundle {
                path: GeometryBuilder::build_as(&rect),
                transform: Transform::from_translation(vec3(0.0, 100.0, 0.0)),
                ..default()
            },
            // Collider::ball(2.0),
            Fill::color(Color::RED),
            GravityScale(5.0),
        ))
        .id();

    let a_rev_joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(0.0, 0.0))
        .local_anchor2(Vec2::new(-50.0, 0.0))
        .build();
    let b_rev_joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(0.0, 0.0))
        .local_anchor2(Vec2::new(50.0, 0.0))
        .build();

    let a = commands.spawn(JointBundle::from_translation(a_pos)).id();
    let b = commands.spawn(JointBundle::from_translation(b_pos)).id();

    let a_to_bone = commands.spawn(ImpulseJoint::new(a, a_rev_joint)).id();
    let b_to_bone = commands.spawn(ImpulseJoint::new(b, b_rev_joint)).id();

    commands.get_entity(bone).unwrap().add_child(a_to_bone);
    commands.get_entity(bone).unwrap().add_child(b_to_bone);
}
