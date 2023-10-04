use bevy::prelude::{
    default, Children, Commands, DespawnRecursiveExt, Entity, Local, Query, Res, ResMut, Resource,
    With,
};
use bevy_rapier2d::prelude::{QueryFilter, QueryFilterFlags, RapierContext};

use crate::{controls::control_state::ControlState, handles::Handles};

use super::{
    construction_mode::{ConstructionMode, Mode},
    icons::{
        Anchor, AnchorSet, BoneIcon, BoneIconBundle, JointIcon, JointIconBundle, MuscleIconBundle,
    },
    mode_menu::ModeMenuBundle,
};

#[derive(Resource)]
pub struct Constructor {
    part_menu: Option<Entity>,
}
impl Constructor {
    pub fn new() -> Self {
        return Self { part_menu: None };
    }

    pub fn spawn(&mut self, commands: &mut Commands) {
        self.part_menu = Some(ModeMenuBundle::new(commands));
    }

    pub fn despawn(&mut self, commands: &mut Commands) {
        commands.entity(self.part_menu.unwrap()).despawn_recursive();
        self.part_menu = None;
    }
}

pub fn handle_joint_construction(
    mut commands: Commands,
    mut cs: ResMut<ControlState>,
    handles: Res<Handles>,
) {
    if !cs.double_click {
        return;
    }
    cs.double_click = false;

    JointIconBundle::new(
        &mut commands,
        cs.world_mouse_pos,
        10.0,
        &handles.joint_mesh,
        &handles.joint_material,
    );
}

pub struct AnchoredIconConstruction {
    anchored_entity: Option<Entity>,
}
impl Default for AnchoredIconConstruction {
    fn default() -> Self {
        Self {
            anchored_entity: None,
        }
    }
}

pub fn handle_anchored_icon_construction(
    mut commands: Commands,
    joint_icons: Query<&Children, With<JointIcon>>,
    bone_icons: Query<&Children, With<BoneIcon>>,
    mut anchored_icons: Query<&mut AnchorSet>,
    mut cs: ResMut<ControlState>,
    cm: Res<ConstructionMode>,
    handles: Res<Handles>,
    rapier_context: Res<RapierContext>,
    mut bc: Local<AnchoredIconConstruction>,
) {
    if !cs.left_mouse_up {
        return;
    }
    cs.left_mouse_up = false;
    let is_bone = cm.current_mode == Mode::Bone;

    let mut potential_anchor_ent = None;
    rapier_context.intersections_with_point(
        cs.world_mouse_pos,
        QueryFilter {
            flags: QueryFilterFlags::EXCLUDE_SOLIDS,
            ..default()
        },
        |e| {
            let anchor = match is_bone {
                true => joint_icons.get(e),
                false => bone_icons.get(e),
            };
            match anchor {
                Ok(child) => {
                    match child.first() {
                        Some(e) => potential_anchor_ent = Some(*e),
                        None => println!("Joint icon has no anchor point"),
                    }
                    false
                }
                Err(e) => {
                    println!("No joint icon exists here, {:?}", e);
                    true
                }
            }
        },
    );

    if potential_anchor_ent.is_none() {
        return;
    }
    let anchor_ent = potential_anchor_ent.unwrap();

    match bc.anchored_entity {
        Some(anchored_icon_ent) => match anchored_icons.get_mut(anchored_icon_ent) {
            Ok(mut anchor_set) => {
                anchor_set.set_anchor(anchor_ent);
                bc.anchored_entity = None;
            }
            Err(_) => todo!(),
        },
        None => match is_bone {
            true => {
                let bone_icon_ent = BoneIconBundle::new(
                    &mut commands,
                    6.0,
                    &handles.bone_mesh,
                    &handles.bone_material,
                    [Anchor::Ent(anchor_ent), Anchor::Mouse],
                );
                bc.anchored_entity = Some(bone_icon_ent);
            }
            false => {
                let muscle_icon_ent = MuscleIconBundle::new(
                    &mut commands,
                    6.0,
                    &handles.muscle_mesh,
                    &handles.muscle_neutral_material,
                    [Anchor::Ent(anchor_ent), Anchor::Mouse],
                );
                bc.anchored_entity = Some(muscle_icon_ent);
            }
        },
    }
}
