use bevy::prelude::{resource_exists, IntoSystemConfigs, Plugin, Res, Update};

use self::organism_list::{unfreeze_queued, update_brains, update_muscles, OrganismList};

pub mod bone;
pub mod brain;
pub mod genome;
pub mod handles;
pub mod helper_fn;
pub mod joint;
pub mod muscle;
pub mod organism;
pub mod organism_list;

// Plugin to handle organisms
pub struct OrganismPlugin;
impl Plugin for OrganismPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (update_brains, update_muscles, unfreeze_queued)
                .run_if(resource_exists::<OrganismList>()),
        );
    }
}
