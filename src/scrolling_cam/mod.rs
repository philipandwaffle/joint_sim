use bevy::{math::vec3, prelude::*};

use self::{
    camera::{translate_cam, ScrollingCam},
    control_state::{update_control_state, Bindings, ControlState},
};

pub mod camera;
pub mod control_state;
pub struct ScrollingCamPlugin;
impl Plugin for ScrollingCamPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ControlState::default())
            .insert_resource(Bindings::default())
            .add_systems(Startup, spawn_cam)
            .add_systems(Update, (update_control_state, translate_cam));
    }
}

fn spawn_cam(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_translation(vec3(0.0, 0.0, 1.0)),
            ..default()
        },
        ScrollingCam,
    ));
}
