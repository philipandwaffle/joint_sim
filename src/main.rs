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

    app.run();
}
