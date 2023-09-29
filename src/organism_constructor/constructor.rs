use bevy::prelude::{Commands, DespawnRecursiveExt, Entity, Res, ResMut, Resource};

use crate::{
    controls::control_state::ControlState,
    handles::{self, Handles},
};

use super::{
    construction_mode::{ConstructionMode, Mode},
    icons::JointIcon,
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

pub fn handle_construction(
    mut commands: Commands,
    cm: Res<ConstructionMode>,
    mut cs: ResMut<ControlState>,
    handles: Res<Handles>,
) {
    match cm.current_mode {
        Mode::None => {}
        Mode::Joint => {
            if cs.double_click {
                cs.double_click = false;

                commands.spawn(JointIcon::new(
                    cs.world_mouse_pos,
                    10.0,
                    &handles.joint_mesh,
                    &handles.joint_material,
                ));
            }
        }
        Mode::Bone => {}
        Mode::Muscle => {}
    }
}
