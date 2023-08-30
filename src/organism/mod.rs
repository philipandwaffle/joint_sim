use bevy::prelude::{resource_exists, IntoSystemConfigs, Plugin, Update};

use self::organism::{freeze_queued, update_brains, update_muscles, OrganismList};

pub mod bone;
pub mod brain;
pub mod joint;
pub mod muscle;
pub mod organism;

pub struct OrganismPlugin;
impl Plugin for OrganismPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (update_brains, update_muscles, freeze_queued)
                .run_if(resource_exists::<OrganismList>()),
        );
    }
}
