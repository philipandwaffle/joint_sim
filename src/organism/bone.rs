use std::f32::consts::PI;

use bevy::{
    math::vec2,
    prelude::{
        default, BuildChildren, Bundle, Color, Commands, Component, ComputedVisibility, Entity,
        GlobalTransform, Quat, SpatialBundle, Transform, Vec2, Vec3,
    },
    transform::TransformBundle,
};
use bevy_prototype_lyon::{
    prelude::{Fill, GeometryBuilder, ShapeBundle},
    shapes,
};
use bevy_rapier2d::prelude::{
    AdditionalMassProperties, Collider, ExternalImpulse, ImpulseJoint, LockedAxes,
    RevoluteJointBuilder, RigidBody, Sensor,
};

use super::joint::JointBundle;

#[derive(Bundle)]
pub struct BoneBundle {
    bone: Bone,
    spatial_bundle: SpatialBundle,
    rigid_body: RigidBody,
    external_impulse: ExternalImpulse,
    mass: AdditionalMassProperties,
}
impl BoneBundle {
    pub fn new(translation: Vec2) -> Self {
        return Self {
            bone: Bone,
            spatial_bundle: SpatialBundle::from_transform(Transform::from_translation(
                translation.extend(-0.1),
            )),
            rigid_body: RigidBody::Dynamic,
            external_impulse: ExternalImpulse::default(),
            mass: AdditionalMassProperties::Mass(0.5),
        };
    }
}

#[derive(Bundle)]
pub struct BoneDisplayBundle {
    shape_bundle: ShapeBundle,
    fill: Fill,
    collider: Collider,
    sensor: Sensor,
}
impl BoneDisplayBundle {
    pub fn new(len: f32, z_rot: f32) -> Self {
        let bone_width = 3.0;

        let bone_rect = shapes::Rectangle {
            extents: vec2(bone_width, len),
            origin: shapes::RectangleOrigin::Center,
        };

        return Self {
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&bone_rect),
                transform: Transform::from_rotation(Quat::from_rotation_z(z_rot)),
                ..default()
            },
            fill: Fill::color(Color::hsl(360.0, 0.37, 0.84)),
            collider: Collider::cuboid(bone_width * 0.5, (len - 10.0) * 0.5),
            sensor: Sensor,
        };
    }
}

// Component to mark the mid point of a bone
// Used for muscles to pull on
#[derive(Component)]
pub struct Bone;
impl Bone {
    // Development fn for testing new bone
    pub fn spawn(commands: &mut Commands, joints: [Entity; 2], joint_pos: [Vec2; 2]) -> Entity {
        let [a_pos, b_pos] = joint_pos;

        // Create joint
        let ab = b_pos - a_pos;
        let dir = ab * 0.5;
        let len = ab.length();
        let x = if ab.x >= 0.0 { -1.0 } else { 1.0 };
        let z_rot = x * f32::acos(ab.y / len);

        let bone_ent = commands
            .spawn(BoneBundle::new(a_pos + dir))
            .with_children(|p| {
                p.spawn(BoneDisplayBundle::new(len, z_rot));
            })
            .id();

        let bearing_a = RevoluteJointBuilder::new().local_anchor1(-dir).build();
        let bearing_b = RevoluteJointBuilder::new().local_anchor1(dir).build();
        let axel_a = commands.spawn(ImpulseJoint::new(bone_ent, bearing_a)).id();
        let axel_b = commands.spawn(ImpulseJoint::new(bone_ent, bearing_b)).id();

        commands.get_entity(joints[0]).unwrap().add_child(axel_a);
        commands.get_entity(joints[1]).unwrap().add_child(axel_b);
        return bone_ent;
    }
}
