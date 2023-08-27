use bevy::prelude::{Plugin, Update};

pub mod body;
pub mod bone;
pub mod joint;
pub mod muscle;

pub struct OrganismPlugin;
impl Plugin for OrganismPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {}
}
