use bevy::prelude::{Plugin, Update};

use self::organism::{update_brains, update_muscles};

pub mod bone;
pub mod brain;
pub mod joint;
pub mod muscle;
pub mod organism;

pub struct OrganismPlugin;
impl Plugin for OrganismPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, (update_brains, update_muscles));
    }
}
