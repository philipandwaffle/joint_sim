use bevy::prelude::{Plugin, Update};

use self::body::handle_bodies;

pub mod body;
pub mod bone;
pub mod brain;
pub mod joint;
pub mod muscle;

pub struct OrganismPlugin;
impl Plugin for OrganismPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, handle_bodies);
    }
}
