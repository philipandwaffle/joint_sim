use bevy::prelude::{BuildChildren, Commands, Entity, Transform, Vec2};
use bevy_rapier2d::prelude::{
    Collider, GravityScale, ImpulseJoint, RevoluteJointBuilder, RigidBody,
};

use super::joint::JointBundle;

pub struct Bone;
impl Bone {
    pub fn new(
        commands: &mut Commands,
        joints: [Entity; 2],
        joint_pos: [Vec2; 2],
        length: Option<f32>,
    ) {
        let [mut a_pos, mut b_pos] = joint_pos;

        if let Some(length) = length {
            let dir = b_pos - a_pos;
            let len_diff = dir.length() - length;
            let correction_vec = dir.normalize() * len_diff * 0.5;
            a_pos += correction_vec;
            b_pos -= correction_vec;
        }

        let joint_ab = RevoluteJointBuilder::new()
            .local_anchor1(b_pos - a_pos)
            .local_anchor2(a_pos - a_pos)
            .build();

        let impulse_joint = commands.spawn(ImpulseJoint::new(joints[0], joint_ab)).id();

        commands
            .get_entity(joints[1])
            .unwrap()
            .add_child(impulse_joint);
    }

    pub fn new1(
        commands: &mut Commands,
        joints: [Entity; 2],
        joint_pos: [Vec2; 2],
        length: Option<f32>,
    ) {
        let dir = (joint_pos[1] - joint_pos[0]) / 2.0;
        let mid = (joint_pos[0] + joint_pos[1]) / 2.0;

        // let bone_centre = commands
        //     .spawn((
        //         RigidBody::Dynamic,
        //         Collider::ball(1.0),
        //         Transform::from_translation(mid.extend(0.0)),
        //         GravityScale(5.0),
        //     ))
        //     .id();
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
