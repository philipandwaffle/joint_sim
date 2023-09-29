use bevy::{
    math::vec2,
    prelude::{Commands, Plugin, Res, Startup, Update},
};

use crate::handles::Handles;

use self::{
    construction_mode::{ConstructionMode, ConstructionModePlugin},
    constructor::{handle_construction, Constructor},
    drag::{move_dragging, set_draggable},
    icons::JointIcon,
};

mod construction_grid;
mod construction_mode;
pub mod constructor;
mod drag;
mod icons;
pub mod mode_menu;

pub struct OrganismConstructionPlugin;
impl Plugin for OrganismConstructionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Constructor::new());
        app.add_plugins(ConstructionModePlugin);
        app.add_systems(Update, (handle_construction, move_dragging, set_draggable));
    }
}
