use bevy::prelude::Plugin;

use crate::config::structs::Config;

pub mod structs;

pub struct ConfigPlugin;
impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let mut c = Config::load_cfg("settings.cfg");
        c.generation.reset_timer();
        app.insert_resource(c.generation).insert_resource(c.camera);
    }
}
