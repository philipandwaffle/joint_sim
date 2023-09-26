use bevy::prelude::{Commands, DespawnRecursiveExt, Entity, Resource};

use super::part_menu::{self, PartMenuBundle};

#[derive(Resource)]
pub struct Constructor {
    part_menu: Option<Entity>,
}
impl Constructor {
    pub fn new() -> Self {
        return Self { part_menu: None };
    }

    pub fn spawn(&mut self, commands: &mut Commands) {
        self.part_menu = Some(PartMenuBundle::new(commands));
    }

    pub fn despawn(&mut self, commands: &mut Commands) {
        commands.entity(self.part_menu.unwrap()).despawn_recursive();
        self.part_menu = None;
    }
}
