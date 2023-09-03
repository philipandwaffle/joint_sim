use crate::controls::control_state::ControlState;

use bevy::{math::vec3, prelude::*};

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

pub fn spawn_cam(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_translation(vec3(0.0, 0.0, 1.0)),
            ..default()
        },
        ScrollingCam,
    ));
}
