use bevy::prelude::{Plugin, Update};

use self::{
    body::{update_bodies, update_linear_muscle_velocities},
    bone::apply_motor_impulse,
    muscle::apply_linear_velocity_delta,
};

pub mod body;
pub mod bone;
pub mod joint;
pub mod muscle;

pub struct OrganismPlugin;
impl Plugin for OrganismPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // app.add_systems(
        //     Update,
        //     (apply_linear_velocity_delta, update_linear_muscle_velocities),
        // );
    }
}
