use bevy::prelude::{Plugin, Update};

use self::{body::update_bodies, bone::apply_motor_impulse};

pub mod body;
pub mod bone;
pub mod joint;
pub mod muscle;

pub struct OrganismPlugin;
impl Plugin for OrganismPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, (apply_motor_impulse, update_bodies));
    }
}
