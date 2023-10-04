use crate::{config::structs::CameraConfig, controls::control_state::ControlState};

use bevy::prelude::*;

#[derive(Component)]
pub struct ScrollingCam;

pub fn translate_cam(
    mut cam: Query<(&mut Transform, &mut OrthographicProjection), With<ScrollingCam>>,
    mut cs: ResMut<ControlState>,
) {
    match cam.get_single_mut() {
        Ok((mut t, mut op)) => {
            t.translation += cs.translate_delta.extend(0.0);
            op.scale += cs.zoom_delta * 0.1;

            cs.reset_translation();
            cs.reset_zoom();
        }
        Err(err) => println!("More than one cam in the scene, {:?}", err),
    }
}

pub fn spawn_cam(mut commands: Commands, cc: Res<CameraConfig>) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_translation(cc.starting_translation.extend(1.0)),
            projection: OrthographicProjection {
                scale: cc.starting_zoom,
                ..default()
            },
            ..default()
        },
        ScrollingCam,
    ));
}
