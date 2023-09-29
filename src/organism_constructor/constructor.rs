use bevy::prelude::{Commands, DespawnRecursiveExt, Entity, Res, ResMut, Resource};

use crate::{controls::control_state::ControlState, handles::Handles};

use super::{
    construction_mode::ConstructionMode,
    mode_menu::{self, ModeMenuBundle},
};

#[derive(Resource)]
pub struct Constructor {
    part_menu: Option<Entity>,
}
impl Constructor {
    pub fn new() -> Self {
        return Self { part_menu: None };
    }

    pub fn spawn(&mut self, commands: &mut Commands, handles: &Handles) {
        self.part_menu = Some(ModeMenuBundle::new(commands, handles));
    }

    pub fn despawn(&mut self, commands: &mut Commands) {
        commands.entity(self.part_menu.unwrap()).despawn_recursive();
        self.part_menu = None;
    }
}

pub fn handle_construction(cm: Res<ConstructionMode>, mut cs: ResMut<ControlState>) {
    if cs.double_click {
        cs.double_click = false;
        println!("hello");
    }
}
