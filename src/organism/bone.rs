use std::f32::consts::PI;

use bevy::{
    math::vec2,
    prelude::{
        default, BuildChildren, Color, Commands, ComputedVisibility, Entity, GlobalTransform, Quat,
        Transform, Vec2, Vec3,
    },
};
use bevy_prototype_lyon::{
    prelude::{Fill, GeometryBuilder, ShapeBundle},
    shapes,
};
use bevy_rapier2d::prelude::{Collider, ImpulseJoint, LockedAxes, RevoluteJointBuilder, RigidBody};

use super::joint::JointBundle;

pub struct Bone;
impl Bone {
    // Create a new bone
    fn old(commands: &mut Commands, joints: [Entity; 2], joint_pos: [Vec2; 2]) -> Entity {
        let [a_pos, b_pos] = joint_pos;

        // Create joint
        let ab = b_pos - a_pos;
        let joint_ab = RevoluteJointBuilder::new()
            .local_anchor1(ab)
            .local_anchor2(Vec2::ZERO)
            .build();
        let impulse_joint = commands.spawn(ImpulseJoint::new(joints[0], joint_ab)).id();
        let len = ab.length();

        // Add impulse joint as child
        commands
            .get_entity(joints[1])
            .unwrap()
            .add_child(impulse_joint);

        let x = if ab.x >= 0.0 { -1.0 } else { 1.0 };
        let z_rot = x * f32::acos(ab.y / len);
        let bone_width = 3.0;
        let bone_rect = shapes::Rectangle {
            extents: vec2(bone_width, len),
            origin: shapes::RectangleOrigin::CustomCenter(vec2(0.0, -0.5 * len)),
            ..default()
        };

        let bone_ent = commands
            .spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&bone_rect),
                    transform: Transform {
                        translation: ab.extend(-0.1),
                        rotation: Quat::from_rotation_z(z_rot),
                        ..default()
                    },
                    ..default()
                },
                Fill::color(Color::hsl(360.0, 0.37, 0.84)),
            ))
            .id();
        commands.get_entity(joints[0]).unwrap().add_child(bone_ent);
        return bone_ent;
    }

    // Development fn for testing new bone
    pub fn new(commands: &mut Commands, joints: [Entity; 2], joint_pos: [Vec2; 2]) -> Entity {
        let [a_pos, b_pos] = joint_pos;

        // Create joint
        let ab = b_pos - a_pos;
        let dir = ab * 0.5;
        let len = ab.length();
        let x = if ab.x >= 0.0 { -1.0 } else { 1.0 };
        let z_rot = x * f32::acos(ab.y / len);
        let bone_width = 3.0;

        let bone_rect = shapes::Rectangle {
            extents: vec2(bone_width, len),
            origin: shapes::RectangleOrigin::Center,
        };

        let bone_ent = commands
            .spawn((
                RigidBody::Dynamic,
                GlobalTransform::default(),
                ComputedVisibility::default(),
                Transform::from_translation((a_pos + dir).extend(-0.1)),
            ))
            .with_children(|p| {
                p.spawn((
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&bone_rect),
                        transform: Transform::from_rotation(Quat::from_rotation_z(z_rot)),
                        ..default()
                    },
                    Collider::cuboid(bone_width * 0.5, (len - 10.0) * 0.5),
                    Fill::color(Color::hsl(360.0, 0.37, 0.84)),
                ));
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

    pub fn new_dev_broken(commands: &mut Commands, joints: [Entity; 2], joint_pos: [Vec2; 2]) {
        let dir = (joint_pos[1] - joint_pos[0]) / 2.0;
        let mid = (joint_pos[0] + joint_pos[1]) / 2.0;
        let bone_joint = JointBundle::new(mid, 0.1, 0.0, 0.5);
        let bone_centre = commands.spawn(bone_joint).id();

        let a_rev_joint = RevoluteJointBuilder::new()
            // .local_anchor1(Vec2::ZERO)
            .local_anchor1(-dir)
            .build();
        let b_rev_joint = RevoluteJointBuilder::new()
            // .local_anchor1(Vec2::ZERO)
            .local_anchor1(dir)
            .build();

        let a_axel = commands
            .spawn(ImpulseJoint::new(joints[0], a_rev_joint))
            .id();
        let b_axel = commands
            .spawn(ImpulseJoint::new(joints[1], b_rev_joint))
            .id();

        commands.get_entity(bone_centre).unwrap().add_child(a_axel);
        commands.get_entity(bone_centre).unwrap().add_child(b_axel);
    }
}
