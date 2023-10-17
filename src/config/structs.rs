use bevy::{
    math::vec2,
    prelude::{Resource, Vec2},
    time::{Timer, TimerMode},
};
use core::panic;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

use crate::organism::genome::Genome;

#[derive(Resource, Debug, Serialize, Deserialize, Clone)]
pub struct CameraConfig {
    pub move_modifier: f32,
    pub zoom_modifier: f32,
    pub starting_translation: Vec2,
    pub starting_zoom: f32,
}
impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            move_modifier: 5.0,
            zoom_modifier: 0.5,
            starting_translation: vec2(1720.0, 1000.0),
            starting_zoom: 2.0,
        }
    }
}

#[derive(Resource, Debug, Serialize, Deserialize, Clone)]
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
impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            num_organisms: 500,
            vertical_sep: 200.0,
            generation_duration: 20.0,
            cur_generation: 0,
            timer: Default::default(),
            unfreeze_flag: true,
            debug_flag: false,
        }
    }
}
impl GenerationConfig {
    pub fn reset_timer(&mut self) {
        self.timer = Timer::new(
            Duration::from_secs_f32(self.generation_duration),
            TimerMode::Once,
        )
    }
}

#[derive(Resource, Debug, Serialize, Deserialize, Clone)]
pub struct SaveConfig {
    pub enable: bool,
    pub folder: String,
    pub rate: u32,
    pub load_save: bool,
    pub load_file: String,
}
impl Default for SaveConfig {
    fn default() -> Self {
        Self {
            enable: false,
            folder: "saved_generations/test_2".to_string(),
            rate: 20,
            load_save: false,
            load_file: "saved_generations/test_0/14-09-2023_23-24_gen251.json".to_string(),
        }
    }
}

// defunct for now
#[derive(Resource, Debug, Serialize, Deserialize, Clone)]
pub struct GenomeConfig {
    pub default: Genome,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub camera: CameraConfig,
    pub generation: GenerationConfig,
    // pub control: ControlConfig,
    pub save: SaveConfig,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            camera: Default::default(),
            generation: Default::default(),
            save: Default::default(),
        }
    }
}
impl Config {
    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    pub fn load_cfg(path: &str) -> Config {
        return Config::default();
    }

    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
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
