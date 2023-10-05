use bevy::{
    a11y::accesskit::Vec2,
    prelude::{
        default, Children, Commands, DespawnRecursiveExt, Entity, Query, Res, ResMut, Resource,
        With,
    },
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
    joints: Vec<(Entity, Vec2)>,
    bones: Vec<(Entity, [u32; 2])>,
    muscles: Vec<(Entity, [u32; 2])>,
}
impl Default for Constructor {
    fn default() -> Self {
        return Self {
            part_menu: None,
            joints: vec![],
            bones: vec![],
            muscles: vec![],
        };
    }
}
impl Constructor {
    pub fn spawn(&mut self, commands: &mut Commands) {
        self.part_menu = Some(ModeMenuBundle::new(commands));
    }
    pub fn despawn(&mut self, commands: &mut Commands) {
        commands.entity(self.part_menu.unwrap()).despawn_recursive();
        self.part_menu = None;
        for (e, _) in self.joints.iter() {
            commands.entity(*e).despawn_recursive();
        }
        for (e, _) in self.bones.iter() {
            commands.entity(*e).despawn_recursive();
        }
        for (e, _) in self.muscles.iter() {
            commands.entity(*e).despawn_recursive();
        }
    }

    pub fn push_joint(&mut self, e: Entity, pos: Vec2) {
        self.joints.push((e, pos));
    }

    pub fn remove_joint(&mut self, e: Entity) {
        match self.joints.iter().position(|x| x.0 == e) {
            Some(i) => {
                // self.joints.push(e);
            }
            None => return,
        }
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

#[derive(Resource)]
pub struct AnchoredIconConstruction {
    anchored_entity: Option<Entity>,
}
impl AnchoredIconConstruction {
    pub fn clear(&mut self, commands: &mut Commands) {
        match self.anchored_entity {
            Some(e) => {
                commands.entity(e).despawn();
                self.anchored_entity = None;
            }
            None => return,
        }
    }
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
    mut aic: ResMut<AnchoredIconConstruction>,
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

    match aic.anchored_entity {
        Some(anchored_icon_ent) => match anchored_icons.get_mut(anchored_icon_ent) {
            Ok(mut anchor_set) => {
                anchor_set.set_anchor(anchor_ent);
                aic.anchored_entity = None;
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
                aic.anchored_entity = Some(bone_icon_ent);
            }
            false => {
                let muscle_icon_ent = MuscleIconBundle::new(
                    &mut commands,
                    6.0,
                    &handles.muscle_mesh,
                    &handles.muscle_neutral_material,
                    [Anchor::Ent(anchor_ent), Anchor::Mouse],
                );
                aic.anchored_entity = Some(muscle_icon_ent);
            }
        },
    }
}
