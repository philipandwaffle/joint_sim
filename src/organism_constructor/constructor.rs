use std::arch::x86_64::_andn_u32;

use bevy::prelude::{
    default, BuildChildren, Commands, DespawnRecursiveExt, Entity, GlobalTransform, Res, ResMut,
    Resource, Transform,
};
use bevy_rapier2d::prelude::{QueryFilter, QueryFilterFlags, RapierContext};

use crate::{
    controls::control_state::ControlState,
    handles::{self, Handles},
};

use super::{
    construction_mode::{ConstructionMode, Mode},
    icons::{Anchor, AnchorPoint, AnchoredIcon, JointIcon},
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
    rapier_context: Res<RapierContext>,
) {
    match cm.current_mode {
        Mode::None => {}
        Mode::Joint => {
            if !cs.double_click {
                return;
            }
            cs.double_click = false;

            commands.spawn(JointIcon::new(
                cs.world_mouse_pos,
                10.0,
                &handles.joint_mesh,
                &handles.joint_material,
            ));
        }
        Mode::Bone => {
            if !cs.double_click {
                return;
            }
            cs.double_click = false;

            let mut anchor_ent = Entity::PLACEHOLDER;
            rapier_context.intersections_with_point(
                cs.world_mouse_pos,
                QueryFilter {
                    flags: QueryFilterFlags::EXCLUDE_SOLIDS,
                    ..default()
                },
                |e| match commands.get_entity(e) {
                    Some(mut joint_ent) => {
                        joint_ent.with_children(|joint| {
                            anchor_ent = joint.spawn((AnchorPoint, Transform::default())).id();
                        });
                        false
                    }
                    None => true,
                },
            );

            if anchor_ent == Entity::PLACEHOLDER {
                println!("Anchor entity has been created without spawn");
            }

            commands.spawn(AnchoredIcon::new(
                6.0,
                &handles.bone_mesh,
                &handles.bone_material,
                [Anchor::Ent(anchor_ent), Anchor::Mouse],
            ));
        }
        Mode::Muscle => {}
    }
}
