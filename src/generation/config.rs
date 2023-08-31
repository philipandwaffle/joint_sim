use std::time::Duration;

use bevy::prelude::*;

#[derive(Resource)]
pub struct GenerationConfig {
    pub num_organisms: usize,
    pub timer: Timer,
    pub unfreeze_flag: bool,
}
impl GenerationConfig {
    pub fn set_freeze_flag(&mut self, flag: bool) {}
    pub fn get_unfreeze_flag(&self) -> bool {
        return self.unfreeze_flag;
    }
}
