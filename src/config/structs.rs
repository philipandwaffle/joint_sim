use bevy::prelude::*;
use bevy::{
    prelude::Resource,
    time::{Timer, TimerMode},
};
use core::panic;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

#[derive(Resource, Debug, Default, Serialize, Deserialize, Clone)]
pub struct CameraConfig {
    pub move_modifier: f32,
    pub zoom_modifier: f32,
}
#[derive(Resource, Debug, Default, Serialize, Deserialize, Clone)]
pub struct GenerationConfig {
    pub num_organisms: usize,
    pub vertical_sep: f32,
    pub generation_duration: f32,
    #[serde(skip_serializing, skip_deserializing)]
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
            generation_duration,
            timer: Timer::new(
                Duration::from_secs_f32(generation_duration),
                TimerMode::Once,
            ),
            unfreeze_flag,
            debug_flag,
        };
    }

    pub fn reset_timer(&mut self) {
        self.timer = Timer::new(
            Duration::from_secs_f32(self.generation_duration),
            TimerMode::Once,
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Config {
    pub camera: CameraConfig,
    pub generation: GenerationConfig,
}
impl Config {
    pub fn insert_config_resources(&self, app: &mut App) {
        app.insert_resource(self.camera.clone())
            .insert_resource(self.generation.clone());
    }
    
    pub fn load_cfg(path: &str) -> Config {
        let file = File::open(path);
        if let Err(err) = file {
            panic!("Error opening file {:?}", err);
        }

        let reader = BufReader::new(file.unwrap());

        let json: Result<Config, serde_json::Error> = serde_json::from_reader(reader);
        match json {
            Ok(cfg) => return cfg,
            Err(err) => panic!("Error reading JSON {:?}", err),
        }
    }
}
