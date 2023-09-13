use bevy::{
    prelude::{App, Resource, Vec2, Vec3},
    time::{Timer, TimerMode},
};
use core::panic;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

use crate::organism::genome::Genome;

#[derive(Resource, Debug, Default, Serialize, Deserialize, Clone)]
pub struct CameraConfig {
    pub move_modifier: f32,
    pub zoom_modifier: f32,
    pub starting_translation: Vec2,
    pub starting_zoom: f32,
}
#[derive(Resource, Debug, Default, Serialize, Deserialize, Clone)]
pub struct GenerationConfig {
    pub num_organisms: usize,
    pub vertical_sep: f32,
    pub generation_duration: f32,
    pub cur_generation: u32,
    #[serde(skip_serializing, skip_deserializing)]
    pub timer: Timer,
    pub unfreeze_flag: bool,
    pub debug_flag: bool,
}
impl GenerationConfig {
    pub fn reset_timer(&mut self) {
        self.timer = Timer::new(
            Duration::from_secs_f32(self.generation_duration),
            TimerMode::Once,
        )
    }
}

#[derive(Resource, Debug, Default, Serialize, Deserialize, Clone)]
pub struct SaveConfig {
    pub folder: String,
    pub rate: u32,
    pub load_save: bool,
    pub load_file: String,
}

// defunct for now
#[derive(Resource, Debug, Default, Serialize, Deserialize, Clone)]
pub struct GenomeConfig {
    pub default: Genome,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Config {
    pub camera: CameraConfig,
    pub generation: GenerationConfig,
    pub save: SaveConfig,
    // pub starting: GenomeConfig,
}
impl Config {
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
