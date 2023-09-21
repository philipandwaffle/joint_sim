use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::config::structs::CameraConfig;

#[derive(Resource)]
pub struct ControlState {
    pub translate_delta: Vec2,
    pub zoom_delta: f32,
    pub left_mouse_down: bool,
    pub save: bool,
}
impl Default for ControlState {
    fn default() -> Self {
        Self {
            translate_delta: Vec2::ZERO,
            zoom_delta: 0.0,
            left_mouse_down: false,
            save: false,
        }
    }
}
impl ControlState {
    pub fn reset_translation(&mut self) {
        self.translate_delta = Vec2::ZERO;
    }
    pub fn reset_zoom(&mut self) {
        self.zoom_delta = 0.0;
    }
}

#[derive(Resource)]
pub struct Bindings {
    up: KeyCode,
    left: KeyCode,
    down: KeyCode,
    right: KeyCode,
    zoom_in: KeyCode,
    zoom_out: KeyCode,
    save: KeyCode,
}
impl Default for Bindings {
    fn default() -> Self {
        Self {
            up: KeyCode::W,
            left: KeyCode::A,
            down: KeyCode::S,
            right: KeyCode::D,
            zoom_in: KeyCode::Up,
            zoom_out: KeyCode::Down,
            save: KeyCode::P,
        }
    }
}

pub fn update_control_state(
    keyboard: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut cs: ResMut<ControlState>,
    bindings: Res<Bindings>,
    camera_config: Res<CameraConfig>,
) {
    let mut td = Vec2::ZERO;
    if keyboard.pressed(bindings.up) {
        td.y += 1.0
    }
    if keyboard.pressed(bindings.left) {
        td.x -= 1.0
    }
    if keyboard.pressed(bindings.down) {
        td.y -= 1.0
    }
    if keyboard.pressed(bindings.right) {
        td.x += 1.0
    }

    let mut zd = 0.0;
    if keyboard.pressed(bindings.zoom_in) {
        zd += 1.0
    }
    if keyboard.pressed(bindings.zoom_out) {
        zd -= 1.0
    }

    cs.left_mouse_down = mouse.pressed(MouseButton::Right);

    if !cs.save && keyboard.just_pressed(bindings.save) {
        cs.save = true;
    }

    if td != Vec2::ZERO {
        cs.translate_delta = td * camera_config.move_modifier;
    }
    if zd != 0.0 {
        cs.zoom_delta = zd * camera_config.zoom_modifier;
    }
}
