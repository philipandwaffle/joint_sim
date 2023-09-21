use bevy::prelude::{App, Plugin, Startup, Update};

use self::{
    camera::{spawn_cam, translate_cam},
    control_state::{update_control_state, Bindings, ControlState},
    save::save_generation,
    ui::ui_test,
};

pub mod camera;
mod construction_grid;
pub mod control_state;
pub mod save;
mod ui;

pub struct ControlPlugin;
impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ControlState::default())
            .insert_resource(Bindings::default())
            .add_systems(Startup, (spawn_cam /*ui_test*/,))
            .add_systems(
                Update,
                (update_control_state, translate_cam, save_generation),
            );
    }
}
