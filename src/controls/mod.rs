use bevy::prelude::{App, Plugin, Startup, Update};

use self::{
    camera::{spawn_cam, translate_cam},
    control_state::{update_control_state, Bindings, ControlState},
    save::save_generation,
};

pub mod camera;
pub mod control_state;
pub mod save;

pub struct ControlPlugin;
impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ControlState::default())
            .insert_resource(Bindings::default())
            .add_systems(Startup, spawn_cam)
            .add_systems(
                Update,
                (update_control_state, translate_cam, save_generation),
            );
    }
}
