use bevy::diagnostic::DiagnosticsPlugin;
use bevy::log::LogPlugin;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    window::WindowMode,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use handles::setup_handles;
use organism::helper_fn::quat_z_rot;
use organism_constructor::OrganismConstructionPlugin;
use scene_manager::SceneManagerPlugin;
use std::env;
use std::f32::consts::PI;
use wasm_bindgen::prelude::*;

use controls::ControlPlugin;
use generation::GenerationPlugin;
use organism::OrganismPlugin;

use crate::config::ConfigPlugin;

mod collider_layer;
mod color_palette;
mod config;
mod controls;
mod generation;
mod handles;
mod organism;
mod organism_constructor;
mod scene_manager;

#[wasm_bindgen]
pub fn start() {
    env::set_var("RUST_BACKTRACE", "full");

    let profiling_mode = false;
    let debug_mode = false;

    let mut app = App::new();
    app.insert_resource(RapierConfiguration {
        gravity: Vec2::NEG_Y * 200.0,
        ..default()
    })
    .insert_resource(Msaa::Sample4)
    .add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Joint Sim".into(),
                    position: WindowPosition::At(IVec2::ZERO),
                    // resolution: (1920., 1080.).into(),
                    resolution: (1920. / 6.0, 1080.).into(),
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
            .set(ImagePlugin::default_nearest()), // .disable::<LogPlugin>()
                                                  // .disable::<DiagnosticsPlugin>(),
    )
    .add_systems(PreStartup, setup_handles)
    .add_plugins((
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        // RapierPhysicsPlugin::<ColliderLayerHook>::pixels_per_meter(100.0),
        ConfigPlugin,
        ControlPlugin,
        SceneManagerPlugin,
        GenerationPlugin,
        OrganismConstructionPlugin,
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
            WorldInspectorPlugin::new(),
        ));
    }
    // app.add_systems(Update, log_world);
    app.run();
}
