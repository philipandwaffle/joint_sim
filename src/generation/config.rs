use bevy::{prelude::Resource, time::Timer};

#[derive(Resource)]
pub struct GenerationConfig {
    pub num_organisms: usize,
    pub timer: Timer,
    pub unfreeze_flag: bool,
}
