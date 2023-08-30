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
use organism::joint::JointBundle;
use organism::muscle::Muscle;
use organism::organism::{Organism, OrganismList};
use organism::OrganismPlugin;
use organism_tester::OrganismTestingPlugin;
use rand::Rng;
use scrolling_cam::ScrollingCamPlugin;

mod organism;
mod organism_tester;
mod scrolling_cam;

fn brain_test() {
    let mut rng = rand::thread_rng();

    let mut brain = Brain::new(vec![2, 5, 1], |x| f32::tanh(x));
    let mut brains = vec![];
    let mut fitness = vec![];

    for _ in 0..20 {
        let b = brain.clone();
        let f = calc_fitness(&b);

        brains.push(b);
        fitness.push(f);
    }

    for epoch in 0..100 {
        let avg_fitness = fitness.iter().sum::<f32>() / fitness.len() as f32;
        println!("epoch {} average fitness {}", epoch, avg_fitness);
        println!("fitnesses {:?}", fitness);

        let mut next_gen = vec![];
        for i in 0..brains.len() {
            if fitness[i] >= avg_fitness {
                let mut b = brains[i].clone();
                b.mutate(0.1, 1.0);
                next_gen.push(b);
            }
        }

        let next_gen_count = next_gen.len();
        for _ in 0..(20 - next_gen_count) {
            let mut b = (next_gen[rng.gen_range(0..next_gen_count)]).clone();
            b.mutate(0.1, 1.0);
            next_gen.push(b);
        }
        brains = next_gen;
        for i in 0..brains.len() {
            fitness[i] = calc_fitness(&brains[i]);
        }
    }

    println!("{:?}", brain);
    println!("{:?}", brain.feed_forward(vec![1.0, 1.0]));
    brain.mutate(0.1, 1.0);
    println!("{:?}", brain);
}

fn calc_fitness(b: &Brain) -> f32 {
    let i = b.feed_forward(vec![0.0, 0.0])[0];
    let j = b.feed_forward(vec![-1.0, 0.0])[0];
    let k = b.feed_forward(vec![0.0, -1.0])[0];
    let l = b.feed_forward(vec![1.0, 1.0])[0];

    return (j + k) - (i + l);
}

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
            WorldInspectorPlugin::new(),
        ));
    }

    app.run();
}

fn spawn_ground(mut commands: Commands) {
    let width = 2000.0;
    let height = 20.0;
    let rectangle = shapes::Rectangle {
        extents: vec2(width, height),
        ..default()
    };

    for i in 0..10 {
        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&rectangle),
                transform: Transform::from_translation(vec3(0.0, (i as f32 * 200.0) - 20.0, 0.0)),
                ..default()
            },
            Fill::color(Color::BLACK),
            RigidBody::Fixed,
            Collider::cuboid(width * 0.5, height * 0.5),
        ));
    }
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
