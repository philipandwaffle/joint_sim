use bevy::prelude::{BuildChildren, Commands, Entity, Vec2};
use bevy_rapier2d::prelude::{ImpulseJoint, RevoluteJointBuilder};

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
}
