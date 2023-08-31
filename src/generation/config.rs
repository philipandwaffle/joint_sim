use std::time::Duration;

use bevy::{
    prelude::Resource,
    time::{Timer, TimerMode},
};

#[derive(Resource)]
pub struct GenerationConfig {
    pub num_organisms: usize,
    pub vertical_sep: f32,
    pub timer: Timer,
    pub unfreeze_flag: bool,
    pub debug_flag: bool,
}
impl GenerationConfig {
    pub fn new(
        num_organisms: usize,
        vertical_sep: f32,
        generation_duration: f32,
        unfreeze_flag: bool,
        debug_flag: bool,
    ) -> Self {
        return Self {
            num_organisms,
            vertical_sep,
            timer: Timer::new(
                Duration::from_secs_f32(generation_duration),
                TimerMode::Once,
            ),
            unfreeze_flag,
            debug_flag,
        };
    }
}
