use bevy::prelude::{BuildChildren, Commands, Entity, Vec2};
use bevy_rapier2d::prelude::{ImpulseJoint, RevoluteJointBuilder};

use super::joint::JointBundle;

pub struct Bone;
impl Bone {
    // Create a new bone
    pub fn new(commands: &mut Commands, joints: [Entity; 2], joint_pos: [Vec2; 2]) {
        let [mut a_pos, mut b_pos] = joint_pos;

        // Create joint
        let joint_ab = RevoluteJointBuilder::new()
            .local_anchor1(b_pos - a_pos)
            .local_anchor2(Vec2::ZERO)
            .build();
        let impulse_joint = commands.spawn(ImpulseJoint::new(joints[0], joint_ab)).id();

        // Add impulse joint as child
        commands
            .get_entity(joints[1])
            .unwrap()
            .add_child(impulse_joint);
    }

    // Development fn for testing new bone
    pub fn new_dev(commands: &mut Commands, joints: [Entity; 2], joint_pos: [Vec2; 2]) {
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
