use bevy::{
    math::vec2,
    prelude::{
        default, BuildChildren, Bundle, Color, Commands, Component, Entity, Handle, Quat,
        SpatialBundle, Transform, Vec2,
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::{
    AdditionalMassProperties, Collider, ColliderMassProperties, ExternalImpulse, ImpulseJoint,
    RevoluteJointBuilder, RigidBody, Sensor,
};

#[derive(Bundle)]
pub struct BoneBundle {
    bone: Bone,
    spatial_bundle: SpatialBundle,
    rigid_body: RigidBody,
    external_impulse: ExternalImpulse,
    mass: AdditionalMassProperties,
}
impl BoneBundle {
    pub fn spawn(
        commands: &mut Commands,
        mesh: &Mesh2dHandle,
        material: &Handle<ColorMaterial>,
        joints: [Entity; 2],
        joint_pos: [Vec2; 2],
    ) -> (Entity, Vec2) {
        let width = 3.0;

        let [a_pos, b_pos] = joint_pos;

        // Create joint
        let ab = b_pos - a_pos;
        let dir = ab * 0.5;
        let len = ab.length();
        let x = if ab.x >= 0.0 { -1.0 } else { 1.0 };
        let z_rot = x * f32::acos(ab.y / len);
        let mid = a_pos + dir;

        let display = commands
            .spawn(BoneDisplayBundle::new(
                mesh,
                material,
                vec2(width, len - 10.0),
                z_rot,
            ))
            .id();
        let bone_ent = commands.spawn(BoneBundle::new(mid, 0.0)).id();
        commands.get_entity(bone_ent).unwrap().add_child(display);

        let bearing_a = RevoluteJointBuilder::new().local_anchor1(-dir).build();
        let bearing_b = RevoluteJointBuilder::new().local_anchor1(dir).build();
        let axel_a = commands.spawn(ImpulseJoint::new(bone_ent, bearing_a)).id();
        let axel_b = commands.spawn(ImpulseJoint::new(bone_ent, bearing_b)).id();

        commands.get_entity(joints[0]).unwrap().add_child(axel_a);
        commands.get_entity(joints[1]).unwrap().add_child(axel_b);
        return (bone_ent, mid);
    }

    pub fn new(translation: Vec2, z_rot: f32) -> Self {
        return Self {
            bone: Bone,
            spatial_bundle: SpatialBundle::from_transform(Transform {
                translation: translation.extend(-0.1),
                rotation: Quat::from_rotation_z(z_rot),
                ..default()
            }),
            rigid_body: RigidBody::Dynamic,
            external_impulse: ExternalImpulse::default(),
            mass: AdditionalMassProperties::Mass(1.0),
        };
    }
}

#[derive(Bundle)]
pub struct BoneDisplayBundle {
    material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    collider: Collider,
    sensor: Sensor,
    collider_mass: ColliderMassProperties,
}
impl BoneDisplayBundle {
    pub fn new(
        mesh: &Mesh2dHandle,
        material: &Handle<ColorMaterial>,
        size: Vec2,
        z_rot: f32,
    ) -> Self {
        return Self {
            material_mesh_bundle: MaterialMesh2dBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: Transform {
                    rotation: Quat::from_rotation_z(z_rot),
                    scale: size.extend(0.0),
                    ..default()
                },
                ..default()
            },
            collider_mass: ColliderMassProperties::Density(0.2),
            collider: Collider::cuboid(0.5, 0.5),
            sensor: Sensor,
        };
    }
}

// Component to mark the mid point of a bone
// Used for muscles to pull on
#[derive(Component)]
pub struct Bone;
