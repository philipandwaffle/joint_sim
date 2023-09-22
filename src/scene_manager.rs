use bevy::prelude::App;

enum Scene {
    StartMenu,
    OrganismConstructor,
    OrganismSimulation,
}
impl Scene {
    fn pre_change(&self, app: &mut App) {}
    fn post_change(&self, app: &mut App) {}
}
