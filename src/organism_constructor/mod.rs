use bevy::prelude::{IntoSystemConfigs, Plugin, Res, Update};

use self::{
    construction_mode::{ConstructionMode, ConstructionModePlugin, Mode},
    constructor::{
        handle_anchored_icon_construction, handle_joint_construction, AnchoredIconConstruction,
        Constructor,
    },
    drag::{move_dragging, set_draggable},
    icons::anchor_icons,
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
        app.insert_resource(AnchoredIconConstruction::default());
        app.add_plugins(ConstructionModePlugin);
        app.add_systems(Update, (anchor_icons, move_dragging, set_draggable));
        app.add_systems(Update, handle_joint_construction.run_if(construct_joint));
        app.add_systems(
            Update,
            handle_anchored_icon_construction.run_if(construct_anchored_icon),
        );
    }
}

fn construct_joint(cm: Res<ConstructionMode>) -> bool {
    return cm.current_mode == Mode::Joint;
}
fn construct_anchored_icon(cm: Res<ConstructionMode>) -> bool {
    return cm.current_mode == Mode::Bone || cm.current_mode == Mode::Muscle;
}
