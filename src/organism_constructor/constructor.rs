use std::arch::x86_64::_andn_u32;

use bevy::{
    asset::Error,
    prelude::{
        default, BuildChildren, Children, Commands, DespawnRecursiveExt, Entity, GlobalTransform,
        Local, Query, Res, ResMut, Resource, Transform, With,
    },
    transform::TransformBundle,
};
use bevy_rapier2d::prelude::{QueryFilter, QueryFilterFlags, RapierContext};

use crate::{
    controls::control_state::ControlState,
    handles::{self, Handles},
};

use super::{
    construction_mode::{ConstructionMode, Mode},
    icons::{Anchor, AnchorPoint, AnchorSet, AnchoredIcon, JointIcon, JointIconBundle},
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

pub struct BoneConstruction {
    bone_on_mouse: bool,
    anchored_entity: Option<Entity>,
}
impl Default for BoneConstruction {
    fn default() -> Self {
        Self {
            bone_on_mouse: false,
            anchored_entity: None,
        }
    }
}

pub fn handle_bone_construction(
    mut commands: Commands,
    joints: Query<&Children, With<JointIcon>>,
    mut bone_icons: Query<&mut AnchorSet>,
    mut cs: ResMut<ControlState>,
    handles: Res<Handles>,
    rapier_context: Res<RapierContext>,
    mut bc: Local<BoneConstruction>,
) {
    if !cs.left_mouse_up {
        return;
    }
    cs.left_mouse_up = false;

    let mut potential_anchor_ent = None;
    rapier_context.intersections_with_point(
        cs.world_mouse_pos,
        QueryFilter {
            flags: QueryFilterFlags::EXCLUDE_SOLIDS,
            ..default()
        },
        |e| match joints.get(e) {
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
        },
    );

    if potential_anchor_ent.is_none() {
        return;
    }
    let anchor_ent = potential_anchor_ent.unwrap();

    match bc.anchored_entity {
        Some(bone_icon_ent) => match bone_icons.get_mut(bone_icon_ent) {
            Ok(mut anchor_set) => {
                anchor_set.set_anchor(anchor_ent);
                bc.anchored_entity = None;
            }
            Err(_) => todo!(),
        },
        None => {
            let bone_icon_ent = commands
                .spawn(AnchoredIcon::new(
                    6.0,
                    &handles.bone_mesh,
                    &handles.bone_material,
                    [Anchor::Ent(anchor_ent), Anchor::Mouse],
                ))
                .id();
            bc.anchored_entity = Some(bone_icon_ent);
        }
    }
}
