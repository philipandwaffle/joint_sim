use bevy::{
    ecs::system::SystemParam,
    prelude::{Commands, Component, Plugin, Query},
};
use bevy_rapier2d::prelude::{BevyPhysicsHooks, NoUserData};

// just some skele code to maybe setup layers at some point
// check here to see an example
// https://github.com/dimforge/bevy_rapier/blob/a149ff59933f26869482fa3797d1188afecde750/bevy_rapier3d/examples/contact_filter3.rs#L19

pub struct ColliderLayerPlugin;
impl Plugin for ColliderLayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        todo!()
    }
}

#[derive(Component)]
pub struct Layer {}

#[derive(SystemParam)]
pub struct ColliderLayerHook<'w, 's> {
    tags: Query<'w, 's, &'static Layer>,
}
impl BevyPhysicsHooks for ColliderLayerHook<'_, '_> {
    fn filter_contact_pair(
        &self,
        _context: bevy_rapier2d::prelude::PairFilterContextView,
    ) -> Option<bevy_rapier2d::prelude::SolverFlags> {
        None
    }

    fn filter_intersection_pair(
        &self,
        _context: bevy_rapier2d::prelude::PairFilterContextView,
    ) -> bool {
        false
    }

    fn modify_solver_contacts(
        &self,
        _context: bevy_rapier2d::prelude::ContactModificationContextView,
    ) {
    }
}

fn setup_physics(mut commands: Commands) {}
