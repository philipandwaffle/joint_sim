use bevy::{
    asset::Error,
    math::vec2,
    prelude::{
        default, Children, Commands, DespawnRecursiveExt, Entity, Parent, Query, Res, ResMut,
        Resource, Transform, Vec2, With,
    },
};
use bevy_rapier2d::prelude::{QueryFilter, QueryFilterFlags, RapierContext};

use crate::{
    controls::control_state::ControlState, handles::Handles, organism::organism::OrganismBuilder,
};

use super::{
    construction_mode::{ConstructionMode, Mode},
    construction_zone::ConstructionZone,
    icons::{
        Anchor, AnchorPoint, AnchorSet, BoneIcon, BoneIconBundle, JointIcon, JointIconBundle,
        MuscleIcon, MuscleIconBundle,
    },
    mode_menu::ModeMenuBundle,
};

#[derive(Resource)]
pub struct Constructor {
    part_menu: Option<Entity>,
    construction_zone: Option<Entity>,
    pub in_bounds: bool,
    joints: Vec<Entity>,
    bones: Vec<Entity>,
    muscles: Vec<Entity>,
}
impl Default for Constructor {
    fn default() -> Self {
        return Self {
            part_menu: None,
            construction_zone: None,
            in_bounds: false,
            joints: vec![],
            bones: vec![],
            muscles: vec![],
        };
    }
}
impl Constructor {
    pub fn spawn(&mut self, commands: &mut Commands) {
        self.part_menu = Some(ModeMenuBundle::new(commands));
        self.construction_zone = Some(ConstructionZone::new(commands))
    }
    pub fn despawn(&mut self, commands: &mut Commands) {
        commands.entity(self.part_menu.unwrap()).despawn_recursive();
        commands
            .entity(self.construction_zone.unwrap())
            .despawn_recursive();

        self.part_menu = None;
        for e in self.joints.iter() {
            commands.entity(*e).despawn_recursive();
        }
        for e in self.bones.iter() {
            commands.entity(*e).despawn();
        }
        for e in self.muscles.iter() {
            commands.entity(*e).despawn();
        }
    }

    pub fn create_builder(
        &self,
        joint_icons: &Query<(&Transform, &JointIcon)>,
        anchors: &Query<&Parent, With<AnchorPoint>>,
        bone_anchors: &Query<(&AnchorSet, &BoneIcon)>,
        muscle_anchors: &Query<(&AnchorSet, &MuscleIcon)>,
    ) -> Result<OrganismBuilder, Error> {
        let mut joint_pos = vec![Vec2::ZERO; self.joints.len()];
        let mut bones = vec![[0, 0]; self.bones.len()];
        let mut muscles = vec![[0, 0]; self.muscles.len()];

        for (t, j_i) in joint_icons {
            joint_pos[j_i.id] = t.translation.truncate();
        }
        let x_offset = joint_pos.iter().map(|pos| pos.x).sum::<f32>() / joint_pos.len() as f32;
        let y_offset = joint_pos
            .iter()
            .map(|pos| pos.y)
            .min_by(|a, b| a.total_cmp(b))
            .unwrap();
        let offset = vec2(x_offset, y_offset);
        joint_pos = joint_pos
            .iter()
            .map(|pos| *pos - offset)
            .collect::<Vec<Vec2>>();

        for (a_s, b_i) in bone_anchors.iter() {
            // is this a fucked mapping and is this readable?
            bones[b_i.id] = joint_icons
                .get_many(anchors.get_many(a_s.get_ents()?)?.map(|p| p.get()))?
                .map(|(_, j_i)| j_i.id);
        }

        for (a_s, m_i) in muscle_anchors.iter() {
            // is this a fucked mapping and is this readable?
            muscles[m_i.id] = bone_anchors
                .get_many(anchors.get_many(a_s.get_ents()?)?.map(|p| p.get()))?
                .map(|(_, b_i)| b_i.id);
        }

        println!("joints {:?}", joint_pos);
        println!("bones {:?}", bones);
        println!("muscles {:?}", muscles);
        return Ok(OrganismBuilder::new(
            1,
            vec![6, 6, 6],
            joint_pos,
            bones,
            muscles,
        ));
    }
}

pub fn handle_joint_construction(
    mut commands: Commands,
    mut c: ResMut<Constructor>,
    mut cs: ResMut<ControlState>,
    handles: Res<Handles>,
) {
    if !cs.double_click {
        return;
    }
    cs.double_click = false;

    let joint_ent = JointIconBundle::new(
        &mut commands,
        c.joints.len(),
        cs.world_mouse_pos,
        5.0,
        &handles.joint_mesh,
        &handles.joint_material,
    );
    c.joints.push(joint_ent);
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
    mut c: ResMut<Constructor>,
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
                match is_bone {
                    true => c.bones.push(anchored_icon_ent),
                    false => c.muscles.push(anchored_icon_ent),
                }
                anchor_set.set_anchor(anchor_ent);
                aic.anchored_entity = None;
            }
            Err(_) => todo!(),
        },
        None => match is_bone {
            true => {
                let bone_icon_ent = BoneIconBundle::new(
                    &mut commands,
                    c.bones.len(),
                    3.0,
                    &handles.bone_mesh,
                    &handles.bone_material,
                    [Anchor::Ent(anchor_ent), Anchor::Mouse],
                );
                aic.anchored_entity = Some(bone_icon_ent);
            }
            false => {
                let muscle_icon_ent = MuscleIconBundle::new(
                    &mut commands,
                    c.muscles.len(),
                    3.0,
                    &handles.muscle_mesh,
                    &handles.muscle_neutral_material,
                    [Anchor::Ent(anchor_ent), Anchor::Mouse],
                );
                aic.anchored_entity = Some(muscle_icon_ent);
            }
        },
    }
}
