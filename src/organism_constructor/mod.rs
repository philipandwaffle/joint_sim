use bevy::{
    math::vec2,
    prelude::{Commands, Plugin, Res, Startup, Update},
};

use crate::handles::Handles;

use self::{
    drag::{move_dragging, set_draggable},
    icons::JointIcon,
};

mod construction_grid;
mod drag;
mod icons;
pub mod part_menu;

pub struct OrganismConstructionPlugin;
impl Plugin for OrganismConstructionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_test);
        app.add_systems(Update, (move_dragging, set_draggable));
    }
}

fn setup_test(mut commands: Commands, handles: Res<Handles>) {
    commands.spawn(JointIcon::new(
        vec2(1000.0, 100.0),
        &handles.joint_mesh,
        &handles.joint_material,
    ));
}
