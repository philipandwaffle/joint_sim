use bevy::prelude::*;

#[derive(Resource)]
pub struct ControlState {
    pub translate_delta: Vec2,
    pub zoom_delta: f32,
}
impl Default for ControlState {
    fn default() -> Self {
        Self {
            translate_delta: Vec2::ZERO,
            zoom_delta: 0.0,
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
        }
    }
}

pub fn update_control_state(
    input: Res<Input<KeyCode>>,
    mut cs: ResMut<ControlState>,
    bindings: Res<Bindings>,
) {
    let mut td = Vec2::ZERO;
    if input.pressed(bindings.up) {
        td.y += 1.0
    }
    if input.pressed(bindings.left) {
        td.x -= 1.0
    }
    if input.pressed(bindings.down) {
        td.y -= 1.0
    }
    if input.pressed(bindings.right) {
        td.x += 1.0
    }

    let mut zd = 0.0;
    if input.pressed(bindings.zoom_in) {
        zd += 1.0
    }
    if input.pressed(bindings.zoom_out) {
        zd -= 1.0
    }

    if td != Vec2::ZERO {
        cs.translate_delta = td;
    }
    if zd != 0.0 {
        cs.zoom_delta = zd;
    }
}
