use bevy::prelude::{Plugin, Update};

use self::drag::{drag_draggable, set_draggable};

mod drag;
mod icons;

pub struct OrganismConstructionPlugin;
impl Plugin for OrganismConstructionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, (drag_draggable, set_draggable));
    }
}
