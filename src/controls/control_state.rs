use bevy::{input::mouse::MouseMotion, prelude::*, utils::Instant, window::PrimaryWindow};

use crate::config::structs::CameraConfig;

use super::camera::ScrollingCam;

#[derive(Resource)]
pub struct ControlState {
    pub translate_delta: Vec2,
    pub zoom_delta: f32,
    pub left_mouse_down: bool,
    pub double_click: bool,
    pub world_mouse_pos: Vec2,
    pub save: bool,
}
impl Default for ControlState {
    fn default() -> Self {
        Self {
            translate_delta: Vec2::ZERO,
            zoom_delta: 0.0,
            left_mouse_down: false,
            double_click: false,
            world_mouse_pos: Vec2::ZERO,
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
pub struct ControlConfig {
    up: KeyCode,
    left: KeyCode,
    down: KeyCode,
    right: KeyCode,
    zoom_in: KeyCode,
    zoom_out: KeyCode,
    save: KeyCode,
    double_click_window: f32,
}
impl Default for ControlConfig {
    fn default() -> Self {
        Self {
            up: KeyCode::W,
            left: KeyCode::A,
            down: KeyCode::S,
            right: KeyCode::D,
            zoom_in: KeyCode::Up,
            zoom_out: KeyCode::Down,
            save: KeyCode::P,
            double_click_window: 0.3,
        }
    }
}

pub struct DoubleClick {
    timer: Option<Instant>,
}
impl DoubleClick {
    pub fn is_double_click(&mut self, click_window: f32) -> bool {
        match self.timer {
            Some(t) => {
                let elapsed = t.elapsed().as_secs_f32();
                self.timer = Some(Instant::now());
                return elapsed <= click_window;
            }
            None => {
                self.timer = Some(Instant::now());
                return false;
            }
        }
    }
}
impl Default for DoubleClick {
    fn default() -> Self {
        Self { timer: None }
    }
}

pub fn update_control_state(
    keyboard: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut cs: ResMut<ControlState>,
    cc: Res<ControlConfig>,
    camera_config: Res<CameraConfig>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &mut GlobalTransform), With<ScrollingCam>>,
    mut double_click: Local<DoubleClick>,
) {
    let mut td = Vec2::ZERO;
    if keyboard.pressed(cc.up) {
        td.y += 1.0
    }
    if keyboard.pressed(cc.left) {
        td.x -= 1.0
    }
    if keyboard.pressed(cc.down) {
        td.y -= 1.0
    }
    if keyboard.pressed(cc.right) {
        td.x += 1.0
    }

    let mut zd = 0.0;
    if keyboard.pressed(cc.zoom_in) {
        zd += 1.0
    }
    if keyboard.pressed(cc.zoom_out) {
        zd -= 1.0
    }

    cs.left_mouse_down = mouse.pressed(MouseButton::Left);
    if mouse.just_released(MouseButton::Left)
        && double_click.is_double_click(cc.double_click_window)
    {
        cs.double_click = true;
    }
    let (c, t) = camera.single();
    if let Some(world_mouse_pos) = windows
        .get_single()
        .unwrap()
        .cursor_position()
        .and_then(|cursor| c.viewport_to_world(t, cursor))
        .map(|ray| ray.origin.truncate())
    {
        cs.world_mouse_pos = world_mouse_pos;
    }

    if !cs.save && keyboard.just_pressed(cc.save) {
        cs.save = true;
    }

    if td != Vec2::ZERO {
        cs.translate_delta = td * camera_config.move_modifier;
    }
    if zd != 0.0 {
        cs.zoom_delta = zd * camera_config.zoom_modifier;
    }
}
