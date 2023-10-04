use bevy::{
    math::vec2,
    prelude::{Commands, IntoSystemConfigs, Plugin, Res, Startup, Update},
};

use crate::handles::Handles;

use self::{
    construction_mode::{ConstructionMode, ConstructionModePlugin, Mode},
    constructor::{handle_bone_construction, handle_joint_construction, Constructor},
    drag::{move_dragging, set_draggable},
    icons::{anchor_icons, JointIconBundle},
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
        app.add_systems(Update, (anchor_icons, move_dragging, set_draggable));
        app.add_systems(Update, handle_joint_construction.run_if(construct_joint));
        app.add_systems(Update, handle_bone_construction.run_if(construct_bone));
    }
}

fn construct_joint(cm: Res<ConstructionMode>) -> bool {
    return cm.current_mode == Mode::Joint;
}
fn construct_bone(cm: Res<ConstructionMode>) -> bool {
    return cm.current_mode == Mode::Bone;
}
fn construct_muscle(cm: Res<ConstructionMode>) -> bool {
    return cm.current_mode == Mode::Muscle;
}
