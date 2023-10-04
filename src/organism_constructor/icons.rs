use bevy::{
    math::vec3,
    prelude::{
        default, BuildChildren, Bundle, Commands, Component, Entity, GlobalTransform, Handle,
        Parent, Quat, Query, Res, Transform, Vec2, Vec3, With, Without,
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::{Collider, Sensor};

use crate::{controls::control_state::ControlState, organism::helper_fn::vec2_z_rot};

#[derive(Component)]
pub struct DraggableIcon;
#[derive(Component)]
pub struct AnchorPoint;
#[derive(Component)]
pub struct AnchorSet {
    anchors: [Anchor; 2],
}
impl AnchorSet {
    pub fn get_anchor_pos(
        &self,
        anchors: &Query<&Parent, With<AnchorPoint>>,
        joint_trans: &Query<&GlobalTransform>,
        mp: &Vec2,
    ) -> Option<[Vec2; 2]> {
        let a_pos = self.anchors[0].get_anchor_pos(anchors, joint_trans, mp);
        let b_pos = self.anchors[1].get_anchor_pos(anchors, joint_trans, mp);
        if a_pos.is_none() || b_pos.is_none() {
            return None;
        }
        return Some([a_pos.unwrap(), b_pos.unwrap()]);
    }

    pub fn set_anchor(&mut self, e: Entity) {
        self.anchors[1] = Anchor::Ent(e);
    }
}
pub enum Anchor {
    Mouse,
    Ent(Entity),
}
impl Anchor {
    pub fn get_anchor_pos(
        &self,
        anchors: &Query<&Parent, With<AnchorPoint>>,
        joint_trans: &Query<&GlobalTransform>,
        mp: &Vec2,
    ) -> Option<Vec2> {
        return match self {
            Anchor::Mouse => Some(mp.clone()),
            Anchor::Ent(e) => match anchors.get(*e) {
                Ok(joint) => match joint_trans.get(joint.get()) {
                    Ok(gt) => return Some(gt.translation().truncate()),
                    Err(e) => {
                        println!("Joint anchor was somehow orphaned {:?}", e);
                        return None;
                    }
                },
                Err(e) => {
                    println!("Anchor entity doesn't exist {:?}", e);
                    return None;
                }
            },
        };
    }
}

#[derive(Component)]
pub struct JointIcon;
#[derive(Bundle)]
pub struct JointIconBundle {
    joint_icon: JointIcon,
    draggable_icon: DraggableIcon,
    material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    collider: Collider,
    sensor: Sensor,
}
impl JointIconBundle {
    pub fn new(
        commands: &mut Commands,
        translation: Vec2,
        radius: f32,
        mesh: &Mesh2dHandle,
        material: &Handle<ColorMaterial>,
    ) -> Entity {
        return commands
            .spawn(Self {
                joint_icon: JointIcon,
                draggable_icon: DraggableIcon,
                material_mesh_bundle: MaterialMesh2dBundle {
                    mesh: mesh.clone(),
                    material: material.clone(),
                    transform: Transform {
                        translation: translation.extend(0.3),
                        scale: Vec3::ONE * radius,
                        ..default()
                    },
                    ..default()
                },
                collider: Collider::ball(1.0),
                sensor: Sensor,
            })
            .with_children(|j| {
                j.spawn(AnchorPoint);
            })
            .id();
    }
}

#[derive(Bundle)]
pub struct AnchoredIconBundle {
    material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    anchored: AnchorSet,
}
impl AnchoredIconBundle {
    pub fn new(
        width: f32,
        z_pos: f32,
        mesh: &Mesh2dHandle,
        material: &Handle<ColorMaterial>,
        anchors: [Anchor; 2],
    ) -> Self {
        return Self {
            material_mesh_bundle: MaterialMesh2dBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: Transform {
                    translation: vec3(0.0, 0.0, z_pos),
                    scale: vec3(width, 0.0, 1.0),
                    ..default()
                },

                ..default()
            },
            anchored: AnchorSet { anchors },
        };
    }
}

#[derive(Component)]
pub struct BoneIcon;
#[derive(Bundle)]
pub struct BoneIconBundle {
    bone_icon: BoneIcon,
    anchored_icon_bundle: AnchoredIconBundle,
    collider: Collider,
    sensor: Sensor,
}
impl BoneIconBundle {
    pub fn new(
        commands: &mut Commands,
        width: f32,
        mesh: &Mesh2dHandle,
        material: &Handle<ColorMaterial>,
        anchors: [Anchor; 2],
    ) -> Entity {
        return commands
            .spawn(Self {
                bone_icon: BoneIcon,
                anchored_icon_bundle: AnchoredIconBundle::new(width, -0.1, mesh, material, anchors),
                collider: Collider::cuboid(0.5, 0.4),
                sensor: Sensor,
            })
            .with_children(|bone| {
                bone.spawn(AnchorPoint);
            })
            .id();
    }
}

#[derive(Bundle)]
pub struct MuscleIconBundle {
    anchored_icon_bundle: AnchoredIconBundle,
}
impl MuscleIconBundle {
    pub fn new(
        commands: &mut Commands,
        width: f32,
        mesh: &Mesh2dHandle,
        material: &Handle<ColorMaterial>,
        anchors: [Anchor; 2],
    ) -> Entity {
        return commands
            .spawn(Self {
                anchored_icon_bundle: AnchoredIconBundle::new(width, -0.2, mesh, material, anchors),
            })
            .id();
    }
}

pub fn anchor_icons(
    mut anchored_icons: Query<(&mut Transform, &AnchorSet), Without<AnchorPoint>>,
    anchors: Query<&Parent, With<AnchorPoint>>,
    joint_trans: Query<&GlobalTransform>,
    cs: Res<ControlState>,
) {
    for (mut t, a) in anchored_icons.iter_mut() {
        let [a_pos, b_pos] = match a.get_anchor_pos(&anchors, &joint_trans, &cs.world_mouse_pos) {
            Some(anchor_pos) => anchor_pos,
            None => return,
        };

        let ab = b_pos - a_pos;
        let len = ab.length();

        let z_pos = t.translation.z;
        t.translation = (a_pos + (ab * 0.5)).extend(z_pos);
        t.rotation = Quat::from_rotation_z(vec2_z_rot(&b_pos, &a_pos));
        t.scale.y = len;
    }
}
