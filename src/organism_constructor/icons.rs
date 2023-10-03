use bevy::{
    math::vec3,
    prelude::{
        default, Bundle, Component, Entity, Handle, Quat, Query, Res, Transform, Vec2, Vec3, With,
        Without,
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
pub enum Anchor {
    Mouse,
    Ent(Entity),
}

#[derive(Bundle)]
pub struct JointIcon {
    icon: DraggableIcon,
    material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    collider: Collider,
    sensor: Sensor,
}
impl JointIcon {
    pub fn new(
        translation: Vec2,
        radius: f32,
        mesh: &Mesh2dHandle,
        material: &Handle<ColorMaterial>,
    ) -> Self {
        return Self {
            icon: DraggableIcon,
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
        };
    }
}

#[derive(Bundle)]
pub struct AnchoredIcon {
    material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    anchored: AnchorSet,
}
impl AnchoredIcon {
    pub fn new(
        width: f32,
        mesh: &Mesh2dHandle,
        material: &Handle<ColorMaterial>,
        anchors: [Anchor; 2],
    ) -> Self {
        return Self {
            material_mesh_bundle: MaterialMesh2dBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: Transform::from_scale(vec3(width, 0.0, 1.0)),
                ..default()
            },
            anchored: AnchorSet { anchors },
        };
    }
}

pub fn anchor_icons(
    mut anchored_icons: Query<(&mut Transform, &AnchorSet), Without<AnchorPoint>>,
    anchor_trans: Query<&Transform, With<AnchorPoint>>,
    cs: Res<ControlState>,
) {
    for (mut t, a) in anchored_icons.iter_mut() {
        let a_pos = match a.anchors[0] {
            Anchor::Mouse => cs.world_mouse_pos,
            Anchor::Ent(e) => anchor_trans.get(e).unwrap().translation.truncate(),
        };
        let b_pos = match a.anchors[1] {
            Anchor::Mouse => cs.world_mouse_pos,
            Anchor::Ent(e) => anchor_trans.get(e).unwrap().translation.truncate(),
        };

        let ab = b_pos - a_pos;
        let len = ab.length();

        t.translation = (a_pos + (ab * 0.5)).extend(-0.3);
        t.rotation = Quat::from_rotation_z(vec2_z_rot(&b_pos, &a_pos));
        t.scale.y = len;
    }
}
